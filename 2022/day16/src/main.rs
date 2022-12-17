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

#[derive(Debug)]
struct Walker<'a> {
    stack: Vec<(u8, u8)>,
    valves: &'a [Valve],
    distances: &'a mut [u8],
    pos: u8,
}

impl<'a> Walker<'a> {
    fn new(valves: &'a [Valve], distances: &'a mut [u8]) -> Self {
        Walker {
            stack: Vec::<(u8, u8)>::new(),
            valves,
            distances,
            pos: 0,
        }
    }

    fn backtrack(&mut self) -> bool {
        loop {
            match self.stack.pop() {
                Some((pos, last_tunnel)) => {
                    let next_tunnel = last_tunnel as usize + 1;
                    let valve = &self.valves[pos as usize];

                    if let Some(&next_pos) = valve.tunnels.get(next_tunnel) {
                        self.stack.push((pos, next_tunnel as u8));
                        self.pos = next_pos;
                        return true;
                    }
                },
                None => return false,
            }
        }
    }

    fn walk(&mut self, pos: u8) {
        self.pos = pos;

        loop {
            // Have we already been here with a shorter distance?
            if self.distances[self.pos as usize] as usize <= self.stack.len() {
                if self.backtrack() {
                    continue;
                } else {
                    break;
                }
            }

            self.distances[self.pos as usize] = self.stack.len() as u8;

            let valve = &self.valves[self.pos as usize];

            if valve.tunnels.is_empty() {
                if self.backtrack() {
                    continue;
                } else {
                    break;
                }
            } else {
                self.stack.push((self.pos, 0));
                self.pos = valve.tunnels[0];
            }
        }
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

fn calculate_distances(valves: &[Valve]) -> Vec<u8> {
    let mut distances = vec![u8::MAX; valves.len() * valves.len()];

    for valve_num in 0..valves.len() {
        let this_valve_distances =
            &mut distances[valve_num * valves.len()..
                           (valve_num + 1) * valves.len()];
        Walker::new(valves, this_valve_distances).walk(valve_num as u8);
    }

    distances
}

fn get_valves_with_flow(valves: &[Valve]) -> Vec<u8> {
    (0..valves.len())
        .filter(|&i| valves[i].flow_rate > 0)
        .map(|i| i as u8)
        .collect()
}

fn permute<F>(valves: &mut [u8], mut func: F)
    where F: FnMut(&[u8]) -> usize
{
    let mut stack = Vec::<u8>::new();

    'outer: loop {
        if stack.len() >= valves.len() {
            let cut_point = func(&valves);

            stack.truncate(cut_point);

            loop {
                match stack.pop() {
                    Some(n) => {
                        // Put the element back
                        if n > 0 {
                            valves.swap(stack.len(), stack.len() + n as usize);
                        }
                        if (n as usize) + 1 < valves.len() - stack.len() {
                            valves.swap(stack.len(),
                                        stack.len() + n as usize + 1);
                            stack.push(n + 1);
                            break;
                        }
                    },
                    None => break 'outer,
                }
            }
        } else {
            stack.push(0);
        }
    }
}

fn score_valves(valves: &[Valve],
                distances: &[u8],
                valve_order: &[u8],
                total_time: usize) -> (usize, usize) {
    let mut score = 0;
    let mut pos = 0;
    let mut time = 0;
    let mut cut_point = valve_order.len();

    for (i, &valve_num) in valve_order.iter().enumerate() {
        time += distances[pos * valves.len() +
                          valve_num as usize] as usize + 1;

        if time > total_time {
            cut_point = i + 1;
            break;
        }

        score += (total_time - time) *
            (valves[valve_num as usize].flow_rate as usize);

        pos = valve_num as usize;
    }

    (score, cut_point)
}

fn subsets<F>(all_values: &[u8],
              mut func: F)
    where F: FnMut(&mut [u8], &mut [u8])
{
    let mut subset = Vec::<u8>::new();
    let mut rest = Vec::<u8>::new();

    for set_mask in 0..(1u64 << all_values.len()) {
        subset.clear();
        rest.clear();

        for (i, &value) in all_values.iter().enumerate() {
            if set_mask & (1u64 << i) == 0 {
                rest.push(value);
            } else {
                subset.push(value);
            }
        }

        func(&mut subset, &mut rest);
    }
}

fn best_score_for_valves(valves: &[Valve],
                         distances: &[u8],
                         valves_with_flow: &mut [u8],
                         total_time: usize) -> usize
{
    let mut best_score = usize::MIN;

    permute(valves_with_flow, |valve_order| {
        let (score, cut_point) = score_valves(valves,
                                              distances,
                                              valve_order,
                                              total_time);

        if score > best_score {
            best_score = score;
        }

        cut_point
    });

    best_score
}

fn part1(valves: &[Valve],
         distances: &[u8],
         valves_with_flow: &mut [u8]) -> usize
{
    best_score_for_valves(valves, distances, valves_with_flow, TOTAL_TIME)
}

fn part2(valves: &[Valve],
         distances: &[u8],
         valves_with_flow: &mut [u8]) -> usize
{
    let mut best_score = usize::MIN;

    subsets(valves_with_flow, |you_valves, elephant_valves| {
        let score =
            best_score_for_valves(valves,
                                  distances,
                                  you_valves,
                                  TOTAL_TIME - 4) +
            best_score_for_valves(valves,
                                  distances,
                                  elephant_valves,
                                  TOTAL_TIME - 4);

        if score > best_score {
            best_score = score;
        }
    });

    best_score
}

fn main() -> std::process::ExitCode {
    let valves = match read_valves(&mut std::io::stdin().lines()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(valves) => valves,
    };

    let distances = calculate_distances(&valves);

    let mut valves_with_flow = get_valves_with_flow(&valves);

    println!("part 1: {}", part1(&valves,
                                 &distances,
                                 &mut valves_with_flow));
    println!("part 2: {}", part2(&valves,
                                 &distances,
                                 &mut valves_with_flow));

    std::process::ExitCode::SUCCESS
}
