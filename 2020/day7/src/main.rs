use std::process::ExitCode;
use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;
use std::borrow::Borrow;

enum RuleError {
    BadNumber,
    MissingContains,
    MissingNameTerminator,
    MissingFullStop,
}

impl fmt::Display for RuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RuleError::BadNumber => "bad number",
                RuleError::MissingContains => "missing “contains” keyword",
                RuleError::MissingNameTerminator => "missing “bag” or “bags”",
                RuleError::MissingFullStop => "missing full stop at end",
            },
        )
    }
}

struct Bag {
    name: BagName,
    contains: Vec<BagSpace>,
}

struct BagSpace {
    amount: u32,
    bag: usize,
}

struct BagSet {
    names: HashMap<BagName, usize>,
    bags: Vec<Bag>,
}

#[derive(Eq, Hash, PartialEq)]
struct BagName(Rc<String>);

impl Borrow<str> for BagName {
    fn borrow(&self) -> &str {
        self.0.as_ref()
    }
}

impl Clone for BagName {
    fn clone(&self) -> BagName {
        BagName(Rc::clone(&self.0))
    }
}

impl fmt::Display for BagName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl BagSet {
    fn new() -> BagSet {
        BagSet {
            names: HashMap::new(),
            bags: Vec::new(),
        }
    }

    fn get_bag(&mut self, name: &str) -> usize {
        if let Some(bag_num) = self.names.get(name) {
            return *bag_num;
        }

        let name = BagName(Rc::new(name.to_string()));
        let bag_num = self.bags.len();

        self.names.insert(name.clone(), bag_num);

        self.bags.push(Bag {
            name: name,
            contains: Vec::new(),
        });

        bag_num
    }

    fn parse_rule(&mut self, rule: &str) -> Result<(), RuleError> {
        let Some(rule) = rule.strip_suffix('.')
        else {
            return Err(RuleError::MissingFullStop);
        };

        let Some((name, tail)) = rule.split_once(" bags contain ")
        else {
            return Err(RuleError::MissingContains);
        };

        let bag = self.get_bag(name);

        if tail == "no other bags" {
            return Ok(());
        }

        for sub_bag in tail.split(", ") {
            let Some((num, tail)) = sub_bag.split_once(" ")
            else {
                return Err(RuleError::BadNumber);
            };

            let Ok(amount) = num.parse::<u32>()
            else {
                return Err(RuleError::BadNumber);
            };

            let name = match tail.strip_suffix(" bags") {
                Some(name) => name,
                None => match tail.strip_suffix(" bag") {
                    Some(name) => name,
                    None => {
                        return Err(RuleError::MissingNameTerminator)
                    },
                },
            };

            let sub_bag = self.get_bag(name);

            self.bags[bag].contains.push(BagSpace { amount, bag: sub_bag });
        }

        Ok(())
    }
}

fn read_bag_set() -> Result<BagSet, String> {
    let mut bag_set = BagSet::new();

    for (line_num, line) in std::io::stdin().lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(e) => return Err(e.to_string()),
        };

        if let Err(e) = bag_set.parse_rule(&line) {
            return Err(format!("line {}: {}", line_num + 1, e));
        }
    }

    Ok(bag_set)
}

fn main() -> ExitCode {
    let bag_set = match read_bag_set() {
        Ok(bag_set) => bag_set,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    ExitCode::SUCCESS
}
