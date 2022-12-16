use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: u8,
    tunnels: Vec<u16>,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    OpenValve,
    TakeTunnel(u8),
}

#[derive(Debug)]
struct Walker<'a> {
    stack: Vec<(Action, u16)>,
    pos: u16,
    open_valves: HashMap<u16, u8>,
    valves: &'a HashMap<u16, Valve>,
    best_score: usize,
}

enum VisitResult {
    Continue,
    Backtrack,
}

impl<'a> Walker<'a> {
    fn new(valves: &'a HashMap<u16, Valve>) -> Walker<'a> {
        Walker {
            stack: Vec::<(Action, u16)>::new(),
            pos: valve_num(&"AA"),
            open_valves: HashMap::<u16, u8>::new(),
            valves,
            best_score: usize::MIN,
        }
    }

    fn visit(&mut self) -> VisitResult {
        if self.stack.len() > TOTAL_TIME {
            return VisitResult::Backtrack;
        }

        if let Some((Action::OpenValve, _)) = self.stack.last() {
            match self.open_valves.entry(self.pos) {
                Entry::Occupied(_) => return VisitResult::Backtrack,
                Entry::Vacant(e) => { e.insert(self.stack.len() as u8); },
            }
        }

        if self.stack.len() >= TOTAL_TIME {
            self.score_actions();
        }

        VisitResult::Continue
    }

    fn score_actions(&mut self) {
        let score = self.open_valves
            .iter()
            .map(|(valve, &open_time)|
                 self.valves[valve].flow_rate as usize *
                 (TOTAL_TIME - open_time as usize))
            .sum::<usize>();

        if score > self.best_score {
            print!("{:4}: AA", score);

            for (action, last_pos) in self.stack.iter() {
                print!(",");
                match action {
                    Action::OpenValve => print!("op"),
                    Action::TakeTunnel(tunnel_num) => {
                        let next_valve =
                            self.valves[last_pos].tunnels[*tunnel_num as usize];
                        print!("{}", valve_name(next_valve));
                    },
                }
            }

            println!("");

            self.best_score = score;
        }
    }

    fn take_tunnel(&mut self, tunnel_num: usize) -> bool {
        let valve = &self.valves[&self.pos];

        if let Some(&valve) = valve.tunnels.get(tunnel_num) {
            self.stack.push((Action::TakeTunnel(tunnel_num as u8), self.pos));
            self.pos = valve;
            true
        } else {
            false
        }
    }

    fn backtrack(&mut self) -> bool {
        loop {
            let (last_action, last_pos) = match self.stack.pop() {
                Some(s) => s,
                None => return false,
            };

            let next_tunnel = match last_action {
                Action::OpenValve => {
                    self.open_valves.remove(&self.pos);
                    0
                },
                Action::TakeTunnel(t) => {
                    self.pos = last_pos;
                    t as usize + 1
                },
            };

            if self.take_tunnel(next_tunnel) {
                return true;
            }
        }
    }

    fn walk(&mut self) {
        loop {
            match self.visit() {
                VisitResult::Continue => {
                    match self.stack.last() {
                        Some((Action::OpenValve, _)) => {
                            if !self.take_tunnel(0) && !self.backtrack() {
                                break;
                            }
                        },
                        _ => {
                            self.stack.push((Action::OpenValve, self.pos));
                        },
                    };
                },
                VisitResult::Backtrack => {
                    if !self.backtrack() {
                        break;
                    }
                }
            }
        }
    }
}

const TOTAL_TIME: usize = 30;

fn valve_num(name: &str) -> u16 {
    // The valve name should be validated by the regexp so we can
    // just use unwrap here
    (((name.as_bytes()[0] - b'A') as u16) << 8) |
    (name.as_bytes()[1] - b'A') as u16
}

fn valve_name(num: u16) -> String {
    format!("{}{}",
            ((num >> 8) as u8 + b'A') as char,
            ((num & 0xff) as u8 + b'A') as char)
}

fn read_valves<I>(lines: &mut I) -> Result<HashMap<u16, Valve>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let re = regex::Regex::new("^Valve ([A-Z]{2}) has flow rate=(\\d+); \
                                tunnels? leads? to valves? ((?:[A-Z]{2}, )*\
                                [A-Z]{2})$").unwrap();
    let mut valves = HashMap::<u16, Valve>::new();

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

        let flow_rate = match captures[2].parse::<u8>() {
            Err(e) => return Err(e.to_string()),
            Ok(r) => r,
        };

        let this_valve_num = valve_num(&captures[1]);

        if let Some(_) = valves.get(&this_valve_num) {
            return Err(format!("line {}: duplicate tunnel", line_num + 1));
        }

        valves.insert(this_valve_num, Valve {
            flow_rate,
            tunnels: captures[3]
                .split(", ")
                .map(|s| valve_num(s))
                .collect(),
        });
    }

    for (&valve_num, valve) in valves.iter() {
        for &target_valve in valve.tunnels.iter() {
            if let None = valves.get(&target_valve) {
                return Err(format!("valve {} links to non-existant valve {}",
                                   valve_name(valve_num),
                                   valve_name(target_valve)));
            }
        }
    }

    if let None = valves.get(&valve_num(&"AA")) {
        return Err("missing valve AA".to_string());
    }

    Ok(valves)
}

fn main() -> std::process::ExitCode {
    let valves = match read_valves(&mut std::io::stdin().lines()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(valves) => valves,
    };

    Walker::new(&valves).walk();

    std::process::ExitCode::SUCCESS
}
