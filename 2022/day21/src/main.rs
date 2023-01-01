use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug, Clone)]
enum NamedExpression {
    Constant(i64),
    Operation { a: u32, op: Operator, b: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
}

impl Operator {
    fn apply(self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Plus => a + b,
            Operator::Minus => a - b,
            Operator::Divide => a / b,
            Operator::Multiply => a * b,
        }
    }
}

#[derive(Debug)]
enum Expression {
    Constant(i64),
    Operation {
        a: Box<Expression>,
        op: Operator,
        b: Box<Expression>,
    },
    Marker,
}

enum BuildStackEntry {
    NeedLeft(Operator, u32),
    NeedRight(Box<Expression>, Operator),
}

enum FoldStackEntry<'a> {
    NeedLeft(Operator, &'a Expression),
    NeedRight(Expression, Operator),
}

fn monkey_name_to_int(name: &str) -> u32 {
    assert_eq!(name.len(), 4);

    let mut result = 0u32;

    for b in name.bytes() {
        result = (result << 8) | b as u32;
    }

    result
}

fn int_to_monkey_name(num: u32) -> String {
    String::from_utf8(num.to_be_bytes().to_vec()).unwrap()
}

fn read_monkies<I>(lines: &mut I) ->
    Result<HashMap<u32, NamedExpression>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let re = regex::Regex::new("^([a-z]{4}): (?:(\\d+)|([a-z]{4}) ([+/*-]) \
                                ([a-z]{4}))$").unwrap();
    let mut monkies = HashMap::new();

    for (line_num, result) in lines.enumerate() {
        let line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        let captures = match re.captures(&line) {
            Some(c) => c,
            None => return Err(format!("line {}: invalid syntax",
                                       line_num + 1)),
        };

        let monkey_name = monkey_name_to_int(&captures[1]);

        let expression = match captures.get(2) {
            Some(constant) => {
                let constant = match constant.as_str().parse::<i64>() {
                    Ok(c) => c,
                    Err(e) => return Err(format!("line {}: {}",
                                                 line_num + 1,
                                                 e)),
                };
                NamedExpression::Constant(constant)
            },
            None => {
                let a = monkey_name_to_int(&captures[3]);
                let b = monkey_name_to_int(&captures[5]);

                let op = match &captures[4] {
                    "+" => Operator::Plus,
                    "-" => Operator::Minus,
                    "/" => Operator::Divide,
                    "*" => Operator::Multiply,
                    _ => panic!("impossible operator: {}", &captures[4]),
                };

                NamedExpression::Operation { a, op, b }
            },
        };

        match monkies.entry(monkey_name) {
            Entry::Occupied(_) => {
                return Err(format!("line {}: duplicate monkey “{}”",
                                   line_num + 1,
                                   &captures[1]));
            },
            Entry::Vacant(e) => { e.insert(expression); },
        }
    }

    Ok(monkies)
}

fn build_expression(monkies: &HashMap<u32, NamedExpression>,
                    monkey: u32,
                    marker: Option<u32>) ->
    Result<Expression, String>
{
    let mut stack = Vec::<BuildStackEntry>::new();
    let mut next_monkey = monkey;

    loop {
        let mut expression = match marker {
            Some(m) if m == next_monkey => Expression::Marker,
            _ => {
                let named_expression = match monkies.get(&next_monkey) {
                    None => {
                        return Err(format!("missing monkey {}",
                                           int_to_monkey_name(next_monkey)));
                    },
                    Some(e) => e,
                };

                match named_expression {
                    NamedExpression::Constant(c) =>
                        Expression::Constant(*c),
                    &NamedExpression::Operation { a: left, op, b: right } => {
                        stack.push(BuildStackEntry::NeedLeft(op, right));
                        next_monkey = left;
                        continue;
                    },
                }
            },
        };

        loop {
            match stack.pop() {
                None => return Ok(expression),
                Some(BuildStackEntry::NeedLeft(op, right)) => {
                    let left = Box::new(expression);
                    stack.push(BuildStackEntry::NeedRight(left, op));
                    next_monkey = right;
                    break;
                },
                Some(BuildStackEntry::NeedRight(left, op)) => {
                    expression =
                        Expression::Operation {
                            a: left,
                            op,
                            b: Box::new(expression),
                        };
                },
            };
        }
    }
}

fn fold_constants(expression: &Expression) -> Expression {
    let mut stack = Vec::<FoldStackEntry>::new();
    let mut next_expression = expression;

    loop {
        let mut expression = match next_expression {
            Expression::Constant(c) =>
                Expression::Constant(*c),
            Expression::Marker =>
                Expression::Marker,
            &Expression::Operation { a: ref left, op, b: ref right } => {
                stack.push(FoldStackEntry::NeedLeft(op, right));
                next_expression = left;
                continue;
            },
        };

        loop {
            match stack.pop() {
                None => return expression,
                Some(FoldStackEntry::NeedLeft(op, right)) => {
                    let entry =
                        FoldStackEntry::NeedRight(expression, op);
                    stack.push(entry);
                    next_expression = right;
                    break;
                },
                Some(FoldStackEntry::NeedRight(left, op)) => {
                    if let Expression::Constant(a) = left {
                        if let Expression::Constant(b) = expression {
                            let r = op.apply(a, b);
                            expression = Expression::Constant(r);
                            continue;
                        }
                    }

                    expression =
                        Expression::Operation {
                            a: Box::new(left),
                            op,
                            b: Box::new(expression),
                        };
                },
            };
        }
    }
}

fn equate_expression(mut expression: &Expression, mut value: i64) ->
    Result<i64, String>
{
    loop {
        match expression {
            Expression::Constant(_) => {
                break Err("tried to equate value with constant \
                           expression".to_string());
            },
            Expression::Marker => {
                break Ok(value);
            },
            &Expression::Operation { ref a, op, ref b } => {
                let is_right;
                let constant;

                if let &Expression::Constant(c) = a.as_ref() {
                    expression = &b;
                    constant = c;
                    is_right = false;
                } else if let &Expression::Constant(c) = b.as_ref() {
                    expression = &a;
                    constant = c;
                    is_right = true;
                } else {
                    return Err("neither side of folded expression is a \
                                constant".to_string());
                };

                value = match op {
                    Operator::Plus => value - constant,
                    Operator::Minus => {
                        if is_right {
                            value + constant
                        } else {
                            constant - value
                        }
                    },
                    Operator::Divide => {
                        if is_right {
                            value * constant
                        } else {
                            constant / value
                        }
                    },
                    Operator::Multiply => value / constant,
                }
            },
        }
    }
}

fn part1(monkies: &HashMap<u32, NamedExpression>) -> Result<i64, String> {
    let expression = build_expression(&monkies,
                                      monkey_name_to_int("root"),
                                      None)?;

    match fold_constants(&expression) {
        Expression::Constant(c) => Ok(c),
        _ => panic!("constant folding didn’t fold to a constant"),
    }
}

fn part2(monkies: &HashMap<u32, NamedExpression>) -> Result<i64, String> {
    let expression = build_expression(&monkies,
                                      monkey_name_to_int("root"),
                                      Some(monkey_name_to_int("humn")))?;

    let (a, b) = match fold_constants(&expression) {
        Expression::Constant(_) =>
            return Err("root is a constant".to_string()),
        Expression::Marker =>
            return Err("root is a marker".to_string()),
        Expression::Operation { a, b, .. } =>
            (a, b),
    };

    let (expression, constant) = if let Expression::Constant(c) = *a {
        (b, c)
    } else if let Expression::Constant(c) = *b {
        (a, c)
    } else {
        return Err("neither side of folded root expression is a \
                    constant".to_string());
    };

    equate_expression(&expression, constant)
}

fn main() -> std::process::ExitCode {
    let monkies = match read_monkies(&mut std::io::stdin().lines()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(m) => m,
    };

    let mut ret = std::process::ExitCode::SUCCESS;

    print!("part 1: ");

    match part1(&monkies) {
        Err(e) => {
            println!("{}", e);
            ret = std::process::ExitCode::FAILURE;
        },
        Ok(v) => println!("{}", v),
    }

    print!("part 2: ");

    match part2(&monkies) {
        Err(e) => {
            println!("{}", e);
            ret = std::process::ExitCode::FAILURE;
        },
        Ok(v) => println!("{}", v),
    }

    ret
}
