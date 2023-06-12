use std::process::ExitCode;
use std::fmt;
use std::collections::{HashMap, HashSet};

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
    contains: Vec<BagSpace>,
}

struct BagSpace {
    amount: u32,
    bag: usize,
}

struct BagSet {
    names: HashMap<String, usize>,
    bags: Vec<Bag>,
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

        let bag_num = self.bags.len();

        self.names.insert(name.to_string(), bag_num);

        self.bags.push(Bag {
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

    fn containers(&self, bag_name: &str) -> ContainerIter {
        let mut iter = ContainerIter {
            bag_set: self,
            sub_bags: Vec::new(),
            visited: HashSet::new(),
        };

        if let Some(bag_num) = self.names.get(bag_name) {
            iter.queue_containers(*bag_num);
        }

        iter
    }
}

struct ContainerIter<'a> {
    bag_set: &'a BagSet,
    sub_bags: Vec<usize>,
    visited: HashSet<usize>,
}

impl<'a> Iterator for ContainerIter<'a> {
    type Item = &'a Bag;

    fn next(&mut self) -> Option<&'a Bag> {
        if let Some(bag_num) = self.sub_bags.pop() {
            self.queue_containers(bag_num);
            Some(&self.bag_set.bags[bag_num])
        } else {
            None
        }
    }
}

impl<'a> ContainerIter<'a> {
    fn queue_containers(&mut self, bag_num: usize) {
        for (container_num, container) in self.bag_set.bags.iter().enumerate() {
            if self.visited.contains(&container_num) {
                continue;
            }

            for space in container.contains.iter() {
                if space.bag == bag_num {
                    self.visited.insert(container_num);
                    self.sub_bags.push(container_num);
                    break;
                }
            }
        }
    }
}

struct StackEntry<'a> {
    children: std::slice::Iter<'a, BagSpace>,
    count: u64,
    amount: u32,
}

fn part2(bag_set: &BagSet) -> u64 {
    let mut count: u64 = 0;
    let mut stack = Vec::<StackEntry>::new();

    if let Some(&bag_num) = bag_set.names.get("shiny gold") {
        stack.push(StackEntry {
            children: bag_set.bags[bag_num].contains.iter(),
            count: 0,
            amount: 1,
        });
    }

    while let Some(mut entry) = stack.pop() {
        match entry.children.next() {
            Some(space) => {
                stack.push(entry);
                stack.push(StackEntry {
                    children: bag_set.bags[space.bag].contains.iter(),
                    count: 1,
                    amount: space.amount,
                });
            },
            None => {
                let to_add = entry.count * entry.amount as u64;

                match stack.last_mut() {
                    Some(parent_entry) => parent_entry.count += to_add,
                    None => count += to_add,
                }
            },
        }
    }

    count
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

    println!("part 1: {}", bag_set.containers("shiny gold").count());
    println!("part 2: {}", part2(&bag_set));

    ExitCode::SUCCESS
}
