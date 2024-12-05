use std::process::ExitCode;
use std::str::FromStr;
use std::collections::HashMap;

struct Rule {
    before: u8,
    after: u8,
}

struct Data {
    rules: Vec<Rule>,
    updates: Vec<Vec<u8>>,
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

fn rules_to_bitset(rules: &[Rule]) -> RuleBits {
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

fn validate_update(
    rule_bits: &RuleBits,
    update: &[u8],
) -> Result<(), (u8, u8)> {
    let Some(&(mut next)) = update.last()
    else {
        return Ok(());
    };

    for &page in update.iter().rev().skip(1) {
        rule_bits
            .get(&page)
            .and_then(|bits| (bits & (1u128 << next) != 0).then_some(()))
            .ok_or_else(|| (page, next))?;

        next = page;
    }

    Ok(())
}

fn check_page_ordering(rule_bits: &RuleBits) -> bool {
    let mut pages = rule_bits.keys().cloned().collect::<Vec<_>>();
    let mut result = true;

    pages.sort_by_key(|page| {
        u32::MAX -
            rule_bits.get(page).map(|bits| bits.count_ones()).unwrap_or(0)
    });

    for (index, page) in pages.iter().enumerate() {
        let pages_after_bits = rule_bits.get(page).cloned().unwrap_or(0);
        let n_pages_after = pages_after_bits.count_ones();

        if n_pages_after as usize != pages.len() - index {
            if result {
                eprintln!("page order: {:?}", pages);
                result = false;
            }

            eprint!(
                "page {} at index {} has {} pages after:",
                page,
                index,
                n_pages_after,
            );

            let mut afters = pages_after_bits;

            while afters != 0 {
                let page = afters.trailing_zeros();
                eprint!(" {}", page);
                afters &= !(1u128 << page);
            }

            eprintln!();
        }
    }

    result
}

fn main() -> ExitCode {
    let data = match read_data(std::io::stdin().lines()) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    let rule_bits = fill_befores(&rules_to_bitset(&data.rules));

    if !check_page_ordering(&rule_bits) {
        return ExitCode::FAILURE;
    }

    let mut part1 = 0;

    for (update_num, update) in data.updates.iter().enumerate() {
        match validate_update(&rule_bits, update) {
            Err((before, after)) => {
                println!(
                    "update {}: no rule to put {} before {}",
                    update_num + 1,
                    before,
                    after,
                );
            },
            Ok(()) => {
                println!("update {}: OK", update_num + 1);
                part1 += update[update.len() / 2];
            }
        }
    }

    println!("Part 1: {}", part1);

    ExitCode::SUCCESS
}
