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
    width: usize,
    height: usize,
    values: Vec<Vec<u8>>,
    line: Vec<u8>,
}

impl GridLoader {
    fn new() -> GridLoader {
        GridLoader {
            width: 0,
            height: 0,
            values: Vec::new(),
            line: Vec::new(),
        }
    }

    fn add_byte(&mut self, byte: u8) -> Result<bool, Error> {
        match byte {
            b'\n' => {
                if self.line.len() == 0 {
                    return Ok(true);
                } else if self.line.len() > self.width {
                    self.width = self.line.len();
                }

                self.height += 1;
                self.values.push(self.line.to_owned());
                self.line.clear();
            },
            b => {
                self.line.push(b);
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
                if loader.line.len() != 0 {
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

        let mut values = vec![b' '; loader.width * loader.height];

        for y in 0..loader.height {
            let line = &loader.values[y];
            values[y * loader.width..y * loader.width + line.len()].copy_from_slice(line);
        }

        Ok(Grid {
            width: loader.width,
            height: loader.height,
            values: values.into_boxed_slice(),
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

