use std::rc::Rc;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io;
use std::io::BufRead;
use std::borrow::Borrow;

const N_DIRECTIONS: u8 = 4;

struct Node {
    direction: u8,
    depth: u32,
    score: u32,
    parent: Option<Rc<Node>>
}

impl Node {
    fn new(direction: u8,
           score: u32,
           parent: Option<Rc<Node>>) -> Node {
        let depth = match parent {
            Some(ref p) => p.depth + 1,
            None => 1
        };
        Node {
            direction: direction,
            score: score,
            parent: parent,
            depth: depth
        }
    }

    fn less(&self, other: &Node) -> bool {
        macro_rules! compare {
            ($field:ident) => {{
                if self.$field < other.$field {
                    return true;
                }
                if self.$field > other.$field {
                    return false;
                }
            }}
        }

        compare!(score);
        compare!(depth);
        compare!(direction);

        false
    }
}

struct NodeHeap {
    entries: Vec<Rc<Node>>
}

// Implements a priority queue. I know that Rust has a type for this
// builtin, but it was a learning exercise to implement it myself.
impl NodeHeap {
    fn new() -> NodeHeap {
        NodeHeap { entries: Vec::new() }
    }

    fn push(&mut self, node: Rc<Node>) {
        let mut pos = self.entries.len();
        self.entries.push(node);

        while pos > 0 {
            let parent = (pos - 1) / 2;

            if !self.entries[pos].less(&self.entries[parent]) {
                break;
            }

            self.entries.swap(pos, parent);

            pos = parent;
        }
    }

    fn pop(&mut self) -> Rc<Node> {
        if self.entries.len() <= 1 {
            return self.entries.pop().unwrap();
        }

        // The first entry is the at the root of the heap and we want
        // to return it. However we also want to temporarily move the
        // last entry to be the new root. To do this will swap the
        // entries and then remove the old root with a pop.

        let last = self.entries.len() - 1;
        self.entries.swap(0, last);
        let ret = self.entries.pop().unwrap();

        let mut pos = 0;

        loop {
            let child = 2 * pos + 1;

            if child >= self.entries.len() {
                break;
            }

            // Find the smallest of the three nodes
            let mut smallest = pos;

            if self.entries[child].less(&self.entries[smallest]) {
                smallest = child;
            }

            if child + 1 < self.entries.len() &&
                self.entries[child + 1].less(&self.entries[smallest]) {
                smallest = child + 1;
            }

            // If the root is already the smallest then they are
            // already in the correct order
            if smallest == pos {
                break;
            }

            self.entries.swap(pos, smallest);

            pos = smallest;
        }

        ret
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: i8,
    y: i8
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct State {
    zero_pos: Pos,
    goal_pos: Pos,
}

struct Puzzle {
    width: i8,
    height: i8,
    start_state: State,
    board: Vec<u8>
}

impl Puzzle {
    fn is_valid_position(&self, pos: &Pos) -> bool {
        if pos.x < 0 || pos.x >= self.width ||
            pos.y < 0 || pos.y >= self.height {
            false
        } else {
            self.board[pos.x as usize + pos.y as usize *
                       self.width as usize] as char != '#'
        }
    }
}

impl State {
    fn score(&self, puzzle: &Puzzle) -> u32 {
        let goal_x = self.goal_pos.x as i32;
        let goal_y = self.goal_pos.y as i32;
        let zero_x = self.zero_pos.x as i32;
        let zero_y = self.zero_pos.y as i32;
        let puzzle_width = puzzle.width as i32;
        let puzzle_height = puzzle.height as i32;

        ((goal_x + goal_y) * (puzzle_width + puzzle_height) +
         (goal_x - zero_x).abs() + (goal_y - zero_y).abs())
            as u32
    }

    fn apply_move(&mut self, direction: u8) {
        let old_pos = self.zero_pos;
        let diff = if (direction & 1) == 0 { -1 } else { 1 };

        if (direction & 2) == 0 {
            self.zero_pos.y += diff;
        } else {
            self.zero_pos.x += diff;
        }

        if self.zero_pos == self.goal_pos {
            self.goal_pos = old_pos;
        }
    }
}

struct History {
    entries: HashMap<State, u32>
}

impl History {
    fn new() -> History {
        History { entries: HashMap::new() }
    }

    fn add(&mut self, state: &State, depth: u32) -> bool {
        // Add the entry to the hash map if it’s not already there or
        // the existing entry has a higher depth
        match self.entries.entry(*state) {
            Entry::Vacant(v) => {
                v.insert(depth);
                true
            },
            Entry::Occupied(mut o) => {
                let mut entry_depth = o.get_mut();

                if *entry_depth > depth {
                    *entry_depth = depth;
                    true
                } else {
                    false
                }
            }
        }
    }
}

struct SearchData {
    history: History,
    node_heap: NodeHeap,

    // This is just used as a small temporary buffer to be able to get
    // the directions from the nodes and then process them in reverse
    // order. The C version was using a vla on the stack for this but
    // there doesn’t seem to be an equivalent in Rust.
    direction_list: Vec<u8>
}

fn expand_position(search_data: &mut SearchData,
                   parent: Option<Rc<Node>>,
                   puzzle: &Puzzle,
                   start_state: &State) {
    for direction in 0..N_DIRECTIONS {
        let mut move_state = *start_state;
        move_state.apply_move(direction);

        if !puzzle.is_valid_position(&move_state.zero_pos) {
            continue;
        }

        let depth = match parent {
            Some(ref node) => node.depth + 1,
            None => 1
        };

        if !search_data.history.add(&move_state, depth) {
            continue;
        }

        let score = move_state.score(&puzzle);
        let node = Rc::new(Node::new(direction, score, parent.clone()));
        search_data.node_heap.push(node);
    }
}

fn get_node_state(search_data: &mut SearchData,
                  puzzle: &Puzzle,
                  last_node: Rc<Node>,
                  state: &mut State)
{
    search_data.direction_list.clear();

    let mut node = last_node;

    loop {
        search_data.direction_list.push(node.direction);

        node = match node.parent {
            Some(ref n) => n.clone(),
            None => break
        }
    }

    *state = puzzle.start_state;

    for direction in search_data.direction_list.iter().rev() {
        state.apply_move(*direction);
    }
}

fn expand_initial_nodes(search_data: &mut SearchData,
                        puzzle: &Puzzle) {
    expand_position(search_data,
                    None, // parent
                    puzzle,
                    &puzzle.start_state);
}

fn skip_string(p: &mut std::str::Chars, string: &str) -> Result<(), io::Error> {
    for ch in string.chars() {
        match p.next() {
            Some(c) if c == ch => (),
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData,
                                           "bad line"))

        }
    }

    Ok(())
}

fn parse_number(p: &mut std::str::Chars) -> Result<i32, io::Error> {
    let rest = p.as_str();
    // Find the byte index of the first non-digit, or if not the
    // length of the string in bytes
    let length = match rest.char_indices().find(|&(_, ch)|
                                                ch < '0' ||
                                                ch > '9') {
        Some((pos, _)) => pos,
        None => rest.len()
    };
    // Skip the iterator ahead to the end of the number
    p.clone_from(&rest[length..].chars());
    // Parse the number using the substring
    match rest[0..length].parse() {
        Ok(n) => Ok(n),
        Err(_) =>
            Err(io::Error::new(io::ErrorKind::InvalidData, "bad line"))
    }
}

fn skip_spaces(p: &mut std::str::Chars) {
    let mut end = p.clone();

    while let Some(n) = end.next() {
        if n != ' ' {
            break;
        }
        p.next();
    }
}

fn read_board() -> Result<Puzzle, io::Error> {
    struct Device {
        pos: Pos,
        used: i32
    };
    let mut devices = Vec::<Device>::new();

    let mut puzzle = Puzzle {
        width: 0,
        height: 0,
        start_state: State {
            goal_pos: Pos { x: 0, y: 0 },
            zero_pos: Pos { x: 0, y: 0 },
        },
        board: Vec::new()
    };

    let mut min_size = i32::max_value();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = try!(line);

        if line.starts_with("root@") || line.starts_with("Filesystem") {
            continue;
        }

        let mut p = line.chars();

        try!(skip_string(&mut p, "/dev/grid/node-x"));
        let x = try!(parse_number(&mut p)) as i8;
        try!(skip_string(&mut p, "-y"));
        let y = try!(parse_number(&mut p)) as i8;
        skip_spaces(&mut p);
        let size = try!(parse_number(&mut p));
        try!(skip_string(&mut p, "T"));
        skip_spaces(&mut p);
        let used = try!(parse_number(&mut p));

        if x > puzzle.width {
            puzzle.width = x;
        }
        if y > puzzle.height {
            puzzle.height = y;
        }
        if size < min_size {
            min_size = size;
        }

        devices.push(Device {
            pos: Pos { x: x, y: y },
            used: used
        });
    }

    puzzle.width += 1;
    puzzle.height += 1;
    puzzle.board.resize(puzzle.width as usize *
                        puzzle.height as usize,
                        ' ' as u8);

    for d in devices {
        puzzle.board[d.pos.x as usize + d.pos.y as usize *
                     puzzle.width as usize] =
            if d.used <= min_size {
                ' ' as u8
            } else {
                '#' as u8
            };

        if d.used == 0 {
            puzzle.start_state.zero_pos = d.pos;
        }
    }

    puzzle.start_state.goal_pos.x = puzzle.width - 1;
    puzzle.start_state.goal_pos.y = 0;

    Ok(puzzle)
}

#[allow(unused_variables)]
fn print_solution(puzzle: &Puzzle,
                  node: &Node)
{
    print!("{}\n", node.depth);
}

fn solve(puzzle: &Puzzle) {
    let mut search_data = SearchData {
        history: History::new(),
        node_heap: NodeHeap::new(),
        direction_list: Vec::new()
    };
    let mut best_score = u32::max_value();

    expand_initial_nodes(&mut search_data, puzzle);

    while search_data.node_heap.entries.len() > 0 {
        let node = search_data.node_heap.pop();

        if node.depth > best_score {
            continue;
        }

        let mut state = puzzle.start_state;
        get_node_state(&mut search_data, puzzle, node.clone(), &mut state);

        if state.goal_pos.x == 0 && state.goal_pos.y == 0 {
            best_score = node.depth;
            print_solution(puzzle, node.borrow());
        } else {
            expand_position(&mut search_data, Some(node), puzzle, &state);
        }
    }
}

fn main() {
    match read_board() {
        Err(e) => {
            print!("{}\n", e.to_string());
            return;
        },
        Ok(n) => solve(&n)
    }
}
