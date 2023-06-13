#[derive(Debug)]
struct Bus {
    id: u64,
    target_minute: u64,
}

fn read_timetable() -> Vec<Bus> {
    let mut lines = std::io::stdin().lines();

    lines.next().unwrap().unwrap();

    lines.next()
        .unwrap()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(target_minute, id)| {
            if id == "x" {
                None
            } else {
                let id = id.parse().unwrap();
                let target_minute = ((id - 1) * target_minute as u64) % id;

                Some(Bus { id, target_minute })
            }
        })
        .collect()
}

fn main() {
    let timetable = read_timetable();

    let mut multiplier = 1;
    let mut t = 0;

    for bus in timetable.iter() {
        println!("{:?}", bus);
        while t % bus.id != bus.target_minute {
            t += multiplier;
        }

        multiplier *= bus.id;
    }

    println!("part 2: {}", t);
}
