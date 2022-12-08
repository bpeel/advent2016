use std::io::{Error, ErrorKind};

// The parsed input with a u8 to represent each tree height
#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    values: Box<[u8]>,
}

// Data deduced for each tree in the map
#[derive(Debug, Clone)]
struct PointVisibility {
    // Part1: a bit mask of visibility of the edge of the map in each
    // direction. Eg a 1 in the first bit means the tree can be seen
    // from the left of the map, the next bit is from the right of the
    // map, etc. If the mask is nonzero then the tree is visible from
    // the outside.
    bits: u8,
    // Part2: How far we can see in each direction from this tree
    distances: [u8; 4],
}

// The array of deduced visibility information for the entire map
#[derive(Debug, Clone)]
struct Visibility {
    width: usize,
    height: usize,
    values: Box<[PointVisibility]>,
}

impl Visibility {
    const FROM_LEFT: u8 = 0;
    const FROM_RIGHT: u8 = 1;
    const FROM_TOP: u8 = 2;
    const FROM_BOTTOM: u8 = 3;
    // Bitmask to set if the tree is visible from all directions (this
    // is the starting assumption during the calculation
    const ALL: u8 = 0b1111u8;

    fn new(map: &Map) -> Visibility {
        let mut vis = Visibility {
            width: map.width,
            height: map.height,
            values: std::iter::repeat(PointVisibility {
                bits: Visibility::ALL,
                distances: [0u8; 4]
            })
                .take(map.width * map.height)
                .collect(),
        };

        // Iterates over the map in each of the four directions. The
        // sweep function is a for loop nested in a for loop to cover
        // the entire map. The four combinations of sweeps are all of
        // the combinations of either having the x or the y axis in
        // the outer loop, and then either iterating forwards or
        // backwards in the inner loop. Each sweep is expressed with
        // two iterators, one for the outer loop and one for the inner
        // loop. The values from the iterators added together give the
        // index into the map array.

        // Sweep right
        vis.sweep(map,
                  Visibility::FROM_LEFT,
                  (0..vis.values.len()).step_by(vis.width),
                  0..vis.width);
        // Sweep left
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
                   axis: u8,
                   outer_iter: O,
                   inner_iter: I)
        where O: Iterator<Item = usize>,
              I: Iterator<Item = usize> + Clone,
    {
        assert_eq!(self.width, map.width);
        assert_eq!(self.height, map.height);

        for outer in outer_iter {
            // The tallest tree that we’ve seen so far along this
            // axis. This is used for part 1.
            let mut tallest_tree = -1;
            // An array of positions along the current axis indexed by
            // tree height. The value represents the position of the
            // nearest tree that would block a tree which has the
            // height of the array index. This is updated during the
            // sweep.
            let mut blocker_positions = [0usize; 10];

            // For the inner loop, we use enumerate so that inner_num
            // will be the number of steps we have taken along this
            // axis for the sweep so far
            for (inner_num, inner) in inner_iter.clone().enumerate() {
                let pos = outer + inner;
                let this_tree = map.values[pos] as i16;

                // Part 1: if the tree is shorter or equal to the
                // tallest tree we’ve seen so far then remove the
                // visibility bit for this axis. Otherwise this tree
                // is the tallest tree so far.
                if this_tree <= tallest_tree {
                    self.values[pos].bits &= !(1u8 << axis);
                } else {
                    tallest_tree = this_tree;
                }

                // Part 2: blocker_positions contains the position
                // along the current direction of the nearest tree
                // that would block visibility for a tree of each
                // height. We can use this to easy calculate the
                // distance to that tree.
                self.values[pos].distances[axis as usize] =
                    (inner_num - blocker_positions[this_tree as usize]) as u8;

                // Update blocker_positions. This tree is the new
                // nearest tree that would block any later trees that
                // have the same height or less.
                for i in 0..=this_tree as usize {
                    blocker_positions[i] = inner_num;
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

    // Count the number of trees that are visible from at least one direction
    println!("part 1: {}", vis.values.iter().filter(|v| v.bits > 0).count());

    // For each tree, calculate the score by multiplying together all
    // of the distances to the nearest blocking tree in each
    // direction. Take the maximum score.
    let part2 = vis.values.iter()
        .map(|pv| pv.distances.iter().fold(1, |a, &b| a as u32 * b as u32))
        .max().unwrap();

    println!("part 2: {}", part2);

    std::process::ExitCode::SUCCESS
}
