use std::io::BufRead;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum JetDirection {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Jet {
    directions: Vec<JetDirection>,
    jet_pos: usize,
}

impl Jet {
    fn new(directions: Vec<JetDirection>) -> Jet {
        Jet { directions, jet_pos: 0 }
    }

    fn read(&mut self) -> JetDirection {
        let direction = self.directions[self.jet_pos];
        self.jet_pos = (self.jet_pos + 1) % self.directions.len();
        direction
    }
}

static SHAPES: [u16; 5] = [
    0x000f, // horizontal bar
    0x0272, // plus symbol
    0x0447, // reverse L symbol
    0x1111, // vertical bar
    0x0033, // 2x2 box
];

const SHAPE_HEIGHT: usize = 4;
const SHAPE_WIDTH: usize = 4;
const GRID_WIDTH: usize = 7;

fn shape_can_be_at(grid: &[u8],
                   mut shape: u16,
                   x_pos: usize,
                   y_pos: usize) -> bool {
    for shape_y in 0..SHAPE_HEIGHT {
        let grid_line = match grid.get(shape_y + y_pos) {
            None => return true,
            Some(l) => l,
        };

        let shape_line = (shape & ((1u16 << SHAPE_WIDTH) - 1)) as u8;

        if grid_line & (shape_line << x_pos) != 0 {
            return false;
        }

        shape >>= SHAPE_WIDTH;
    }

    true
}

fn can_move_left(grid: &[u8],
                 shape: u16,
                 x_pos: usize,
                 y_pos: usize) -> bool {
    if x_pos <= 0 {
        return false;
    }

    shape_can_be_at(grid, shape, x_pos - 1, y_pos)
}

fn shape_width(mut shape: u16) -> usize {
    let mut longest_width = 0;

    for _ in 0..SHAPE_HEIGHT {
        let line = (shape & ((1u16 << SHAPE_WIDTH) - 1)) as u8;
        let width = u8::BITS as usize - line.leading_zeros() as usize;

        if width > longest_width {
            longest_width = width;
        }

        shape >>= SHAPE_WIDTH;
    }

    longest_width
}

fn can_move_right(grid: &[u8],
                  shape: u16,
                  x_pos: usize,
                  y_pos: usize) -> bool {
    let shape_width = shape_width(shape);

    if x_pos + shape_width + 1 > GRID_WIDTH {
        return false;
    }

    shape_can_be_at(grid, shape, x_pos + 1, y_pos)
}


fn can_move_down(grid: &[u8],
                 shape: u16,
                 x_pos: usize,
                 y_pos: usize) -> bool {
    if y_pos <= 0 {
        return false;
    }

    shape_can_be_at(grid, shape, x_pos, y_pos - 1)
}

fn settle_shape(grid: &mut Vec<u8>,
                mut shape: u16,
                x_pos: usize,
                y_pos: usize) {
    for y in 0..SHAPE_HEIGHT {
        let shape_line = (shape & ((1u16 << SHAPE_WIDTH) - 1)) as u8;

        if shape_line != 0{
            while grid.len() <= y_pos + y {
                grid.push(0);
            }

            grid[y_pos + y] |= shape_line << x_pos;
        }

        shape >>= SHAPE_WIDTH;
    }
}

fn add_shape(grid: &mut Vec<u8>, jet: &mut Jet, shape: u16) {
    let mut y_pos = grid.len() + 3;
    let mut x_pos = 2;

    loop {
        match jet.read() {
            JetDirection::Left => {
                if can_move_left(grid, shape, x_pos, y_pos) {
                    x_pos -= 1;
                }
            },
            JetDirection::Right => {
                if can_move_right(grid, shape, x_pos, y_pos) {
                    x_pos += 1;
                }
            },
        }

        if can_move_down(grid, shape, x_pos, y_pos) {
            y_pos -= 1;
        } else {
            settle_shape(grid, shape, x_pos, y_pos);
            break;
        }
    }
}

fn read_jet<R: BufRead>(reader: &mut R) ->
    Result<Jet, String>
{
    let mut directions = Vec::<JetDirection>::new();

    loop {
        let buf = match reader.fill_buf() {
            Err(e) => return Err(e.to_string()),
            Ok(b) => b,
        };

        if buf.is_empty() {
            break;
        }

        for ch in buf.iter() {
            let dir = match ch {
                b'<' => JetDirection::Left,
                b'>' => JetDirection::Right,
                c if c.is_ascii_whitespace() => continue,
                c => return Err(format!("unexpected char “{}”", c)),
            };

            directions.push(dir);
        }

        let consumed = buf.len();
        reader.consume(consumed);
    }

    Ok(Jet::new(directions))
}

fn print_grid(grid: &[u8]) {
    for &(mut line) in grid.iter().rev() {
        for _ in 0..GRID_WIDTH {
            print!("{}", if (line & 1) == 0 { '.' } else { '#' });
            line >>= 1;
        }
        println!("");
    }
}

fn main() -> std::process::ExitCode {
    let mut jet = match read_jet(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(d) => d,
    };

    let mut grid = Vec::<u8>::new();

    for shape_num in 0..2022 {
        add_shape(&mut grid, &mut jet, SHAPES[shape_num % SHAPES.len()]);
    }

    print_grid(&grid[0..20]);

    println!("part 1: {}", grid.len());

    std::process::ExitCode::SUCCESS
}
