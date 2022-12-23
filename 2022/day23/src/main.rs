use std::io::BufRead;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct State {
    elves: Vec<Elf>,
    map: HashSet<(i32, i32)>,
    considering: HashMap<(i32, i32), u8>,
    next_dir: u8,
}

#[derive(Debug, Clone)]
struct Elf {
    pos: (i32, i32),
    target: Option<(i32, i32)>,
}

#[derive(Debug, Clone)]
struct Bounds {
    min: (i32, i32),
    max: (i32, i32),
}

struct GapCounter {
    bounds: Bounds,
    last_x: i32,
    last_y: i32,
    gaps: usize,
}

impl GapCounter {
    fn new(bounds: Bounds) -> GapCounter {
        let last_x = bounds.min.0 - 1;
        let last_y = bounds.min.1;

        GapCounter { bounds, last_x, last_y, gaps: 0 }
    }

    fn add_pos(&mut self, pos: (i32, i32)) {
        if pos.1 != self.last_y {
            self.gaps += (self.bounds.max.0 - self.last_x) as usize;
            self.gaps += ((self.bounds.max.0 - self.bounds.min.0 + 1) * (pos.1 - self.last_y - 1)) as usize;
            self.last_x = self.bounds.min.0 - 1;
        }

        self.gaps += (pos.0 - self.last_x - 1) as usize;

        self.last_x = pos.0;
        self.last_y = pos.1;
    }

    fn end(&mut self) -> usize {
        self.add_pos((self.bounds.max.0 + 1,
                      self.bounds.max.1));
        self.gaps
    }
}

impl State {
    fn load<F: BufRead>(input: &mut F) -> Result<State, String> {
        let mut state = State {
            elves: Vec::<Elf>::new(),
            map: HashSet::<(i32, i32)>::new(),
            considering: HashMap::<(i32, i32), u8>::new(),
            next_dir: 0,
        };

        let mut pos = (0, 0);

        let buf = match input.fill_buf() {
            Err(e) => return Err(e.to_string()),
            Ok(b) => b,
        };

        for b in buf.iter() {
            match b {
                b'\n' => {
                    pos.0 = -1;
                    pos.1 += 1;
                },
                b'#' => {
                   state.elves.push(Elf { pos, target: None });
                   state.map.insert(pos);
                },
                _ => (),
            };

            pos.0 += 1;
        }

        let buf_len = buf.len();
        input.consume(buf_len);

        Ok(state)
    }

    fn elf_has_neighbor(&self, elf: &Elf) -> bool {
        for y in -1..=1 {
            for x in -1..=1 {
                if y == 0 && x == 0 {
                    continue;
                }

                if self.map.contains(&(elf.pos.0 + x, elf.pos.1 + y)) {
                    return true;
                }
            }
        }

        false
    }
    
    fn move_pos(pos: (i32, i32), dir: u8) -> (i32, i32) {
        match dir {
            0 => (pos.0, pos.1 - 1),
            1 => (pos.0, pos.1 + 1),
            2 => (pos.0 - 1, pos.1),
            3 => (pos.0 + 1, pos.1),
            _ => panic!("unknown dir {}", dir),
        }
    }

    fn can_move_dir(&self, pos: (i32, i32), dir: u8) -> bool {
        let pos = State::move_pos(pos, dir);

        if dir & 2 == 2 {
            for y in -1..=1 {
                if self.map.contains(&(pos.0, pos.1 + y)) {
                    return false;
                }
            }
        } else {
            for x in -1..=1 {
                if self.map.contains(&(pos.0 + x, pos.1)) {
                    return false;
                }
            }
        }

        true
    }

    fn get_elf_target(&self, elf: &Elf) -> Option<(i32, i32)> {
        if !self.elf_has_neighbor(elf) {
            return None;
        }

        for dir in 0..4 {
            let dir = (dir + self.next_dir) % 4;

            if self.can_move_dir(elf.pos, dir) {
                return Some(State::move_pos(elf.pos, dir));
            }
        }

        None
    }

    fn step(&mut self) -> bool {
        self.considering.clear();

        for elf in 0..self.elves.len() {
            let target = self.get_elf_target(&self.elves[elf]);
            if let Some(target) = target {
                self.considering.entry(target)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
            self.elves[elf].target = target;
        }

        let mut elf_moved = false;

        for elf in 0..self.elves.len() {
            let target = match self.elves[elf].target {
                Some(t) => t,
                None => continue,
            };

            if self.considering[&target] > 1 {
                continue;
            }

            self.map.remove(&self.elves[elf].pos);
            self.map.insert(target);
            self.elves[elf].pos = target;
            elf_moved = true;
        }

        self.next_dir = (self.next_dir + 1) % 4;

        elf_moved
    }

    fn bounds(&self) -> Bounds {
        let mut bounds = Bounds {
            min: (i32::MAX, i32::MAX),
            max: (i32::MIN, i32::MIN),
        };

        for pos in self.map.iter() {
            if pos.0 < bounds.min.0 {
                bounds.min.0 = pos.0;
            }
            if pos.0 > bounds.max.0 {
                bounds.max.0 = pos.0;
            }
            if pos.1 < bounds.min.1 {
                bounds.min.1 = pos.1;
            }
            if pos.1 > bounds.max.1 {
                bounds.max.1 = pos.1;
            }
        }

        bounds
    }

    fn count_gaps(&self) -> usize {
        let mut counter = GapCounter::new(self.bounds());
        let mut positions: Vec<(i32, i32)> =
            self.map.iter().map(|&p| p).collect();

        positions.sort_by(|a, b| {
            a.1.cmp(&b.1).then(a.0.cmp(&b.0))
        });

        for pos in positions {
            counter.add_pos(pos);
        }

        counter.end()
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let bounds = self.bounds();

        for y in bounds.min.1..=bounds.max.1 {
            for x in bounds.min.0..=bounds.max.0 {
                write!(f, "{}", if self.map.contains(&(x, y)) { '#' } else { '.' })?;
            }

            if y < bounds.max.1 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

fn main() -> std::process::ExitCode {
    let mut state = match State::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(s) => s,
    };

    for round in 1.. {
        let elf_moved = state.step();

        if round == 10 {
            println!("part 1: {}", state.count_gaps());
        }

        if !elf_moved {
            println!("part 2: {}", round);
            break;
        }
    }

    std::process::ExitCode::SUCCESS
}
