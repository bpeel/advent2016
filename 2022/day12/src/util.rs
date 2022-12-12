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

    pub fn get(&self, (x, y): (i32, i32)) -> u8 {
        assert!(x >= 0 && (x as usize) < self.width);
        assert!(y >= 0 && (y as usize) < self.height);

        self.values[y as usize * self.width + x as usize]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let mut test_input: &[u8] = b"ab\ncd\n";
        let grid = Grid::load(&mut test_input).unwrap();
        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.values.as_ref(), b"abcd");
        assert_eq!(grid.to_string(), "ab\ncd");

        // Non square
        let mut test_input: &[u8] = b"abcdefg\n";
        let grid = Grid::load(&mut test_input).unwrap();
        assert_eq!(grid.width, 7);
        assert_eq!(grid.height, 1);
        let mut test_input: &[u8] = b"abcdefg\nabcdefg\n";
        let grid = Grid::load(&mut test_input).unwrap();
        assert_eq!(grid.width, 7);
        assert_eq!(grid.height, 2);

        // Empty
        let mut test_input: &[u8] = b"";
        let grid = Grid::load(&mut test_input).unwrap();
        assert_eq!(grid.width, 0);
        assert_eq!(grid.height, 0);
        assert_eq!(grid.to_string(), "");
        assert_eq!(grid.values.len(), 0);

        // Non-displayable ASCII characters and bytes >= 127
        let mut test_input: &[u8] = b"\x00\x01\x02\xff\n\x7e\x7f \x1f\n";
        let grid = Grid::load(&mut test_input).unwrap();
        assert_eq!(grid.to_string(), "□□□□\n~□ □");

        // Parsing stops after blank line
        let mut test_input: &[u8] = b"abc\ndef\n\npotato";
        let grid = Grid::load(&mut test_input).unwrap();
        assert_eq!(grid.values.as_ref(), b"abcdef");
        assert_eq!(test_input, b"potato");

        fn check_error(test_input: &[u8], message: &str) {
            let mut test_input_mut: &[u8] = test_input;
            match Grid::load(&mut test_input_mut) {
                Ok(_) => {
                    panic!("expected error while loading “{:?}”", test_input);
                },
                Err(e) => assert_eq!(e.to_string(), message),
            }
        }

        // Errors
        check_error(b"abc", "last line does not end with a newline character");
        check_error(b"abc\ndef\nge\n",
                    "line 3 has different width from first line");
    }
}
