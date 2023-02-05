use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct PolymerPair {
    a: char,
    b: char,
}

#[derive(Debug, Clone)]
struct PolymerRule {
    pair: PolymerPair,
    insert: char,
}

impl FromStr for PolymerRule {
    type Err = String;

    fn from_str(s: &str) -> Result<PolymerRule, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.)(.) -> (.)$").unwrap();
        }

        let captures = match RE.captures(s) {
            None => return Err("invalid polymer rule".to_string()),
            Some(c) => c,
        };

        Ok(PolymerRule {
            pair: PolymerPair {
                a: captures[1].chars().next().unwrap(),
                b: captures[2].chars().next().unwrap(),
            },
            insert: captures[3].chars().next().unwrap(),
        })
    }
}

struct PolymerTemplate {
    rules: HashMap<PolymerPair, char>,
}

impl PolymerTemplate {
    fn new() -> PolymerTemplate {
        PolymerTemplate {
            rules: HashMap::new(),
        }
    }

    fn add_rule(&mut self, rule: &PolymerRule) {
        self.rules.insert(rule.pair, rule.insert);
    }

    fn apply(&self, polymer: &str) -> String {
        let mut result = String::new();
        let mut chars = polymer.chars();

        while let Some(left) = chars.next() {
            result.push(left);

            let right = match chars.clone().next() {
                Some(c) => c,
                None => break,
            };

            if let Some(&insert) = self.rules.get(&PolymerPair { a: left, b: right }) {
                result.push(insert);
            }
        }

        result
    }
}

fn count_values<I>(iter: I) -> HashMap<I::Item, usize>
where
    I: IntoIterator, I::Item: Eq + Hash
{
    let mut counts = HashMap::new();

    for value in iter {
        counts.entry(value)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    counts
}

fn main() {
    let mut lines = std::io::stdin().lines();

    let base_polymer = lines.next().unwrap().unwrap();
    assert_eq!(lines.next().unwrap().unwrap(), "");

    let mut template = PolymerTemplate::new();

    for line in lines {
        template.add_rule(&line.unwrap().parse().unwrap());
    }

    let mut polymer = base_polymer.clone();

    for _ in 0..10 {
        polymer = template.apply(&polymer);
        println!("{}", polymer);
    }

    let counts = count_values(polymer.chars());
    let min = counts.values().map(|&v| v).min().unwrap();
    let max = counts.values().map(|&v| v).max().unwrap();
    println!("part 1: {} - {} = {}", max, min, max - min);
}
