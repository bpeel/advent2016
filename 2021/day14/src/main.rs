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

type PairCount = HashMap<PolymerPair, usize>;

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

    fn apply(&self, polymer: &PairCount) -> PairCount {
        let mut result = HashMap::new();

        for (pair, &count) in polymer {
            match self.rules.get(pair) {
                Some(&v) => {
                    *result.entry(PolymerPair { a: pair.a, b: v }).or_default() += count; 
                    *result.entry(PolymerPair { a: v, b: pair.b }).or_default() += count; 
                },
                None => {
                    result.insert(pair.clone(), count);
                },
            }
        }

        result
    }
}

fn count_pairs(s: &str) -> PairCount {
    let mut counts = PairCount::new();
    let mut chars = s.chars();

    while let Some(a) = chars.next() {
        let b = match chars.clone().next() {
            Some(b) => b,
            None => {
                counts.insert(PolymerPair { a, b: 0 as char }, 1);
                break;
            },
        };

        *counts.entry(PolymerPair { a, b }).or_default() += 1;
    }

    counts
}

fn count_chars(polymer: &PairCount) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    for (&PolymerPair { a, .. }, count) in polymer.iter() {
        *result.entry(a).or_default() += count;
    }

    result
}

fn format_result(polymer: &PairCount) -> String {
    let counts = count_chars(&polymer);
    let min = counts.values().map(|&v| v).min().unwrap();
    let max = counts.values().map(|&v| v).max().unwrap();
    format!("{} - {} = {}", max, min, max - min)
}


fn main() {
    let mut lines = std::io::stdin().lines();

    let base_polymer = lines.next().unwrap().unwrap();
    assert_eq!(lines.next().unwrap().unwrap(), "");

    let mut template = PolymerTemplate::new();

    for line in lines {
        template.add_rule(&line.unwrap().parse().unwrap());
    }

    let mut polymer = count_pairs(&base_polymer);

    for i in 0..40 {
        if i == 10 {
            println!("part 1: {}", format_result(&polymer));
        }
        polymer = template.apply(&polymer);
    }

    println!("part 2: {}", format_result(&polymer));
}
