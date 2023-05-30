use std::str::FromStr;
use std::collections::HashSet;
use std::io;
use std::process::ExitCode;

const IMAGE_KEY_BITS: usize = 512;
const IMAGE_KEY_INTS: usize = IMAGE_KEY_BITS / (u64::BITS as usize);

struct ImageKey {
    bits: [u64; IMAGE_KEY_INTS],
}

impl FromStr for ImageKey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<ImageKey, &'static str> {
        let mut key = ImageKey { bits: [0; IMAGE_KEY_INTS] };
        let mut i = 0;

        for ch in s.chars() {
            if i >= IMAGE_KEY_BITS {
                return Err("Too many bits in image key");
            }

            match ch {
                '#' => {
                    key.bits[i / (u64::BITS as usize)] |=
                        1u64 << (i as u32 % u64::BITS);
                },
                '.' => (),
                _ => return Err("Invalid character in image key"),
            }

            i += 1;
        }

        if i != IMAGE_KEY_BITS {
            Err("Not enough bits in image key")
        } else {
            Ok(key)
        }
    }
}

impl ImageKey {
    fn get(&self, index: usize) -> bool {
        self.bits[index / u64::BITS as usize]
            & (1u64 << (index as u32 % u64::BITS))
            != 0
    }
}

struct Image {
    pixels: HashSet<(i32, i32)>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    // The value returned for pixels outside of the range
    other_pixels: bool,
}

impl Image {
    fn new() -> Image {
        Image {
            pixels: HashSet::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            other_pixels: false,
        }
    }

    fn get(&self, x: i32, y: i32) -> bool {
        if x >= self.min_x
            && x <= self.max_x
            && y >= self.min_y
            && y <= self.max_y
        {
            self.pixels.contains(&(x, y))
        } else {
            self.other_pixels
        }
    }

    fn add_pixel(&mut self, x: i32, y: i32) {
        let was_empty = self.pixels.is_empty();

        if self.pixels.insert((x, y)) {
            if was_empty {
                self.min_x = x;
                self.max_x = x;
                self.min_y = y;
                self.max_y = y;
            } else {
                if x < self.min_x {
                    self.min_x = x;
                }
                if x > self.max_x {
                    self.max_x = x;
                }
                if y < self.min_y {
                    self.min_y = y;
                }
                if y > self.max_y {
                    self.max_y = y;
                }
            }
        }
    }

    fn neighbour_key(&self, x: i32, y: i32) -> usize {
        let mut key = 0;

        for y_off in -1..=1 {
            for x_off in -1..=1 {
                let bit = self.get(x + x_off, y + y_off) as usize;

                key = (key << 1) | bit;
            }
        }

        key
    }

    fn enhance(&self, key: &ImageKey) -> Image {
        let mut result = Image::new();

        for y in self.min_y - 1..=self.max_y + 1 {
            for x in self.min_x - 1..=self.max_x + 1 {
                let neighbour_key = self.neighbour_key(x, y);

                if key.get(neighbour_key) {
                    result.add_pixel(x, y);
                }
            }
        }

        let other_key = if self.other_pixels {
            0b111111111
        } else {
            0b000000000
        };

        result.other_pixels = key.get(other_key);

        result
    }
}

fn parse_input<T: Iterator<Item = io::Result<String>>>(
    lines: T,
) -> Result<(ImageKey, Image), String> {
    let mut key: Option<ImageKey> = None;
    let mut image = Image::new();

    for (line_num, line) in lines.enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(e) => return Err(e.to_string()),
        };

        if line_num == 0 {
            key = Some(line.parse()?);
        } else {
            for (i, ch) in line.chars().enumerate() {
                match ch {
                    '#' => image.add_pixel(i as i32, line_num as i32),
                    '.' => (),
                    _ => {
                        return Err(format!(
                            "line {}: invalid character",
                            line_num + 1,
                        ));
                    },
                }
            }
        }
    }

    match key {
        None => Err("No image key".to_string()),
        Some(key) => Ok((key, image)),
    }
}

fn main() -> ExitCode {
    let (key, mut image) = match parse_input(io::stdin().lines()) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    for i in 0..50 {
        image = image.enhance(&key);

        if i == 2 {
            println!("part 1: {}", image.pixels.len());
        }
    }

    println!("part 2: {}", image.pixels.len());

    ExitCode::SUCCESS
}
