use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    values: Box<[u8]>,
}

#[derive(Debug, Clone)]
struct Visibility {
    width: usize,
    height: usize,
    values: Box<[u8]>,
}

impl Visibility {
    const FROM_LEFT: u8 = 1u8 << 0;
    const FROM_RIGHT: u8 = 1u8 << 1;
    const FROM_TOP: u8 = 1u8 << 2;
    const FROM_BOTTOM: u8 = 1u8 << 3;
    const ALL: u8 = 0b1111u8;

    fn new(map: &Map) -> Visibility {
        let mut vis = Visibility {
            width: map.width,
            height: map.height,
            values: std::iter::repeat(Visibility::ALL)
                .take(map.width * map.height)
                .collect(),
        };

        // Sweep left
        vis.sweep(map,
                  Visibility::FROM_LEFT,
                  (0..vis.values.len()).step_by(vis.width),
                  0..vis.width);
        // Sweep right
        vis.sweep(map,
                  Visibility::FROM_RIGHT,
                  (0..vis.values.len()).step_by(vis.width),
                  (0..vis.width).rev());
        // Sweep down
        vis.sweep(map,
                  Visibility::FROM_TOP,
                  0..vis.width,
                  (0..vis.values.len()).step_by(vis.height));
        // Sweep up
        vis.sweep(map,
                  Visibility::FROM_BOTTOM,
                  0..vis.width,
                  (0..vis.values.len()).step_by(vis.height).rev());

        vis
    }

    fn sweep<O, I>(&mut self,
                   map: &Map,
                   bit: u8,
                   outer_iter: O,
                   inner_iter: I)
        where O: Iterator<Item = usize>,
              I: Iterator<Item = usize> + Clone,
    {
        assert_eq!(self.width, map.width);
        assert_eq!(self.height, map.height);

        for outer in outer_iter {
            let mut tallest_tree = -1;

            for inner in inner_iter.clone() {
                let pos = outer + inner;
                let this_tree = map.values[pos] as i16;

                if this_tree <= tallest_tree {
                    self.values[pos] &= !bit;
                } else {
                    tallest_tree = this_tree;
                }
            }
        }
    }
}

fn load_map<T>(input: &mut T) -> Result<Map, Error>
    where T: std::io::BufRead
{
    let mut x = 0;
    let mut width = 0;
    let mut height = 0;
    let mut values = Vec::<u8>::new();

    loop {
        let bytes = input.fill_buf()?;

        if bytes.len() == 0 {
            if x != 0 {
                return Err(Error::new(ErrorKind::Other,
                                      "last line does not end with a \
                                       newline character"));
            }

            break Ok(Map { width, height, values: Box::from(values) });
        }

        for byte in bytes.iter() {
            match byte {
                b'0'..=b'9' => {
                    values.push(*byte - b'0');
                    x += 1;
                },
                b'\n' => {
                    if height == 0 {
                        width = x;
                    } else if width != x {
                        return Err(Error::new(ErrorKind::Other,
                                              format!("line {} has different \
                                                       width from first line",
                                                      height + 1)));
                    }

                    x = 0;
                    height += 1;
                }
                _ => return Err(Error::new(ErrorKind::Other,
                                           format!("line {} contains an \
                                                    unexpected character",
                                           height + 1))),
            }
        }

        let len = bytes.len();
        input.consume(len);
    }
}

fn main() -> std::process::ExitCode {
    let map = match load_map(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(map) => map,
    };

    let vis = Visibility::new(&map);

    println!("part 1: {}", vis.values.iter().filter(|&&v| v > 0).count());

    std::process::ExitCode::SUCCESS
}
