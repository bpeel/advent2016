use std::process::ExitCode;
use std::collections::HashMap;

const N_DIE_SIDES: u32 = 100;
const N_POSITIONS: u32 = 10;
const WINNING_SCORE_PART1: u32 = 1000;
const WINNING_SCORE_PART2: u32 = 21;
const DIE_ROLLS_PER_TURN: u32 = 3;
const N_PLAYERS: usize = 2;

#[derive(Clone, Copy, Debug)]
struct Player {
    position: u32,
    score: u32,
}

#[derive(Clone, Debug)]
struct Game<D: Die> {
    next_player: usize,
    players: [Player; N_PLAYERS],
    winning_score: u32,
    die: D,
}

#[derive(Clone, Debug)]
struct FinalScores<D: Die> {
    losing_score: u32,
    winning_player: usize,
    die: D,
}

trait Die {
    fn roll_die(&mut self) -> u32;
}

impl<D: Die> Game<D> {
    fn new(
        starting_positions: &[u32; N_PLAYERS as usize],
        winning_score: u32,
        die: D,
    ) -> Game<D> {
        let mut game = Game {
            next_player: 0,
            players: [Player { position: 0, score: 0 }; N_PLAYERS],
            winning_score,
            die,
        };

        for (i, &pos) in starting_positions.iter().enumerate() {
            game.players[i].position = pos;
        }

        game
    }

    fn roll_die(&mut self) -> u32 {
        self.die.roll_die()
    }

    fn take_turn(&mut self) -> bool {
        let dice_score: u32 =
            (0..DIE_ROLLS_PER_TURN).map(|_| self.roll_die()).sum();

        let player = &mut self.players[self.next_player];

        player.position = (player.position + dice_score) % N_POSITIONS;
        player.score += player.position + 1;

        if player.score >= self.winning_score {
            true
        } else {
            self.next_player = (self.next_player + 1) % N_PLAYERS;

            false
        }
    }

    fn finalise(self) -> FinalScores<D> {
        FinalScores {
            losing_score: self.players.iter().enumerate().map(|(i, p)| {
                if i == self.next_player {
                    0
                } else {
                    p.score
                }
            }).sum(),
            die: self.die,
            winning_player: self.next_player,
        }
    }
}

struct DeterministicDie {
    n_die_rolls: u32,
}

impl DeterministicDie {
    fn new() -> DeterministicDie {
        DeterministicDie {
            n_die_rolls: 0,
        }
    }
}

impl Die for DeterministicDie {
    fn roll_die(&mut self) -> u32 {
        let result = self.n_die_rolls % N_DIE_SIDES + 1;

        self.n_die_rolls += 1;

        result
    }
}

fn part1(starting_positions: &[u32; N_PLAYERS]) -> u32 {
    let die = DeterministicDie::new();

    let mut game = Game::new(starting_positions, WINNING_SCORE_PART1, die);

    loop {
        if game.take_turn() {
            let scores = game.finalise();

            println!(
                "losing_score = {}, n_die_rolls = {}",
                scores.losing_score,
                scores.die.n_die_rolls,
            );


            break scores.losing_score * scores.die.n_die_rolls;
        }
    }
}

// Stats about a set of DIE_ROLLS_PER_TURN dice rolls
struct QuantumDieScore {
    // Any example set of die rolls that gives the score
    rolls: [u32; DIE_ROLLS_PER_TURN as usize],
    // The number of combinations that can achieve that score
    combinations: u32,
}

struct QuantumDie {
    possible_scores: Vec<QuantumDieScore>,
    score_sequence: Vec<u8>,
    n_rolls: usize,
}

impl QuantumDie {
    fn new() -> QuantumDie {
        let mut possible_scores = HashMap::<u32, QuantumDieScore>::new();
        let mut dice = [1u32; DIE_ROLLS_PER_TURN as usize];

        'score_loop: loop {
            let score = dice.iter().map(|&a| a).sum();

            possible_scores.entry(score)
                .and_modify(|score| score.combinations += 1)
                .or_insert_with(|| QuantumDieScore {
                    rolls: dice,
                    combinations: 1,
                });

            for die in dice.iter_mut() {
                if *die >= 3 {
                    *die = 1;
                } else {
                    *die += 1;
                    continue 'score_loop;
                }
            }

            break;
        }

        QuantumDie {
            possible_scores: possible_scores.into_values().collect(),
            score_sequence: Vec::new(),
            n_rolls: 0,
        }
    }

    fn count_equivalent_universes(&self) -> u64 {
        let n_scores = self.n_rolls / DIE_ROLLS_PER_TURN as usize;

        // Count the number of universes that would have the same result
        let mut n_universes = 1;

        for &score in self.score_sequence[0..n_scores].iter() {
            let score = &self.possible_scores[score as usize];
            n_universes *= score.combinations as u64;
        }

        n_universes
    }

    fn next_universe(&mut self) -> bool {
        let n_scores = self.n_rolls / DIE_ROLLS_PER_TURN as usize;

        self.score_sequence.truncate(n_scores);
        self.n_rolls = 0;

        for score in self.score_sequence.iter_mut().rev() {
            if *score as usize + 1 >= self.possible_scores.len() {
                *score = 0;
            } else {
                *score += 1;
                return true;
            }
        }

        false
    }
}

impl Die for QuantumDie {
    fn roll_die(&mut self) -> u32 {
        let n_scores = self.n_rolls / DIE_ROLLS_PER_TURN as usize;

        if n_scores >= self.score_sequence.len() {
            self.score_sequence.push(0);
        }

        let roll_num = self.n_rolls % DIE_ROLLS_PER_TURN as usize;

        let score_index = self.score_sequence[n_scores] as usize;
        let score = &self.possible_scores[score_index];
        let result = score.rolls[roll_num];

        self.n_rolls += 1;

        result
    }
}

fn part2(starting_positions: &[u32; N_PLAYERS]) -> u64 {
    let mut die = QuantumDie::new();
    let mut wins = [0u64; N_PLAYERS];

    loop {
        let mut game = Game::new(starting_positions, WINNING_SCORE_PART2, die);

        loop {
            if game.take_turn() {
                let scores = game.finalise();

                die = scores.die;
                wins[scores.winning_player] += die.count_equivalent_universes();

                break;
            }
        }

        if !die.next_universe() {
            break wins.iter().map(|&a| a).max().unwrap()
        }
    }
}

fn main() -> ExitCode {
    let mut args = std::env::args();

    if args.len() != N_PLAYERS + 1 {
        eprintln!("usage: day21 <player1 pos> <player2 pos>");
        return ExitCode::FAILURE;
    }

    args.next().unwrap();

    let mut starting_positions = [0u32; N_PLAYERS];

    for i in 0..N_PLAYERS {
        let arg = args.next().unwrap();

        starting_positions[i] = match arg.parse::<u32>() {
            Ok(position) if position > 0 => position - 1,
            _ => {
                eprintln!("Invalid staring position: {}", arg);
                return ExitCode::FAILURE;
            },
        };
    }

    println!("part 1: {}", part1(&starting_positions));
    println!("part 2: {}", part2(&starting_positions));

    ExitCode::SUCCESS
}
