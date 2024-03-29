use std::io::Read;

const N_MATERIALS: usize = 4;
const N_ROBOTS: usize = N_MATERIALS;

#[derive(Debug, Clone, Copy)]
struct Costs {
    material: [u8; N_MATERIALS],
}

#[derive(Debug, Clone)]
struct Blueprint {
    costs: [Costs; N_ROBOTS],
}

#[derive(Debug, Clone)]
struct StackEntry {
    n_robots: [usize; N_ROBOTS],
    n_materials: [usize; N_MATERIALS],
    robot_created: Option<u8>,
}

fn read_blueprints<R>(input: &mut R) -> Result<Vec<Blueprint>, String>
    where R: Read
{
    let input = match std::io::read_to_string(input) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s,
    };

    let re = regex::Regex::new("Blueprint (\\d+):\\s+\
                                Each ore robot costs (\\d+) ore\\.\\s+\
                                Each clay robot costs (\\d+) ore\\.\\s+\
                                Each obsidian robot costs (\\d+) ore and \
                                (\\d+) clay\\.\\s+\
                                Each geode robot costs (\\d+) ore and \
                                (\\d+) obsidian.").unwrap();

    let mut blueprints = Vec::<Blueprint>::new();

    for captures in re.captures_iter(&input) {
        let mut ints = [0u8; 7];

        for (i, vp) in ints.iter_mut().enumerate() {
            match captures[i + 1].parse() {
                Ok(v) => *vp = v,
                Err(e) => return Err(e.to_string()),
            }
        }

        if ints[0] as usize != blueprints.len() + 1 {
            return Err(format!("blueprint {} is out of order", ints[0]));
        }

        blueprints.push(Blueprint {
            costs: [
                Costs { material: [ ints[1], 0, 0, 0 ] },
                Costs { material: [ ints[2], 0, 0, 0 ] },
                Costs { material: [ ints[3], ints[4], 0, 0 ] },
                Costs { material: [ ints[5], 0, ints[6], 0 ] },
            ],
        });
    }

    if blueprints.is_empty() {
        return Err("no blueprints found in input".to_string());
    }

    Ok(blueprints)
}

fn try_make_robot(blueprint: &Blueprint,
                  n_materials: &mut [usize],
                  robot_type: u8) -> bool {
    let costs = blueprint.costs[robot_type as usize];

    for (material_num, &count) in costs.material.iter().enumerate() {
        if (count as usize) > n_materials[material_num] {
            return false;
        }
    }

    for (material_num, &count) in costs.material.iter().enumerate() {
        n_materials[material_num] -= count as usize;
    }

    true
}

fn robot_is_pointless(blueprint: &Blueprint,
                      n_robots: &[usize],
                      robot_type: u8) -> bool {
    let robot_index = robot_type as usize;

    // It’s never pointless to build the geode robot
    if robot_index >= N_ROBOTS - 1 {
        return false;
    }

    // It is pointless to make this robot if we already have enough
    // robots to build whatever robot needs its resources at every
    // turn
    for costs in blueprint.costs.iter() {
        if costs.material[robot_index] as usize > n_robots[robot_index] {
            return false;
        }
    }

    true
}

fn apply_next_robot(blueprint: &Blueprint,
                    stack: &mut Vec<StackEntry>,
                    start_robot: u8) {
    let mut top = stack.last().unwrap().clone();

    'find_robot_type: {
        for robot_type in start_robot..(N_ROBOTS as u8) {
            if robot_is_pointless(blueprint,
                                  &top.n_robots,
                                  robot_type) {
                continue;
            }

            if try_make_robot(blueprint,
                              &mut top.n_materials,
                              robot_type) {
                top.robot_created = Some(robot_type);
                break 'find_robot_type;
            }
        }

        top.robot_created = None;
    }

    for material in 0..N_MATERIALS {
        top.n_materials[material] += top.n_robots[material];
    }

    if let Some(r) = top.robot_created {
        top.n_robots[r as usize] += 1;
    }

    stack.push(top);
}

fn backtrack(blueprint: &Blueprint,
             stack: &mut Vec<StackEntry>) -> bool {
    loop {
        let top = match stack.pop() {
            Some(t) => t,
            None => return false,
        };

        let robot_created = match top.robot_created {
            Some(r) => r,
            None => continue,
        };

        if stack.is_empty() {
            return false;
        }

        apply_next_robot(blueprint, stack, robot_created + 1);
        return true;
    }
}

fn calc_max_geodes(state: &StackEntry, minutes_remaining: usize) -> usize {
    state.n_materials[N_MATERIALS - 1] +
        (0..minutes_remaining).map(|minute| {
            // At each minute, assume we produced a geode robot the last minute
            // and add all of the geodes produced by the robots
            state.n_robots[N_ROBOTS - 1] + minute
        }).sum::<usize>()
}

fn try_blueprint(blueprint: &Blueprint,
                 n_minutes: usize) -> usize {
    let mut stack = vec![StackEntry {
        n_robots: [0; N_ROBOTS],
        n_materials: [0; N_MATERIALS],
        robot_created: None,
    }; 1];
    let mut best_score = 0;

    // Start with an ore robot
    stack[0].n_robots[0] = 1;

    loop {
        if stack.len() >= n_minutes + 1 {
            let n_geodes = stack.last().unwrap().n_materials[N_MATERIALS - 1];

            if n_geodes > best_score {
                best_score = n_geodes;
            }

            if !backtrack(blueprint, &mut stack) {
                break;
            } else {
                continue;
            }
        }

        // Don’t continue if the maximum number of geodes we could
        // build from here on is worse than the best solution
        let max_geodes = calc_max_geodes(stack.last().unwrap(),
                                         n_minutes + 1 - stack.len());
        if max_geodes <= best_score {
            if !backtrack(blueprint, &mut stack) {
                break;
            } else {
                continue;
            }
        }

        apply_next_robot(blueprint, &mut stack, 0);
    }

    best_score
}

fn main() -> std::process::ExitCode {
    let blueprints = match read_blueprints(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(b) => b,
    };

    let part1 = blueprints
        .iter()
        .enumerate()
        .map(|(num, bp)| {
            let best_score = try_blueprint(bp, 24);
            let result = (num + 1) * best_score;
            println!("{}/{}: {} * {} = {}",
                     num + 1,
                     blueprints.len(),
                     num + 1,
                     best_score,
                     result);
            result
        })
        .sum::<usize>();

    println!("part 1: {}", part1);

    let max_blueprints = std::cmp::min(3, blueprints.len());

    let part2 = blueprints[0..max_blueprints]
        .iter()
        .enumerate()
        .map(|(num, bp)| {
            let best_score = try_blueprint(bp, 32);
            println!("{}/{}: {}",
                     num + 1,
                     max_blueprints,
                     best_score);
            best_score
        })
        .product::<usize>();

    println!("part 2: {}", part2);

    std::process::ExitCode::SUCCESS
}
