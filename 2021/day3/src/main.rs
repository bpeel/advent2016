fn main() {
    let nums: Vec<u32> = std::io::stdin()
        .lines()
        .enumerate()
        .map(|(line_num, result)| {
            match result {
                Err(..) => {
                    eprintln!("I/O error");
                    std::process::exit(1);
                },
                Ok(line) => match u32::from_str_radix(&line, 2) {
                    Err(e) => {
                        eprintln!("{}: {}", line_num + 1, e);
                        std::process::exit(1);
                    },
                    Ok(num) => num,
                }
            }
        })
        .collect();

    let mut bit_counts = [0 as u32; u32::BITS as usize];

    for num in nums.iter() {
        let mut bits = *num;

        loop {
            let bit = bits.trailing_zeros();

            if bit >= u32::BITS {
                break;
            }

            bit_counts[bit as usize] += 1;
            bits &= !((1 as u32) << bit);
        }
    }

    let gamma_rate =
        bit_counts.iter().enumerate().fold(0, |bits, (bit_num, &count)| {
            if count as usize > nums.len() / 2 {
                bits | ((1 as u32) << bit_num)
            } else {
                bits
            }
        });

    let leading_zeros =
        bit_counts.iter().rev().take_while(|&&count| count == 0).count();

    let epsilon_rate = (u32::MAX >> leading_zeros) ^ gamma_rate;

    println!("gamma = {}, epsilon = {}",
             gamma_rate,
             epsilon_rate);

    println!("part 1 = {}",
             gamma_rate * epsilon_rate);
}
