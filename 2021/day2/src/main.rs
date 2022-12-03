struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    fn new() -> Position {
        Position { horizontal: 0, depth: 0 }
    }

    fn run_command(&mut self, command: &str) -> Result<(), String> {
        let space = match command.find(' ') {
            Some(i) => i,
            None => return Err("No space in command".to_string()),
        };

        let amount = match command[space + 1..].trim_end().parse::<i32>() {
            Ok(val) => val,
            Err(..) => return Err("Invalid integer in command".to_string()),
        };

        match &command[0..space] {
            "forward" => self.horizontal += amount,
            "down" => self.depth += amount,
            "up" => self.depth -= amount,
            keyword => return Err("Unknown command: ".to_string() + keyword),
        }

        Ok(())
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

        if let Err(message) = pos.run_command(&line) {
            eprintln!("line {}: {}", line_num + 1, message);
            std::process::exit(1);
        }
    }

    println!("Horizontal position = {}, Depth = {}", pos.horizontal, pos.depth);
    println!("Result = {}", pos.horizontal * pos.depth);
}
