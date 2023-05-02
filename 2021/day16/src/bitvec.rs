use std::fmt;

#[derive(Debug)]
pub struct BitVec {
    bits: Vec<u8>,
    n_bits: usize,
}

#[derive(Debug)]
pub struct Error {
    ch: char,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected character {} in BitVec data", self.ch)
    }
}

impl BitVec {
    pub fn new(data: &str) -> Result<BitVec, Error> {
        let mut bits = Vec::<u8>::new();
        let mut n_bits = 0;

        for ch in data.chars() {
            match ch.to_digit(16) {
                None => return Err(Error { ch }),
                Some(digit) => {
                    let nibble = (digit >> 3)
                        | ((digit >> 1) & 2)
                        | ((digit << 1) & 4)
                        | ((digit << 3) & 8);

                    if n_bits & 4 == 0 {
                        bits.push((nibble) as u8);
                    } else {
                        *bits.last_mut().unwrap() |= (nibble << 4) as u8;
                    }
                },
            }

            n_bits += 4;
        }

        Ok(BitVec { bits, n_bits })
    }

    pub fn read_bit(&self, pos: usize) -> bool {
        assert!(pos < self.n_bits);
        self.bits[pos / 8] & (1 << (pos & 7)) != 0
    }

    pub fn read_bits(&self, mut pos: usize, n_bits: usize) -> u64 {
        let mut result = 0;

        for _ in 0..n_bits {
            result <<= 1;

            if self.read_bit(pos) {
                result |= 1;
            }

            pos += 1;
        }

        result
    }

    pub fn size(&self) -> usize {
        self.n_bits
    }

    pub fn is_trailing_zeroes(&self, start_pos: usize) -> bool {
        if start_pos >= self.n_bits {
            return true;
        }

        let mut byte_pos = start_pos / 8;
        let bit_pos = start_pos % 8;

        if bit_pos > 0 {
            if self.bits[byte_pos] & (0xff << bit_pos) != 0 {
                return false;
            }

            byte_pos += 1;
        }

        for &byte in &self.bits[byte_pos..] {
            if byte != 0 {
                return false;
            }
        }

        true
    }
}
