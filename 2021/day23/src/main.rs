use std::str::FromStr;
use std::process::ExitCode;
use std::cmp::{min, max};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

const N_AMPHIPOD_TYPES: usize = 4;
const N_AMPHIPODS_PER_TYPE: usize = 2;
const TOTAL_N_AMPHIPODS: usize = N_AMPHIPOD_TYPES * N_AMPHIPODS_PER_TYPE;
const N_SIDE_ROOMS: usize = 1;

const MOVES_PER_AMPHIPOD: usize =
// Move into a room
    N_AMPHIPOD_TYPES
// Move beside a room
    + N_AMPHIPOD_TYPES
// Move to left side room
    + N_SIDE_ROOMS + 1
// Move to a right side room
    + N_SIDE_ROOMS;

// The total number of moves that we can consider from a state
const N_MOVES: usize = MOVES_PER_AMPHIPOD * TOTAL_N_AMPHIPODS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Position {
    InRoom {
        room_num: u8,
        room_pos: u8,
    },
    // Waiting to the right of the entry to a room
    OutsideRoom(u8),
    // There will be N_SIDE_ROOMS+1 side rooms to the left to take
    // into account waiting to the left of the leftmost room. These
    // are numbered starting from 0 and increasing numbers go further
    // left.
    LeftSideRoom(u8),
    RightSideRoom(u8),
}

impl Position {
    fn from_move_num(move_num: usize) -> Position {
        // Move into a room
        if move_num < N_AMPHIPOD_TYPES {
            return Position::InRoom { room_num: move_num as u8, room_pos: 0 };
        }
        let move_num = move_num - N_AMPHIPOD_TYPES;

        // Move beside a room
        if move_num < N_AMPHIPOD_TYPES {
            return Position::OutsideRoom(move_num as u8);
        }
        let move_num = move_num - N_AMPHIPOD_TYPES;

        // Move to a left-hand side room
        if move_num <= N_SIDE_ROOMS {
            return Position::LeftSideRoom(move_num as u8);
        }
        let move_num = move_num - N_SIDE_ROOMS - 1;

        // Move to a right-hand side room
        if move_num < N_SIDE_ROOMS {
            return Position::RightSideRoom(move_num as u8);
        }

        unreachable!("Invalid move_num");
    }

    fn x(&self) -> u32 {
        match self {
            &Position::InRoom { room_num, .. } => {
                room_num as u32 * 2 + N_SIDE_ROOMS as u32 + 1
            },
            &Position::OutsideRoom(room_num) => {
                room_num as u32 * 2 + N_SIDE_ROOMS as u32 + 2
            },
            &Position::LeftSideRoom(room_num) => {
                N_SIDE_ROOMS as u32 - room_num as u32
            },
            &Position::RightSideRoom(room_num) => {
                N_SIDE_ROOMS as u32
                    + 1
                    + N_AMPHIPOD_TYPES as u32 * 2
                    + room_num as u32
            },
        }
    }

    fn y(&self) -> u32 {
        match self {
            &Position::InRoom { room_pos, .. } => room_pos as u32 + 1,
            Position::OutsideRoom(_) |
            Position::LeftSideRoom(_) |
            Position::RightSideRoom(_) =>
                0,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    amphipods: [Position; TOTAL_N_AMPHIPODS],
}

impl State {
    fn n_amphipods_in_room(&self, room_num: u8) -> usize {
        let mut result = 0;

        for amphipod in self.amphipods.iter() {
            if let &Position::InRoom { room_num: other_room, .. } = amphipod {
                if other_room == room_num {
                    result += 1;
                }
            }
        }

        result
    }

    fn try_move(
        &self,
        amphipod_num: usize,
        pos: &Position,
    ) -> Option<(Position, u64)> {
        let amphipod_type = amphipod_num / N_AMPHIPODS_PER_TYPE;
        let current_pos = &self.amphipods[amphipod_num];

        let pos = match pos {
            &Position::InRoom { room_num, .. } => {
                if matches!(current_pos, Position::InRoom { .. }) {
                    return None;
                }

                if room_num as usize != amphipod_type {
                    return None;
                }

                let Some(room_pos) = (N_AMPHIPODS_PER_TYPE - 1)
                    .checked_sub(self.n_amphipods_in_room(room_num))
                else { return None; };

                Position::InRoom { room_num, room_pos: room_pos as u8 }
            },
            _ => {
                let &Position::InRoom { room_pos, room_num } = current_pos
                else { return None; };

                if room_pos as usize
                    != N_AMPHIPODS_PER_TYPE
                    - self.n_amphipods_in_room(room_num)
                {
                    return None;
                }

                pos.clone()
            },
        };

        let target_x = pos.x();
        let current_x = current_pos.x();
        let blocking_range =
            min(target_x, current_x)..=max(target_x, current_x);

        if self.amphipods
            .iter()
            .enumerate()
            .find(|&(num, amphipod)| {
                num != amphipod_num
                    && amphipod.y() == 0
                    && blocking_range.contains(&amphipod.x())
            })
            .is_some()
        {
            return None
        }

        let mut move_cost = 1;

        for _ in 0..amphipod_type {
            move_cost *= 10;
        }

        Some((
            pos,
            move_cost
                * (self.amphipods[amphipod_num].x().abs_diff(pos.x())
                   + self.amphipods[amphipod_num].y().abs_diff(pos.y())) as u64
        ))
    }

    fn is_solved(&self) -> bool {
        for (num, amphipod) in self.amphipods.iter().enumerate() {
            let &Position::InRoom { room_num, .. } = amphipod
            else { return false; };

            if num / N_AMPHIPODS_PER_TYPE != room_num as usize {
                return false;
            }
        }

        true
    }
}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<State, String> {
        let mut line_pos = 0usize;
        let mut amphipod_counts = [0; N_AMPHIPOD_TYPES];

        let mut state = State {
            amphipods: [Position::LeftSideRoom(0); TOTAL_N_AMPHIPODS],
        };

        for ch in s.chars() {
            match ch {
                '\n' => {
                    line_pos = 0;
                    continue;
                },
                '#' | ' ' | '.' => (),
                'A'.. if (ch as usize) < 'A' as usize + N_AMPHIPOD_TYPES => {
                    let amphipod_type = ch as usize - 'A' as usize;

                    if amphipod_counts[amphipod_type] >= N_AMPHIPODS_PER_TYPE {
                        return Err(format!(
                            "Too many amphipods of type {}",
                            ch
                        ));
                    }

                    let room_num = match line_pos.checked_sub(
                        N_SIDE_ROOMS + 2
                    ) {
                        Some(room_column) => if room_column & 1 != 0 {
                            return Err("Amphipod inbetween rooms".to_string());
                        } else {
                            room_column / 2
                        },
                        None => {
                            return Err("Amphipod is left of rooms".to_string());
                        },
                    };

                    if room_num >= N_AMPHIPOD_TYPES {
                        return Err(format!(
                            "Amphipod in invalid room {}",
                            room_num,
                        ));
                    }

                    let room_pos = state.n_amphipods_in_room(room_num as u8);

                    if room_pos >= N_AMPHIPODS_PER_TYPE {
                        return Err(format!(
                            "Too many amphipods in room {}",
                            room_num,
                        ));
                    }

                    let amphipod_num =
                        amphipod_type
                        * N_AMPHIPODS_PER_TYPE
                        + amphipod_counts[amphipod_type];

                    state.amphipods[amphipod_num] = Position::InRoom {
                        room_num: room_num as u8,
                        room_pos: room_pos as u8,
                    };

                    amphipod_counts[amphipod_type] += 1;
                }
                _ => {
                    return Err(format!("Unexpected character: {}", ch));
                },
            }

            line_pos += 1;
        }

        Ok(state)
    }
}

struct StackEntry {
    next_move: usize,
    state: State,
    cost: u64,
}

fn read_state() -> Result<State, String> {
    let mut lines = Vec::new();

    for line in std::io::stdin().lines() {
        match line {
            Err(e) => return Err(e.to_string()),
            Ok(line) => lines.push(line),
        }
    }

    lines.join("\n").parse::<State>()
}

fn solve(original_state: &State) -> u64 {
    let mut best_solution = u64::MAX;
    let mut visited_states = HashMap::<State, u64>::new();

    let mut stack = vec![StackEntry {
        next_move: 0,
        state: original_state.clone(),
        cost: 0,
    }];

    visited_states.insert(stack[0].state.clone(), 0);

    while let Some(entry) = stack.pop() {
        for move_num in entry.next_move..N_MOVES {
            let amphipod_num = move_num / MOVES_PER_AMPHIPOD;
            let pos = Position::from_move_num(move_num % MOVES_PER_AMPHIPOD);

            if let Some((pos, cost)) = entry.state.try_move(amphipod_num, &pos)
            {
                let mut state = entry.state.clone();

                state.amphipods[amphipod_num] = pos;

                let next_cost = entry.cost + cost;

                if next_cost >= best_solution {
                    continue;
                }

                match visited_states.entry(state.clone()) {
                    Entry::Occupied(mut e) => {
                        if *e.get() <= next_cost {
                            continue;
                        }

                        e.insert(next_cost);
                    },

                    Entry::Vacant(e) => {
                        e.insert(next_cost);
                    },
                }

                if state.is_solved() {
                    best_solution = next_cost;
                    println!("{}", next_cost);
                    continue;
                }

                stack.push(StackEntry {
                    next_move: move_num + 1,
                    state: entry.state,
                    cost: entry.cost
                });

                stack.push(StackEntry {
                    next_move: 0,
                    state,
                    cost: next_cost,
                });

                break;
            }
        }
    }

    best_solution
}

fn main() -> ExitCode {
    let state = match read_state() {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(s) => s,
    };

    println!("{}", solve(&state));

    ExitCode::SUCCESS
}
