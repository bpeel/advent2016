struct Position {
    horizontal: i32,
    depth: i32,
}

enum CommandKeyword {
    Forward,
    Down,
    Up,
}

struct Command {
    keyword: CommandKeyword,
    amount: i32,
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let space = match s.find(' ') {
            Some(i) => i,
            None => return Err("No space in command".to_string()),
        };

        let amount = match s[space + 1..].trim_end().parse::<i32>() {
            Ok(val) => val,
            Err(..) => return Err("Invalid integer in command".to_string()),
        };

        let keyword = match &s[0..space] {
            "forward" => CommandKeyword::Forward,
            "down" => CommandKeyword::Down,
            "up" => CommandKeyword::Up,
            text => return Err("Unknown command: ".to_string() + text),
        };

        Ok(Command { keyword, amount })
    }
}

impl Position {
    fn new() -> Position {
        Position { horizontal: 0, depth: 0 }
    }

    fn run_command(&mut self, command: &Command) {
        match command.keyword {
            CommandKeyword::Forward => self.horizontal += command.amount,
            CommandKeyword::Down => self.depth += command.amount,
            CommandKeyword::Up => self.depth -= command.amount,
        }
    }
}

fn main() {
    let mut pos = Position::new();

    for (line_num, result) in std::io::stdin().lines().enumerate() {
        let line = match result {
            Err(..) => {
                eprintln!("I/O error while reading commands");
                std::process::exit(1);
            },
            Ok(line) => line,
        };

        let command = match line.parse::<Command>() {
            Err(message) => {
                eprintln!("line {}: {}", line_num + 1, message);
                std::process::exit(1);
            },
            Ok(command) => command,
        };

        pos.run_command(&command);
    }

    println!("Horizontal position = {}, Depth = {}", pos.horizontal, pos.depth);
    println!("Result = {}", pos.horizontal * pos.depth);
}
