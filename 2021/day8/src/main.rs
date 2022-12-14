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

    fn add_pattern(&mut self, pattern: u8) -> bool {
        let mut found_digit = None;

        // If this pattern can only be one of the digits…
        for digit in 0..DIGIT_PATTERNS.len() {
            if self.could_be_digit(digit, pattern) {
                if found_digit != None {
                    return false;
                }

                found_digit = Some(digit);
            }
        }

        let mut changed_something = false;

        if let Some(digit) = found_digit {
            for bit in 0..N_SEGMENTS {
                let new_bits = if pattern & (1u8 << bit) != 0 {
                    // The bits that are in the input pattern can only
                    // map to the bits in the output pattern
                    self.bits[bit] & DIGIT_PATTERNS[digit]
                } else {
                    // The bits that aren’t in the input pattern can’t
                    // map to the bits in the output pattern
                    self.bits[bit] & !DIGIT_PATTERNS[digit]
                };

                if new_bits == self.bits[bit] {
                    continue;
                }

                self.bits[bit] = new_bits;
                changed_something = true;
            }
        }

        changed_something
    }

    fn is_complete(&self) -> bool {
        for output_wires in self.bits.iter() {
            if output_wires.count_ones() != 1 {
                return false;
            }
        }

        true
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

fn main() -> std::process::ExitCode {
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

        loop {
            let mut progress = false;

            for pattern in display.patterns.iter() {
                progress |= mapping.add_pattern(*pattern);
            }

            if !progress {
                break;
            }
        }

        if !mapping.is_complete() {
            eprintln!("line {}: couldn’t solve mapping", line_num + 1);
            return std::process::ExitCode::FAILURE;
        }
    }

    std::process::ExitCode::SUCCESS
}
