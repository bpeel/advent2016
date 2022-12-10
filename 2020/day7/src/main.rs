use std::rc::Rc;
use std::collections::HashMap;

enum RuleError {
    BadNumber,
    MissingContains,
    MissingNameTerminator,
}

struct Bag {
    name: String,
    contains: Vec<BagSpace>,
}

struct BagSpace {
    amount: u32,
    bag: Rc<Bag>,
}

struct BagSet {
    hash: HashMap<String, Rc<Bag>>,
}

impl BagSet {
    fn get_bag(&mut self, name: &str) -> Rc<Bag> {
        match self.hash.get(name) {
            Some(n) => n,
            None => {
                self.hash.insert(String::from(name),
                                 Rc::new(Bag { name: name.to_string(),
                                               contains: Vec::new() }));
                self.hash.get(name).unwrap()
            }
        }.clone()
    }

    fn parse_rule(&mut self, rule: &str) -> Result<(), RuleError> {
        let name_end = match rule.find(" bags contain ") {
            Some(n) => n,
            None => {
                return Err(RuleError::MissingContains);
            }
        };

        let bag = self.get_bag(&rule[0..name_end]);

        for name in rule[name_end..].split(",") {
            let num_end = match rule.find(" ") {
                Some(n) => n,
                None => {
                    return Err(RuleError::BadNumber);
                }
            };

            let amount: u32 = match rule[0..name_end].parse() {
                Ok(n) => n,
                Err(_) => {
                    return Err(RuleError::BadNumber);
                }
            };

            let name_end = match name.find(" bag") {
                Some(n) => n,
                None => {
                    return Err(RuleError::MissingNameTerminator)
                }
            };

            let sub_bag = self.get_bag(&name[num_end..name_end]);

            bag.contains.push(BagSpace { amount, bag: sub_bag.clone() });
        }

        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
