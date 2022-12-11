#[derive(Debug, Clone)]
enum Operation {
    Multiply(i64),
    Add(i64),
    Square,
}

impl Operation {
    fn apply(&self, val: i64) -> i64 {
        match self {
            Operation::Multiply(o) => val * o,
            Operation::Add(o) => val + o,
            Operation::Square => val * val,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test_divisor: i64,
    targets: [usize; 2],
    throw_count: usize,
}

impl std::str::FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new("\
            \\AMonkey \\d+:\n\
            \x20 Starting items: (?P<items>[\\d, ]+)\n\
            \x20 Operation: new = old \
            (?:(?P<square>\\* old)|(?P<op>[*+]) (?P<amount>\\d+))\n\
            \x20 Test: divisible by (?P<test>\\d+)\n\
            \x20   If true: throw to monkey (?P<true_monkey>\\d+)\n\
            \x20   If false: throw to monkey (?P<false_monkey>\\d+)\
            \\s*\\z").unwrap();

        let captures = match re.captures(s) {
            None => return Err("invalid monkey".to_string()),
            Some(c) => c,
        };

        let split_re = regex::Regex::new(r",\s*").unwrap();

        let mut items = Vec::<i64>::new();

        for item in split_re.split(&captures["items"]) {
            items.push(match item.parse::<i64>() {
                Err(e) => return Err(format!("{}: {}", item, e)),
                Ok(i) => i,
            });
        }

        let operation = match captures.name("square") {
            Some(_) => Operation::Square,
            None => {
                let amount = match captures["amount"].parse::<i64>() {
                    Err(e) => return Err(format!("{}: {}",
                                                 &captures["amount"],
                                                 e)),
                    Ok(a) => a,
                };

                match &captures["op"] {
                    "*" => Operation::Multiply(amount),
                    "+" => Operation::Add(amount),
                    _ => panic!("unexpected op shouldn’t have been matched \
                                 by the regexp"),
                }
            }
        };

        let test_divisor = match captures["test"].parse::<i64>() {
            Err(e) => return Err(format!("test divisor: {}", e)),
            Ok(d) => d,
        };

        let targets = [
            match captures["false_monkey"].parse::<usize>() {
                Err(e) => return Err(format!("false monkey: {}", e)),
                Ok(m) => m,
            },
            match captures["true_monkey"].parse::<usize>() {
                Err(e) => return Err(format!("true monkey: {}", e)),
                Ok(m) => m,
            },
        ];

        Ok(Monkey { items, operation, test_divisor, targets, throw_count: 0 })
    }
}

fn run_round(monkies: &mut Vec<Monkey>,
             wrapper: i64,
             divide_worry_level: bool) {
    let mut to_throw = Vec::<(usize, i64)>::new();

    for monkey_num in 0..monkies.len() {
        to_throw.clear();

        let monkey = monkies.get_mut(monkey_num).unwrap();

        monkey.throw_count += monkey.items.len();

        while monkey.items.len() > 0 {
            let mut item = monkey.items.pop().unwrap();

            item = monkey.operation.apply(item);

            if divide_worry_level {
                item /= 3;
            }

            item %= wrapper;

            let target = monkey.targets[(item % monkey.test_divisor == 0)
                                        as usize];
            to_throw.push((target, item));
        }

        for (target, item) in to_throw.iter().rev() {
            monkies[*target].items.push(*item);
        }
    }
}

fn run_part(monkies: &[Monkey],
            n_rounds: usize,
            divide_worry_level: bool) -> String {
    let mut monkies = monkies.to_vec();

    let wrapper = monkies.iter().map(|m| m.test_divisor)
        .fold(1, |a, b| a * b);

    for _ in 0..n_rounds {
        run_round(&mut monkies, wrapper, divide_worry_level);
    }

    monkies.sort_by(|a, b| b.throw_count.cmp(&a.throw_count));

    let a = monkies[0].throw_count;
    let b = monkies[1].throw_count;

    format!("{} * {} = {}", a, b, a * b)
}

fn read_monkies() -> Result<Vec<Monkey>, String> {
    let monkies_str = match std::io::read_to_string(std::io::stdin().lock()) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s,
    };

    let mut monkies = Vec::<Monkey>::new();

    for monkey_str in monkies_str.split("\n\n") {
        match monkey_str.parse::<Monkey>() {
            Err(s) => return Err(s),
            Ok(m) => monkies.push(m),
        }
    }

    Ok(monkies)
}

fn main() -> std::process::ExitCode {
    let monkies = match read_monkies() {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(m) => m,
    };

    println!("part 1: {}", run_part(&monkies, 20, true));
    println!("part 2: {}", run_part(&monkies, 10_000, false));

    std::process::ExitCode::SUCCESS
}
