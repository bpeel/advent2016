#[derive(Copy, Clone, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn index_value(&self) -> u32 {
        match self {
            Hand::Rock => 0,
            Hand::Paper => 1,
            Hand::Scissors => 2,
        }
    }

    fn from_index(index: u32) -> Hand {
        match index {
            0 => Hand::Rock,
            1 => Hand::Paper,
            2 => Hand::Scissors,
            _ => unreachable!(),
        }
    }

    fn score(&self) -> u32 {
        self.index_value() + 1
    }
}

struct HandParseError {
}

impl std::str::FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let ch = match chars.next() {
            Some(ch) => ch,
            None => return Err(HandParseError {}),
        };

        if let Some(..) = chars.next() {
            return Err(HandParseError {});
        }

        match ch {
            'A'..='C' => Ok(Hand::from_index(ch as u32 - 'A' as u32)),
            'X'..='Z' => Ok(Hand::from_index(ch as u32 - 'X' as u32)),
            _ => Err(HandParseError {}),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Round {
    theirs: Hand,
    yours: Hand,
}

struct RoundParseError {
}

impl Round {
    fn new(theirs: Hand, yours: Hand) -> Round {
        Round { theirs, yours }
    }

    fn score_result(self) -> u32 {
        match (self.yours.index_value() + 3 -
               self.theirs.index_value()) % 3 {
            0 => 3, // draw
            1 => 6, // win
            2 => 0, // lose
            _ => unreachable!(),
        }
    }

    fn score(self) -> u32 {
        self.yours.score() + self.score_result()
    }

    fn parse_part2(line: &str) -> Result<Round, RoundParseError> {
        let mut chars = line.chars();

        let theirs = match chars.next() {
            None => return Err(RoundParseError {}),
            Some(ch) => match line[0..ch.len_utf8()].parse::<Hand>() {
                Err(..) => return Err(RoundParseError {}),
                Ok(hand) => hand,
            }
        };

        match chars.next() {
            Some(' ') => (),
            _ => return Err(RoundParseError {}),
        }

        let yours = match chars.next() {
            Some(ch @ 'X'..='Z') =>
            // X = lose = +2
            // Y = draw = +0
            // Z = win = +1
                Hand::from_index((theirs.index_value() +
                                  (ch as u32 + 3 - 'Y' as u32)) % 3),
            _ => return Err(RoundParseError {}),
        };

        if chars.next() != Option::None {
            return Err(RoundParseError {});
        }

        Ok(Round::new(theirs, yours))
    }
}

impl std::str::FromStr for Round {
    type Err = RoundParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 || s.as_bytes()[1] != ' ' as u8 {
            return Err(RoundParseError {});
        }

        let theirs = match s[0..1].parse::<Hand>() {
            Err(..) => return Err(RoundParseError {}),
            Ok(theirs) => theirs,
        };

        let yours = match s[2..].parse::<Hand>() {
            Err(..) => return Err(RoundParseError {}),
            Ok(yours) => yours,
        };

        Ok(Round::new(theirs, yours))
    }
}

fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(|result| {
        match result {
            Err(..) => {
                eprintln!("I/O error while reading rounds");
                std::process::exit(1);
            },
            Ok(line) => line.trim_end().to_string(),
        }
    }).collect();

    let part1: u32 = lines.iter().enumerate().map(|(line_num, line)| {
        match line.parse::<Round>() {
            Err(..) => {
                eprintln!("Invalid round on line {}", line_num + 1);
                std::process::exit(1);
            }
            Ok(round) => round.score(),
        }
    }).sum();

    println!("part 1: {}", part1);

    let part2: u32 = lines.iter().enumerate().map(|(line_num, line)| {
        match Round::parse_part2(line) {
            Err(..) => {
                eprintln!("Invalid round on line {}", line_num + 1);
                std::process::exit(1);
            }
            Ok(round) => round.score(),
        }
    }).sum();

    println!("part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results() {
        assert_eq!(Round::new(Hand::Rock,
                              Hand::Rock).score_result(),
                   3);
        assert_eq!(Round::new(Hand::Rock,
                              Hand::Paper).score_result(),
                   0);
        assert_eq!(Round::new(Hand::Rock,
                              Hand::Scissors).score_result(),
                   6);
        assert_eq!(Round::new(Hand::Paper,
                              Hand::Rock).score_result(),
                   6);
        assert_eq!(Round::new(Hand::Paper,
                              Hand::Paper).score_result(),
                   3);
        assert_eq!(Round::new(Hand::Paper,
                              Hand::Scissors).score_result(),
                   0);
        assert_eq!(Round::new(Hand::Scissors,
                              Hand::Rock).score_result(),
                   0);
        assert_eq!(Round::new(Hand::Scissors,
                              Hand::Paper).score_result(),
                   6);
        assert_eq!(Round::new(Hand::Scissors,
                              Hand::Scissors).score_result(),
                   3);
    }
}
