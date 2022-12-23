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

    fn step(&mut self) {
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

        for elf in 0..self.elves.len() {
            let target = match self.elves[elf].target {
                Some(t) => t,
                None => continue,
            };

            if self.considering[&target] > 1 {
                continue;
            }

            self.elves[elf].pos = target;
        }

        self.next_dir = (self.next_dir + 1) % 4;
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

    for _ in 0..10 {
        state.step();
    }

    println!("{}", state);

    std::process::ExitCode::SUCCESS
}
