use std::sync::LazyLock;
use std::process::ExitCode;
use std::str::FromStr;

static REVEAL_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(\d+) (red|green|blue)").unwrap()
});

static GAME_LINE_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"Game (\d+): (.*)").unwrap()
});

#[derive(Debug, Clone)]
struct Reveal {
    red: u16,
    green: u16,
    blue: u16,
}

#[derive(Debug, Clone)]
struct Game {
    reveals: Vec<Reveal>,
}

impl FromStr for Reveal {
    type Err = String;

    fn from_str(s: &str) -> Result<Reveal, String> {
        let mut reveal = Reveal {
            red: 0,
            green: 0,
            blue: 0,
        };

        for color_str in s.split(", ") {
            let Some(captures) = REVEAL_RE.captures(color_str)
            else {
                return Err(format!("invalid color expression: {}", color_str));
            };

            let Ok(count) = captures[1].parse::<u16>()
            else {
                return Err(format!("invalid count: {}", &captures[1]));
            };

            let field = match captures[2].chars().next() {
                Some('r') => &mut reveal.red,
                Some('g') => &mut reveal.green,
                Some('b') => &mut reveal.blue,
                _ => unreachable!("The regex has gone wrong"),
            };

            let Some(new_count) = field.checked_add(count)
            else {
                return Err(format!("too many balls"));
            };

            *field = new_count;
        }

        Ok(reveal)
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Game, String> {
        let mut reveals = Vec::new();

        for reveal_str in s.split("; ") {
            reveals.push(reveal_str.parse::<Reveal>()?);
        }

        Ok(Game {
            reveals
        })
    }
}

fn read_games<I>(lines: I) -> Result<Vec<Game>, String>
    where I: IntoIterator<Item = Result<String, std::io::Error>>
{
    let mut games = Vec::new();

    for (line_num, result) in lines.into_iter().enumerate() {
        let line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        let captures = match GAME_LINE_RE.captures(&line) {
            Some(c) => c,
            None => {
                return Err(format!(
                    "line {}: invalid syntax",
                    line_num + 1)
                );
            },
        };

        if captures[1].parse::<usize>().ok().and_then(|game_id| {
            (game_id == line_num + 1).then_some(true)
        }).is_none() {
            return Err(format!(
                "line {}: invalid game ID",
                line_num + 1,
            ));
        }

        let game = match captures[2].parse::<Game>() {
            Ok(game) => game,
            Err(e) => return Err(format!(
                "line {}: {}",
                e,
                line_num + 1,
            )),
        };

        games.push(game);
    }

    Ok(games)
}

fn part1(games: &[Game]) -> u32 {
    games.iter().enumerate().filter_map(|(game_id, game)| {
        game.reveals.iter().all(|reveal| {
            reveal.red <= 12 && reveal.green <= 13 && reveal.blue <= 14
        }).then(|| game_id as u32 + 1)
    }).sum::<u32>()
}

fn main() -> ExitCode {
    let games = match read_games(std::io::stdin().lines()) {
        Ok(games) => games,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    println!("Part 1: {}", part1(&games));

    ExitCode::SUCCESS
}
