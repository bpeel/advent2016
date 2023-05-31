use std::process::ExitCode;

const N_DIE_SIDES: u32 = 100;
const N_POSITIONS: u32 = 10;
const WINNING_SCORE_PART1: u32 = 1000;
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
    die: D,
}

trait Die {
    fn roll_die(&mut self) -> u32;
}

impl<D: Die> Game<D> {
    fn new(
        starting_positions: [u32; N_PLAYERS as usize],
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

            println!(
                "part 1: {}",
                scores.losing_score * scores.die.n_die_rolls
            );

            break;
        }
    }

    ExitCode::SUCCESS
}
