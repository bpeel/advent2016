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
    a: (u32, u32),
    b: (u32, u32),
    prize: (u32, u32),
}

impl FromStr for ClawMachine {
    type Err = String;

    fn from_str(s: &str) -> Result<ClawMachine, String> {
        let Some(captures) = CLAW_RE.captures(s)
        else {
            return Err("bad claw machine".to_string());
        };

        let mut parts: [u32; 6] = Default::default();

        for (i, p) in parts.iter_mut().enumerate() {
            match captures[i + 1].parse::<u32>() {
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

pub fn div_up(a: u32, b: u32) -> u32 {
    (a + (b - 1)) / b
}

impl ClawMachine {
    fn most_presses(&self) -> u32 {
        div_up(self.prize.0, self.a.0)
            .max(div_up(self.prize.0, self.b.0))
            .max(div_up(self.prize.1, self.a.1))
            .max(div_up(self.prize.1, self.b.1))
    }

    fn best_strategy(&self) -> Option<(u32, u32)> {
        (0..self.most_presses())
            .filter_map(|num_a| {
                let after_a = (num_a * self.a.0, num_a * self.a.1);
                let rem = (
                    self.prize.0.checked_sub(after_a.0)?,
                    self.prize.1.checked_sub(after_a.1)?,
                );

                let b0 = rem.0 / self.b.0;
                let b1 = rem.1 / self.b.1;

                (b0 == b1 &&
                 rem.0 % self.b.0 == 0 &&
                 rem.1 % self.b.1 == 0)
                    .then(|| (num_a, b0))
            })
            .min_by_key(|(a, b)| a * 3 + b)
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

fn main() -> ExitCode {
    let machines = match read_claw_machines() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    let part1 = machines.iter().filter_map(|machine| {
        machine.best_strategy().map(|(a, b)| {
            println!("{},{} for {:?}", a, b, machine);
            a * 3 + b
        })
    }).sum::<u32>();

    println!("Part 1: {}", part1);

    ExitCode::SUCCESS
}
