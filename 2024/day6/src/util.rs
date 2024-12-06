use std::io::{Error, ErrorKind};
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub values: Box<[u8]>,
}

#[derive(Debug, Clone)]
struct GridLoader {
    x: usize,
    width: usize,
    height: usize,
    values: Vec<u8>,
}

impl GridLoader {
    fn new() -> GridLoader {
        GridLoader {x: 0, width: 0, height: 0, values: Vec::new() }
    }

    fn add_byte(&mut self, byte: u8) -> Result<bool, Error> {
        match byte {
            b'\n' => {
                if self.x == 0 {
                    return Ok(true);
                } else if self.height == 0 {
                    self.width = self.x;
                } else if self.width != self.x {
                    return Err(Error::new(ErrorKind::Other,
                                          format!("line {} has different \
                                                   width from first line",
                                                  self.height + 1)));
                }

                self.x = 0;
                self.height += 1;
            },
            b => {
                self.values.push(b);
                self.x += 1;
            },
        };

        Ok(false)
    }
}

impl Grid {
    pub fn load<T>(input: &mut T) -> Result<Grid, Error>
        where T: std::io::BufRead
    {
        let mut loader = GridLoader::new();

        'read_loop: loop {
            let bytes = input.fill_buf()?;

            if bytes.len() == 0 {
                if loader.x != 0 {
                    return Err(Error::new(ErrorKind::Other,
                                          "last line does not end with a \
                                           newline character"));
                }

                break;
            }

            for (byte_num, byte) in bytes.iter().enumerate() {
                if loader.add_byte(*byte)? {
                    input.consume(byte_num + 1);
                    break 'read_loop;
                }
            }

            let len = bytes.len();
            input.consume(len);
        }

        Ok(Grid {
            width: loader.width,
            height: loader.height,
            values: Box::from(loader.values)
        })
    }

    pub fn get(&self, (xp, yp): (i32, i32)) -> Option<u8> {
        if xp < 0 || yp < 0 {
            return None;
        }

        let x = xp as usize;
        let y = yp as usize;

        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.values[y * self.width + x])
    }

    pub fn get_mut(&mut self, (xp, yp): (i32, i32)) -> Option<&mut u8> {
        if xp < 0 || yp < 0 {
            return None;
        }

        let x = xp as usize;
        let y = yp as usize;

        if x >= self.width || y >= self.height {
            return None;
        }

        Some(&mut self.values[y * self.width + x])
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
           -> Result<(), std::fmt::Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                let b = self.values[y * self.width + x];

                let c = if b >= b' ' && b < 127u8 {
                    b as char
                } else {
                    '□'
                };

                f.write_char(c)?;
            }

            if y < self.height - 1 {
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}
