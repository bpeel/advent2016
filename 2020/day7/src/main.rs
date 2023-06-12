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
    bag: usize,
}

struct BagSet {
    bags: Vec<Bag>,
}

impl BagSet {
    fn get_bag(&mut self, name: &str) -> usize {
        for (bag_num, bag) in self.bags.iter().enumerate() {
            if bag.name == name {
                return bag_num;
            }
        }

        self.bags.push(Bag { name: name.to_string(), contains: Vec::new() });

        self.bags.len() - 1
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

            self.bags[bag].contains.push(BagSpace { amount, bag: sub_bag });
        }

        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
