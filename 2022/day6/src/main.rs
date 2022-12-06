use std::collections::HashSet;

fn main() {
    let text = std::io::stdin().lines().next().unwrap().unwrap();

    for i in 0..text.len() - 4 {
        let mut hs = HashSet::<u8>::new();

        hs.extend(&text.as_bytes()[i..i+4]);

        if hs.len() == 4 {
            println!("part 1 {}", i + 4);
            break;
        }
    }
}
