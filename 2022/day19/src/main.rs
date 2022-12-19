use std::io::Read;

const N_MATERIALS: usize = 3;
const N_ROBOTS: usize = N_MATERIALS + 1;
const N_MINUTES: usize = 24;

#[derive(Debug, Clone, Copy)]
struct Costs {
    material: [u8; N_MATERIALS],
}

#[derive(Debug, Clone)]
struct Blueprint {
    costs: [Costs; N_ROBOTS],
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
                Costs { material: [ ints[1], 0, 0 ] },
                Costs { material: [ ints[2], 0, 0 ] },
                Costs { material: [ ints[3], ints[4], 0 ] },
                Costs { material: [ ints[5], 0, ints[6] ] },
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

fn try_robot_combination(blueprint: &Blueprint,
                         robot_for_minute: &[u8]) -> usize {
    let mut n_robots = [0usize; N_ROBOTS];
    let mut n_materials = [0usize; N_MATERIALS];
    let mut n_geodes = 0usize;

    // We start with a clay robot
    n_robots[0] = 1;

    for &new_robot_type in robot_for_minute.iter() {
        let can_make_robot = try_make_robot(blueprint,
                                            &mut n_materials,
                                            new_robot_type);

        for material in 0..N_MATERIALS {
            n_materials[material] += n_robots[material];
        }

        n_geodes += *n_robots.last().unwrap() as usize;

        if can_make_robot {
            n_robots[new_robot_type as usize] += 1;
        }
    }

    n_geodes
}

fn try_blueprint(blueprint: &Blueprint) -> usize {
    let mut robot_for_minute = [0u8; N_MINUTES];
    let mut best_score = 0usize;

    'outer_loop: loop {
        let score = try_robot_combination(blueprint, &robot_for_minute);

        if score > best_score {
            best_score = score;
        }

        // Get the next robot_for_minute combination
        for robot in robot_for_minute.iter_mut() {
            if *robot as usize + 1 < N_ROBOTS {
                *robot += 1;
                continue 'outer_loop;
            }

            *robot = 0;
        }

        // If we make it here then we’ve exhausted all of the combinations
        break;
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

    let part1 = blueprints.iter().map(|bp| try_blueprint(bp)).max().unwrap();

    println!("part 1: {}", part1);

    std::process::ExitCode::SUCCESS
}
