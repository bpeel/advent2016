use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: u8,
    tunnels: Vec<u16>,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    StayStill,
    OpenValve,
    TakeTunnel(u8),
}

#[derive(Debug)]
struct Walker<'a, const N_ACTORS: usize> {
    stack: Vec<([Action; N_ACTORS], [u16; N_ACTORS])>,
    pos: [u16; N_ACTORS],
    open_valves: HashMap<u16, u8>,
    valves: &'a HashMap<u16, Valve>,
    best_score: usize,
}

impl<'a, const N_ACTORS: usize> Walker<'a, N_ACTORS> {
    const START_TIME: usize = (N_ACTORS - 1) * 4;

    fn new(valves: &'a HashMap<u16, Valve>) -> Walker<'a, N_ACTORS> {
        Walker {
            stack: Vec::<([Action; N_ACTORS], [u16; N_ACTORS])>::new(),
            pos: [valve_num(&"AA"); N_ACTORS],
            open_valves: HashMap::<u16, u8>::new(),
            valves,
            best_score: usize::MIN,
        }
    }

    fn score_actions(&mut self) {
        let score = self.open_valves
            .iter()
            .map(|(valve, &open_time)|
                 self.valves[valve].flow_rate as usize *
                 (TOTAL_TIME - Self::START_TIME - open_time as usize))
            .sum::<usize>();

        if score > self.best_score {
            print!("{:4}: ", score);

            for actor in 0..N_ACTORS {
                if actor > 0 {
                    print!(":");
                }
                print!("AA");
            }

            for (action, last_pos) in self.stack.iter() {
                print!(",");
                for actor in 0..N_ACTORS {
                    if actor > 0 {
                        print!(":");
                    }
                    match action[actor] {
                        Action::OpenValve => print!("op"),
                        Action::TakeTunnel(tunnel_num) => {
                            let tunnel_num = tunnel_num as usize;
                            let valve = &self.valves[&last_pos[actor]];
                            let next_valve = valve.tunnels[tunnel_num];
                            print!("{}", valve_name(next_valve));
                        },
                        Action::StayStill => print!("**"),
                    }
                }
            }

            println!("");

            self.best_score = score;
        }
    }

    fn have_visited_since_last_open(&self, valve: u16) -> bool {
        for (action, pos) in self.stack.iter().rev() {
            for action in action.iter() {
                if let Action::OpenValve = action {
                    return false;
                }
            }

            for &actor_pos in pos.iter() {
                if actor_pos == valve {
                    return true;
                }
            }
        }

        false
    }

    fn can_take_tunnel(&self,
                       actor: usize,
                       tunnel_num: usize) -> bool {
        let valve = &self.valves[&self.pos[actor]];

        if tunnel_num >= valve.tunnels.len() {
            return false;
        }

        let next_valve = valve.tunnels[tunnel_num];

        !self.have_visited_since_last_open(next_valve)
    }

    fn can_open_valve(&self, actor: usize) -> bool {
        if let Some((action, _)) = self.stack.last() {
            if let Action::OpenValve = action[actor] {
                return false;
            }
        }

        if let Some(_) = self.open_valves.get(&self.pos[actor]) {
            return false;
        }

        let valve = &self.valves[&self.pos[actor]];

        // Don’t bother opening valves that have zero flow rate
        if valve.flow_rate == 0 {
            return false;
        }

        true
    }

    fn can_do_action(&self, all_actions: &[Action; N_ACTORS]) -> bool {
        for (actor, action) in all_actions.iter().enumerate() {
            match action {
                Action::StayStill => {
                    // There’s no point in allowing an actor to stay
                    // still if there’s a tunnel they can go to
                    let n_tunnels = self.valves[&self.pos[actor]].tunnels.len();
                    for i in 0..n_tunnels {
                        if self.can_take_tunnel(actor, i) {
                            return false;
                        }
                    }
                },
                Action::TakeTunnel(t) => {
                    if !self.can_take_tunnel(actor, *t as usize) {
                        return false;
                    }
                },
                Action::OpenValve => {
                    // Make sure that no other actors are also
                    // openening the same valve
                    for (other_actor, action) in all_actions[0..actor]
                        .iter()
                        .enumerate() {
                            if matches!(action, Action::OpenValve) &&
                                self.pos[actor] == self.pos[other_actor] {
                                    return false;
                            }
                    }

                    if !self.can_open_valve(actor) {
                        return false;
                    }
                },
            }
        }

        true
    }

    fn next_action(&self, last_action: &[Action; N_ACTORS])
                   -> Option<[Action; N_ACTORS]> {
        let mut action = *last_action;

        loop {
            'find_action: {
                for i in 0..N_ACTORS {
                    match action[i] {
                        Action::StayStill => {
                            action[i] = Action::OpenValve;
                            break 'find_action;
                        },
                        Action::OpenValve => {
                            action[i] = Action::TakeTunnel(0);
                            break 'find_action;
                        },
                        Action::TakeTunnel(t) => {
                            let valve = &self.valves[&self.pos[i]];

                            if t as usize + 1 >= valve.tunnels.len() {
                                action[i] = Action::StayStill;
                            } else {
                                action[i] = Action::TakeTunnel(t + 1);
                                break 'find_action;
                            }
                        },
                    }
                }

                // If we make it here then we’ve exhausted all the
                // possible actions and we need to backtrack
                return None;
            }

            if self.can_do_action(&action) {
                return Some(action);
            }
        }
    }

    fn take_action(&mut self, action: &[Action; N_ACTORS]) {
        self.stack.push((*action, self.pos));

        for (actor, action) in action.iter().enumerate() {
            match action {
                Action::OpenValve => {
                    self.open_valves.insert(self.pos[actor],
                                            self.stack.len() as u8);
                },
                Action::TakeTunnel(t) => {
                    let valve = &self.valves[&self.pos[actor]];
                    self.pos[actor] = valve.tunnels[*t as usize];
                },
                Action::StayStill => (),
            }
        }
    }

    fn backtrack(&mut self) -> bool {
        loop {
            let (last_action, last_pos) = match self.stack.pop() {
                Some(s) => s,
                None => return false,
            };

            for (actor, action) in last_action.iter().enumerate() {
                match action {
                    Action::OpenValve => {
                        self.open_valves.remove(&self.pos[actor]);
                    },
                    Action::TakeTunnel(_) =>
                        self.pos[actor] = last_pos[actor],
                    Action::StayStill =>
                        (),
                }
            }

            if let Some(action) = self.next_action(&last_action) {
                self.take_action(&action);
                return true;
            }
        }
    }

    fn walk(&mut self) {
        loop {
            self.score_actions();

            if self.stack.len() >= TOTAL_TIME - Self::START_TIME {
                if self.backtrack() {
                    continue;
                } else {
                    break;
                }
            }

            let start_action = [Action::StayStill; N_ACTORS];

            if let Some(action) = self.next_action(&start_action) {
                self.take_action(&action);
            } else if !self.backtrack() {
                break;
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

    Walker::<2>::new(&valves).walk();

    std::process::ExitCode::SUCCESS
}
