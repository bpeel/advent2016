type Crate = char;
type Stack = Vec<Crate>;

#[derive(Debug, Clone)]
struct State {
    stacks: Vec<Stack>,
}

impl State {
    fn new() -> State {
        State { stacks: Vec::new() }
    }

    fn apply_move(&mut self, mov: &Move) -> Result<(), String> {
        if mov.source < 1 ||
            mov.source as usize > self.stacks.len() ||
            mov.dest < 1 ||
            mov.dest as usize > self.stacks.len() {
            return Err("Invalid source or destination in move".to_string());
        }

        for _ in 0..mov.amount {
            let c = match self.stacks[mov.source as usize - 1].pop() {
                Some(c) => c,
                None => return Err("Tried to move from an empty \
                                    crate".to_string()),
            };

            self.stacks[mov.dest as usize - 1].push(c);
        }

        Ok(())
    }

    fn tops(&self) -> String {
        self.stacks.iter().map(|stack| {
            match stack.last() {
                Some(&n) => n,
                None => '?',
            }
        }).collect()
    }
}

#[derive(Debug, Clone)]
struct Move {
    amount: u32,
    source: u32,
    dest: u32,
}

fn read_state<I>(lines: &mut I) -> Result<State, std::io::Error>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let mut state = State::new();

    while let Some(result) = lines.next() {
        let line = result?;

        if line.len() == 0 {
            break;
        }

        let n_stacks = (line.len() + 1) / 4;

        if n_stacks > state.stacks.len() {
            state.stacks.extend(std::iter::repeat(Vec::<Crate>::new())
                                .take(n_stacks - state.stacks.len()));
        }

        for stack_num in 0..n_stacks {
            let byte = line.as_bytes()[stack_num * 4 + 1];

            if byte >= 128 {
                continue;
            }

            let ch = byte as char;

            if ch.is_alphabetic() {
                state.stacks[stack_num].push(ch);
            }
        }
    }

    for stack in state.stacks.iter_mut() {
        stack.reverse();
    }

    Ok(state)
}

fn skip_string<I>(iter: &mut I, s: &str) -> Result<(), String>
    where I: Iterator<Item = char>
{
    for s_ch in s.chars() {
        match iter.next() {
            Some(ch) if ch == s_ch => (),
            _ => return Err(format!("expected “{}”", s)),
        }
    }

    Ok(())
}

fn read_number<I>(iter: &mut I) -> Result<u32, String>
    where I: Iterator<Item = char>
{
    let mut had_digit = false;
    let mut num = 0;

    while let Some(ch) = iter.next() {
        if ch < '0' || ch > '9' {
            break;
        }

        num = num * 10 + (ch as u32 - '0' as u32) as u32;

        had_digit = true;
    }

    match had_digit {
        true => Ok(num),
        false => Err("Expected number".to_string()),
    }
}

fn read_moves<I>(lines: &mut I) -> Result<Vec<Move>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let mut moves = Vec::<Move>::new();

    for result in lines {
        let line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        let mut iter = line.chars();

        skip_string(&mut iter, "move ")?;
        let amount = read_number(&mut iter)?;

        skip_string(&mut iter, "from ")?;
        let source = read_number(&mut iter)?;

        skip_string(&mut iter, "to ")?;
        let dest = read_number(&mut iter)?;

        moves.push(Move { amount, source, dest });
    }

    Ok(moves)
}

fn read_data<I>(lines: &mut I) -> Result<(State, Vec<Move>), String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let state = match read_state(lines) {
        Ok(state) => state,
        Err(e) => return Err(e.to_string()),
    };

    let moves = read_moves(lines)?;

    Ok((state, moves))
}

fn main() {
    let mut lines = std::io::stdin().lines();

    let (mut state, moves) = match read_data(&mut lines) {
        Ok(pair) => pair,

        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
    };

    for (move_num, mov) in moves.iter().enumerate() {
        if let Err(e) = state.apply_move(mov) {
            eprintln!("move {}: {}", move_num + 1, e);
            std::process::exit(1);
        }
    }

    println!("part 1: {}", state.tops());
}
