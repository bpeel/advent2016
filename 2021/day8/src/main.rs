const PAT_A: u8 = 1u8 << 0;
const PAT_B: u8 = 1u8 << 1;
const PAT_C: u8 = 1u8 << 2;
const PAT_D: u8 = 1u8 << 3;
const PAT_E: u8 = 1u8 << 4;
const PAT_F: u8 = 1u8 << 5;
const PAT_G: u8 = 1u8 << 6;
const N_SEGMENTS: usize = 7;

static DIGIT_PATTERNS: [u8; 10] = [
    PAT_A | PAT_B | PAT_C | PAT_E | PAT_F | PAT_G,
    PAT_C | PAT_F,
    PAT_A | PAT_C | PAT_D | PAT_E | PAT_G,
    PAT_A | PAT_C | PAT_D | PAT_F | PAT_G,
    PAT_B | PAT_C | PAT_D | PAT_F,
    PAT_A | PAT_B | PAT_D | PAT_F | PAT_G,
    PAT_A | PAT_B | PAT_D | PAT_E | PAT_F | PAT_G,
    PAT_A | PAT_C | PAT_F,
    PAT_A | PAT_B | PAT_C | PAT_D | PAT_E | PAT_F | PAT_G,
    PAT_A | PAT_B | PAT_C | PAT_D | PAT_F | PAT_G,
];

#[derive(Copy, Clone, Debug)]
struct BitIter {
    bits: u8,
}

impl BitIter {
    fn new(bits: u8) -> BitIter {
        BitIter { bits }
    }
}

impl Iterator for BitIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let bit_num = self.bits.trailing_zeros();

        if bit_num >= u8::BITS {
            None
        } else {
            self.bits &= !(1u8 << bit_num);
            Some(bit_num as usize)
        }
    }
}

struct WireMapping {
    // Array indexed by input wire number with bitmask of potential
    // output wire numbers
    bits: [u8; N_SEGMENTS],
}

impl WireMapping {
    fn new() -> WireMapping {
        WireMapping {
            bits: [u8::MAX >> (u8::BITS as usize - N_SEGMENTS); N_SEGMENTS]
        }
    }

    fn could_be_digit(&self, digit: usize, input_wires: u8) -> bool {
        if DIGIT_PATTERNS[digit].count_ones() != input_wires.count_ones() {
            return false;
        }

        // Check that each bit is a possible output bit for the mapping
        'output_bit: for output_bit in BitIter::new(DIGIT_PATTERNS[digit]) {
            let output_mask = 1u8 << output_bit;

            for input_bit in BitIter::new(input_wires) {
                if self.bits[input_bit] & output_mask != 0 {
                    continue 'output_bit;
                }
            }

            // If we make it here we didn’t find a corresponding input bit
            return false;
        }

        true
    }

    fn set_mapping(&mut self, input_bit: usize, output_bit: usize) {
        for bit in 0..N_SEGMENTS {
            if bit == input_bit {
                // The input bit now only has a single mapping
                self.bits[bit] &= 1u8 << output_bit
            } else {
                // This input can not be mapped to the output
                self.bits[bit] &= !(1u8 << output_bit)
            };
        }
    }

    fn add_pattern(&mut self, pattern: u8) {
        let mut found_digit = None;

        // If this pattern can only be one of the digits…
        for digit in 0..DIGIT_PATTERNS.len() {
            if self.could_be_digit(digit, pattern) {
                if found_digit != None {
                    return;
                }

                found_digit = Some(digit);
            }
        }

        if let Some(digit) = found_digit {
            for bit in 0..N_SEGMENTS {
                if pattern & (1u8 << bit) != 0 {
                    // The bits that are in the input pattern can only
                    // map to the bits in the output pattern
                    self.bits[bit] &= DIGIT_PATTERNS[digit]
                } else {
                    // The bits that aren’t in the input pattern can’t
                    // map to the bits in the output pattern
                    self.bits[bit] &= !DIGIT_PATTERNS[digit]
                };
            }
        }
    }

    fn is_complete(&self) -> bool {
        for output_wires in self.bits.iter() {
            if output_wires.count_ones() != 1 {
                return false;
            }
        }

        true
    }

    fn final_mapping(&self) -> [u8; N_SEGMENTS] {
        let mut mapping = [0u8; N_SEGMENTS];

        for bit in 0..N_SEGMENTS {
            mapping[bit] = self.bits[bit].trailing_zeros() as u8;
        }

        mapping
    }
}

#[derive(Clone, Debug)]
struct Display {
    patterns: [u8; 10],
    digits: [u8; 4],
}

fn parse_bit_string(s: &str) -> Result<u8, String> {
    let mut val = 0u8;

    for ch in s.chars() {
        match ch {
            'a'..='g' => val |= 1u8 << (ch as u8 - b'a'),
            _ => return Err("invalid character in pattern".to_string()),
        }
    }

    Ok(val)
}

fn parse_patterns(s: &str) -> Result<[u8; 10], String> {
    let mut patterns = [0u8; 10];
    let mut pattern_num = 0;

    for pattern in s.split_whitespace() {
        if pattern_num >= patterns.len() {
            return Err("too many patterns".to_string());
        }

        patterns[pattern_num] = parse_bit_string(pattern)?;

        pattern_num += 1;
    }

    if pattern_num != patterns.len() {
        return Err("not enough patterns".to_string());
    }

    Ok(patterns)
}

fn parse_digits(s: &str) -> Result<[u8; 4], String> {
    let mut digits = [0u8; 4];
    let mut digit_num = 0;

    for digit in s.split_whitespace() {
        if digit_num >= digits.len() {
            return Err("too many digits".to_string());
        }

        digits[digit_num] = parse_bit_string(digit)?;

        digit_num += 1;
    }

    if digit_num != digits.len() {
        return Err("not enough digits".to_string());
    }

    Ok(digits)
}

impl std::str::FromStr for Display {
    type Err = String;

    fn from_str(s: &str) -> Result<Display, String> {
        let mut parts = s.split(" | ");

        let patterns_str = match parts.next() {
            None => return Err("missing ‘ | ’ separator".to_string()),
            Some(p) => p,
        };

        let patterns = parse_patterns(patterns_str)?;

        let digits_str = match parts.next() {
            None => return Err("missing ‘ | ’ separator".to_string()),
            Some(d) => d,
        };

        if parts.next() != None {
            return Err("too many parts".to_string());
        }

        Ok(Display { patterns, digits: parse_digits(digits_str)? })
    }
}

fn set_mapping_for_bit_counts(mapping: &mut WireMapping, patterns: &[u8]) {
    // Count the number of times each bit appears
    let mut bit_counts = [0u8; N_SEGMENTS];

    for pattern in patterns.iter() {
        for bit in BitIter::new(*pattern) {
            bit_counts[bit] += 1;
        }
    }

    // a 8
    // b 6
    // c 8
    // d 7
    // e 4
    // f 9
    // g 7

    for (input_bit, count) in bit_counts.iter().enumerate() {
        match count {
            // Only segment B appears exactly 6 times
            6 => mapping.set_mapping(input_bit, 1),
            // Only segment E appears exactly 4 times
            4 => mapping.set_mapping(input_bit, 4),
            // Only segment F appears exactly 9 times
            9 => mapping.set_mapping(input_bit, 5),
            _ => (),
        };
    }
}

fn map_digits(digits: &[u8], mapping: &[u8]) -> Result<u32, String> {
    let mut val = 0u32;

    for digit in digits.iter() {
        let mut bits = 0u8;

        for bit in BitIter::new(*digit) {
            bits |= 1u8 << mapping[bit];
        }

        let digit = match DIGIT_PATTERNS.iter().position(|&b| b == bits) {
            Some(d) => d,
            None => return Err(format!("unknown bit pattern {:x}", bits)),
        };

        val = (val * 10) + digit as u32;
    }

    Ok(val)
}

fn main() -> std::process::ExitCode {
    let mut part2 = 0;

    for (line_num, result) in std::io::stdin().lines().enumerate() {
        let line = match result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
        };

        let display = match line.parse::<Display>() {
            Err(s) => {
                eprintln!("line {}: {}", line_num + 1, s);
                return std::process::ExitCode::FAILURE;
            },
            Ok(d) => d,
        };

        let mut mapping = WireMapping::new();

        set_mapping_for_bit_counts(&mut mapping, &display.patterns);

        for pattern in display.patterns.iter() {
            mapping.add_pattern(*pattern);
        }

        if !mapping.is_complete() {
            eprintln!("line {}: couldn’t solve mapping", line_num + 1);
            return std::process::ExitCode::FAILURE;
        }

        let mapping = mapping.final_mapping();

        let display_num = match map_digits(&display.digits, &mapping) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("line {}: {}", line_num + 1, e);
                return std::process::ExitCode::FAILURE;
            }
        };

        part2 += display_num;
    }

    println!("part 2: {}", part2);

    std::process::ExitCode::SUCCESS
}
