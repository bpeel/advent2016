#[derive(Debug, Clone)]
enum Operation {
    Multiply(i64),
    Add(i64),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test_divisor: i64,
    targets: [usize; 2],
    throw_count: usize,
}

fn read_monkey<I>(lines: &mut I) -> Result<Monkey, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    lines.next().unwrap().unwrap();

    let starting_items = lines.next().unwrap().unwrap();
    let re = regex::Regex::new(r"^  Starting items: (.*)$").unwrap();
    let starting_items = re.captures(&starting_items).unwrap()[1].to_string();
    let starting_items = starting_items.split(", ").map(|item| {
        item.parse::<i64>().unwrap()
    }).collect::<Vec<i64>>();

    let operation = lines.next().unwrap().unwrap();
    let operation = if operation == "  Operation: new = old * old" {
        Operation::Square
    } else {
        let re =
            regex::Regex::new(r"^  Operation: new = old ([+*]) (\d+)").unwrap();
        let captures = re.captures(&operation).unwrap();
        if &captures[1] == "*" {
            Operation::Multiply(captures[2].parse::<i64>().unwrap())
        } else {
            Operation::Add(captures[2].parse::<i64>().unwrap())
        }
    };

    let test = lines.next().unwrap().unwrap();
    let re = regex::Regex::new(r"^  Test: divisible by (\d+)").unwrap();
    let captures = re.captures(&test).unwrap();
    let test = captures[1].parse::<i64>().unwrap();

    let mut targets = [0usize; 2];

    for _ in 0..targets.len() {
        let target = lines.next().unwrap().unwrap();
        let re = regex::Regex::new(
            r"^    If (true|false): throw to monkey (\d+)").unwrap();
        let captures = re.captures(&target).unwrap();
        let tnum = if &captures[1] == "true" { 1 } else { 0 };
        let target = captures[2].parse::<usize>().unwrap();
        targets[tnum] = target;
    }

    Ok(Monkey {
        items: starting_items,
        operation,
        test_divisor: test,
        targets,
        throw_count: 0,
    })
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

            match monkey.operation {
                Operation::Multiply(amount) => item *= amount,
                Operation::Add(amount) => item += amount,
                Operation::Square => item *= item,
            }

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

fn main() -> std::process::ExitCode {
    let mut monkies = Vec::<Monkey>::new();

    let mut lines = std::io::stdin().lines();

    loop {
        monkies.push(read_monkey(&mut lines).unwrap());

        if matches!(lines.next(), None) {
            break;
        }
    }

    println!("part 1: {}", run_part(&monkies, 20, true));
    println!("part 2: {}", run_part(&monkies, 10_000, false));

    std::process::ExitCode::SUCCESS
}
