use std::process::ExitCode;
use std::sync::LazyLock;
use std::str::FromStr;

static CLAW_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        "Button A: X\\+(\\d+), Y\\+(\\d+)\n\
         Button B: X\\+(\\d+), Y\\+(\\d+)\n\
         Prize: X=(\\d+), Y=(\\d+)\n*"
    ).unwrap()
});

#[derive(Clone, Debug)]
struct ClawMachine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

impl FromStr for ClawMachine {
    type Err = String;

    fn from_str(s: &str) -> Result<ClawMachine, String> {
        let Some(captures) = CLAW_RE.captures(s)
        else {
            return Err("bad claw machine".to_string());
        };

        let mut parts: [u64; 6] = Default::default();

        for (i, p) in parts.iter_mut().enumerate() {
            match captures[i + 1].parse::<u64>() {
                Ok(v) => *p = v,
                Err(_) => {
                    return Err("bad number".to_string());
                },
            }
        }

        Ok(ClawMachine {
            a: (parts[0], parts[1]),
            b: (parts[2], parts[3]),
            prize: (parts[4], parts[5]),
        })
    }
}

impl ClawMachine {
    fn prioritise(
        &self,
        best: (u64, u64),
        worst: (u64, u64)
    ) -> Option<(u64, u64)> {
        let max_best = (self.prize.0 / best.0)
            .min(self.prize.1 / best.1);

        for n_best in (0..=max_best).rev() {
            let rem = (self.prize.0 - best.0 * n_best,
                       self.prize.1 - best.1 * n_best);

            if rem.0 % worst.0 != 0 {
                continue;
            }

            let n_worst = rem.0 / worst.0;

            if n_worst * worst.1 == rem.1 {
                return Some((n_best, n_worst));
            }
        }

        None
    }

    fn best_strategy(&self) -> Option<(u64, u64)> {
        // One of the buttons has a better cost/distance ratio, so
        // prioritise that one
        if self.a.0 + self.a.1 > (self.b.0 + self.b.1) * 3 {
            self.prioritise(self.a, self.b)
        } else {
            self.prioritise(self.b, self.a)
                .map(|(b, a)| (a, b))
        }
    }
}

fn read_claw_machines() -> Result<Vec<ClawMachine>, String> {
    let source = std::io::read_to_string(std::io::stdin().lock())
        .map_err(|e| e.to_string())?;

    let mut machines = Vec::new();

    for source in source.split("\n\n") {
        machines.push(source.parse::<ClawMachine>()?);
    }

    Ok(machines)
}

fn best_cost(machines: &[ClawMachine]) -> u64 {
    machines.iter().filter_map(|machine| {
        machine.best_strategy().map(|(a, b)| {
            println!("{},{} for {:?}", a, b, machine);
            a * 3 + b
        })
    }).sum::<u64>()
}

fn main() -> ExitCode {
    let machines = match read_claw_machines() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    println!("Part 1: {}", best_cost(&machines));

    let machines = machines.into_iter().map(|mut machine| {
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
        machine
    }).collect::<Vec<_>>();

    println!("Part 2: {}", best_cost(&machines));

    ExitCode::SUCCESS
}