use std::process::ExitCode;
use std::str::FromStr;
use std::collections::HashMap;
use std::fmt;

struct Rule {
    before: u8,
    after: u8,
}

struct Data {
    rules: Vec<Rule>,
    updates: Vec<Vec<u8>>,
}

enum ValidationError {
    NoRule(u8, u8),
    Cycle(u8),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ValidationError::NoRule(before, after) => {
                write!(
                    f,
                    "no rule to put {} before {}",
                    before,
                    after
                )
            },
            ValidationError::Cycle(page) => {
                write!(
                    f,
                    "page {} is before itself",
                    page,
                )
            },
        }
    }
}

type RuleBits = HashMap<u8, u128>;

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Rule, String> {
        let Some((before, after)) = s.split_once('|')
        else {
            return Err("missing ‘|’".to_string());
        };

        let rule = Rule {
            before: before.parse::<u8>().map_err(|e| e.to_string())?,
            after: after.parse::<u8>().map_err(|e| e.to_string())?,
        };

        if rule.before >= 128 || rule.after >= 128 {
            return Err("page number too high".to_string());
        }

        Ok(rule)
    }
}

fn read_rules<I>(lines: &mut I) -> Result<Vec<Rule>, String>
    where I: Iterator<Item = (usize, Result<String, std::io::Error>)>
{
    let mut rules = Vec::new();

    for (line_num, line) in lines {
        let line = line.map_err(|e| e.to_string())?;

        if line.is_empty() {
            break;
        }

        let rule = line.parse().map_err(|e| {
            format!("line {}: {}", line_num + 1, e)
        })?;

        rules.push(rule);
    }

    Ok(rules)
}

fn read_updates<I>(lines: &mut I) -> Result<Vec<Vec<u8>>, String>
    where I: Iterator<Item = (usize, Result<String, std::io::Error>)>
{
    let mut updates = Vec::new();

    for (line_num, line) in lines {
        let line = line.map_err(|e| e.to_string())?;
        let mut pages = Vec::new();

        for page in line.split(',') {
            pages.push(page.parse().map_err(|e| {
                format!("line {}: {}", line_num + 1, e)
            })?);
        }

        if pages.len() & 1 == 0 {
            return Err(format!(
                "line {}: update has no middle page",
                line_num + 1,
            ));
        }

        updates.push(pages);
    }

    Ok(updates)
}

fn read_data<I>(lines: I) -> Result<Data, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let mut lines = lines.enumerate();
    let rules = read_rules(&mut lines)?;
    let updates = read_updates(&mut lines)?;

    Ok(Data { rules, updates })
}

fn rules_to_bitset<'a, I: IntoIterator<Item = &'a Rule>>(rules: I) -> RuleBits {
    let mut rule_bits = HashMap::new();

    for rule in rules {
        let bit = 1u128 << rule.after;

        rule_bits.entry(rule.before)
            .and_modify(|mask| *mask |= bit)
            .or_insert(bit);
    }

    rule_bits
}

fn all_befores(rule_bits: &RuleBits, top_rule: u8) -> u128 {
    let mut mask = 0u128;
    let mut stack = vec![top_rule];

    while let Some(rule_num) = stack.pop() {
        if let Some(&(mut bits)) = rule_bits.get(&rule_num) {
            bits &= !mask;
            mask |= bits;

            while bits != 0 {
                let next_bit = bits.trailing_zeros();
                stack.push(next_bit as u8);
                bits &= !(1u128 << next_bit);
            }
        }
    }

    mask
}

fn fill_befores(rule_bits: &RuleBits) -> RuleBits {
    rule_bits
        .keys()
        .map(|&rule_num| (rule_num, all_befores(rule_bits, rule_num)))
        .collect::<RuleBits>()
}

fn rule_bits_for_update(rules: &[Rule], update: &[u8]) -> RuleBits {
    // Get a bitmask of all pages in the update
    let page_mask = update.iter().fold(0u128, {
        |mask, rule| mask | 1u128 << rule
    });

    let rule_bits = rules_to_bitset(
        rules.iter()
        // Filter out rules that don’t concern the pages in the update
            .filter(|rule| {
                page_mask & (1u128 << rule.before) != 0 &&
                    page_mask & (1u128 << rule.after) != 0
            })
    );

    fill_befores(&rule_bits)
}

fn validate_update(
    rule_bits: &RuleBits,
    update: &[u8],
) -> Result<(), ValidationError> {
    for (&rule, &bits) in rule_bits.iter() {
        if bits & (1u128 << rule) != 0 {
            return Err(ValidationError::Cycle(rule));
        }
    }

    let Some(&(mut next)) = update.last()
    else {
        return Ok(());
    };

    for &page in update.iter().rev().skip(1) {
        rule_bits
            .get(&page)
            .and_then(|bits| (bits & (1u128 << next) != 0).then_some(()))
            .ok_or_else(|| {
                ValidationError::NoRule(page, next)
            })?;

        next = page;
    }

    Ok(())
}

fn main() -> ExitCode {
    let data = match read_data(std::io::stdin().lines()) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    let mut part1 = 0;
    let mut part2 = 0;

    for (update_num, update) in data.updates.iter().enumerate() {
        let rule_bits = rule_bits_for_update(&data.rules, update);

        match validate_update(&rule_bits, update) {
            Err(e) => {
                println!("update {}: {}", update_num + 1, e);

                let mut correct_order = update.to_vec();
                correct_order.sort_by_key(|page| {
                    u32::MAX -
                        rule_bits
                        .get(page)
                        .map(|bits| bits.count_ones())
                        .unwrap_or(0)
                });

                part2 += correct_order[update.len() / 2] as u32;
            },
            Ok(()) => {
                println!("update {}: OK", update_num + 1);
                part1 += update[update.len() / 2] as u32;
            }
        }
    }

    println!(
        "Part 1: {}\n\
         Part 2: {}",
        part1,
        part2,
    );

    ExitCode::SUCCESS
}
