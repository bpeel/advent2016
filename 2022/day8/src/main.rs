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
    values: Box<[bool]>,
}

impl Visibility {
    fn new(map: &Map) -> Visibility {
        let mut vis = Visibility {
            width: map.width,
            height: map.height,
            values: std::iter::repeat(true)
                .take(map.width * map.height)
                .collect(),
        };

        // Sweep left
        vis.sweep(map,
                  (0..vis.values.len()).step_by(vis.width),
                  0..vis.width);
        // Sweep right
        vis.sweep(map,
                  (0..vis.values.len()).step_by(vis.width),
                  (0..vis.width).rev());
        // Sweep down
        vis.sweep(map,
                  0..vis.width,
                  (0..vis.values.len()).step_by(vis.height));
        // Sweep up
        vis.sweep(map,
                  0..vis.width,
                  (0..vis.values.len()).step_by(vis.height).rev());

        vis
    }

    fn sweep<O, I>(&mut self,
                   map: &Map,
                   outer_iter: O,
                   inner_iter: I)
        where O: Iterator<Item = usize>,
              I: Iterator<Item = usize> + Clone,
    {
        assert_eq!(self.width, map.width);
        assert_eq!(self.height, map.height);

        for outer in outer_iter {
            let mut tallest_tree = 0u8;

            for inner in inner_iter.clone() {
                let pos = outer + inner;
                let this_tree = map.values[pos];

                if this_tree <= tallest_tree {
                    self.values[pos] = false;
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

        if x != 0 {
            return Err(Error::new(ErrorKind::Other,
                                  "last line does not end with a \
                                   newline character"));
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

    println!("{:?}", map);

    let vis = Visibility::new(&map);

    for y in 0..vis.height {
        for x in 0..vis.width {
            print!("{}", if vis.values[y * vis.width + x] { '*' } else { '.' });
        }
        println!("");
    }


    std::process::ExitCode::SUCCESS
}
