use std::collections::HashSet;

fn main() {
    let text = std::io::stdin().lines().next().unwrap().unwrap();

    for i in 0..text.len() - 14 {
        let mut hs = HashSet::<u8>::new();

        hs.extend(&text.as_bytes()[i..i+14]);

        if hs.len() == 14 {
            println!("part 1 {}", i + 14);
            break;
        }
    }
}
