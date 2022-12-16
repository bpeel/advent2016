use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u8,
    tunnels: Vec<u8>,
}

struct LoadingValve {
    name: String,
    flow_rate: u8,
    tunnels: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    StayStill,
    OpenValve,
    TakeTunnel(u8),
}

#[derive(Debug)]
struct Walker<'a, const N_ACTORS: usize> {
    stack: Vec<([Action; N_ACTORS], [u8; N_ACTORS])>,
    pos: [u8; N_ACTORS],
    open_valves: HashMap<u8, u8>,
    valves: &'a Vec<Valve>,
    best_score: usize,
}

impl<'a, const N_ACTORS: usize> Walker<'a, N_ACTORS> {
    const START_TIME: usize = (N_ACTORS - 1) * 4;

    fn new(valves: &'a Vec<Valve>) -> Walker<'a, N_ACTORS> {
        Walker {
            stack: Vec::<([Action; N_ACTORS], [u8; N_ACTORS])>::new(),
            pos: [0; N_ACTORS],
            open_valves: HashMap::<u8, u8>::new(),
            valves,
            best_score: usize::MIN,
        }
    }

    fn score_actions(&mut self) {
        let score = self.open_valves
            .iter()
            .map(|(valve, &open_time)|
                 self.valves[*valve as usize].flow_rate as usize *
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
                            let valve = &self.valves[last_pos[actor] as usize];
                            let next_valve = valve.tunnels[tunnel_num];
                            print!("{}", self.valves[next_valve as usize].name);
                        },
                        Action::StayStill => print!("**"),
                    }
                }
            }

            println!("");

            self.best_score = score;
        }
    }

    fn have_visited_since_last_open(&self, valve: u8) -> bool {
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
        let valve = &self.valves[self.pos[actor] as usize];

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

        let valve = &self.valves[self.pos[actor] as usize];

        // Don’t bother opening valves that have zero flow rate
        if valve.flow_rate == 0 {
            return false;
        }

        // Did we skip opening a valve that has a bigger flow rate? If
        // so then there’s no point in opening this one because it
        // would have always been better to open that one first before
        // coming here.
        for (_, pos) in self.stack.iter() {
            if let Some(_) = self.open_valves.get(&pos[actor]) {
                continue;
            }

            let other_valve = &self.valves[pos[actor] as usize];

            if other_valve.flow_rate > valve.flow_rate {
                return false;
            }
        }

        true
    }

    fn can_do_action(&self, all_actions: &[Action; N_ACTORS]) -> bool {
        for (actor, action) in all_actions.iter().enumerate() {
            match action {
                Action::StayStill => {
                    // There’s no point in allowing an actor to stay
                    // still if there’s a tunnel they can go to
                    let valve = &self.valves[self.pos[actor] as usize];
                    let n_tunnels = valve.tunnels.len();
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
                            let valve = &self.valves[self.pos[i] as usize];

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
                    let valve = &self.valves[self.pos[actor] as usize];
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

    fn walk(&mut self) -> usize {
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

        self.best_score
    }
}

const TOTAL_TIME: usize = 30;

fn read_valves<I>(lines: &mut I) -> Result<Vec<Valve>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let re = regex::Regex::new("^Valve ([A-Z]{2}) has flow rate=(\\d+); \
                                tunnels? leads? to valves? ((?:[A-Z]{2}, )*\
                                [A-Z]{2})$").unwrap();
    let mut valves = Vec::<LoadingValve>::new();
    let mut valve_indices = HashMap::<String, usize>::new();

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

        let valve_name = captures[1].to_string();

        if let Some(_) = valve_indices.get(&valve_name) {
            return Err(format!("line {}: duplicate valve", line_num + 1));
        }

        valve_indices.insert(valve_name.clone(), valves.len());

        valves.push(LoadingValve {
            name: valve_name,
            flow_rate,
            tunnels: captures[3]
                .split(", ")
                .map(|s| s.to_string())
                .collect(),
        });
    }

    if valves.len() > u64::BITS as usize {
        return Err("too many valves".to_string());
    }

    // Make sure that AA is valve 0
    match valve_indices.get("AA") {
        None => return Err("missing valve AA".to_string()),
        Some(0) => (),
        Some(&index) => {
            let other_name = &valves[0].name;
            valve_indices.insert(other_name.to_string(), index);
            valve_indices.insert("AA".to_string(), 0);
            let (left, right) = valves.split_at_mut(index);
            std::mem::swap(&mut left[0], &mut right[0]);
        }
    }

    let mut final_valves = Vec::<Valve>::new();

    for valve in valves.iter() {
        let mut final_valve = Valve {
            name: valve.name.clone(),
            flow_rate: valve.flow_rate,
            tunnels: Vec::<u8>::new(),
        };

        for target_valve in valve.tunnels.iter() {
            match valve_indices.get(target_valve) {
                Some(&index) => final_valve.tunnels.push(index as u8),
                None => return Err(format!("valve {} links to non-existant \
                                            valve {}",
                                           valve.name,
                                           target_valve)),
            }
        }

        final_valves.push(final_valve);
    }

    Ok(final_valves)
}

fn main() -> std::process::ExitCode {
    let valves = match read_valves(&mut std::io::stdin().lines()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(valves) => valves,
    };

    let part1 = Walker::<1>::new(&valves).walk();
    let part2 = Walker::<2>::new(&valves).walk();

    println!("part 1: {}\n\
              part 2: {}",
             part1,
             part2);

    std::process::ExitCode::SUCCESS
}
