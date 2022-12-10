fn main() {
    let mut count = 0;

    for result in std::io::stdin().lines() {
        let line = result.unwrap();
        let displays = line.split(" | ").nth(1).unwrap();
        let numbers = displays.split(' ');

        for number in numbers {
            match number.len() {
                2 | 4 | 3 | 7 => count += 1,
                _ => (),
            }
        }
    }

    println!("part 1: {}", count);
}
