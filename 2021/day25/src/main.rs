mod util;

use util::Grid;

struct State {
    grid: Grid,
    temp: Grid,
}

impl State {
    fn new(grid: Grid) -> State {
        let temp = grid.clone();

        State { grid, temp }
    }

    fn step_herd<F>(&mut self, herd_char: u8, mut direction_func: F) -> bool
    where
        F: FnMut(&Grid, (usize, usize)) -> (usize, usize),
    {
        let mut ret = false;

        let src = &self.grid;
        let dst = &mut self.temp;

        dst.values.copy_from_slice(&src.values);

        for y in 0..src.height {
            for x in 0..src.width {
                let this_index = y * src.width + x;

                if src.values[this_index] != herd_char {
                    continue;
                }

                let (next_x, next_y) = direction_func(src, (x, y));

                let next_index = next_y * src.width + next_x;

                if src.values[next_index] == b'.' {
                    dst.values[this_index] = b'.';
                    dst.values[next_index] = herd_char;
                    ret = true;
                }
            }
        }

        std::mem::swap(&mut self.grid, &mut self.temp);

        ret
    }

    fn step_east(&mut self) -> bool {
        self.step_herd(
            b'>',
            |grid, (x, y)| ((x + 1) % grid.width, y),
        )
    }

    fn step_south(&mut self) -> bool {
        self.step_herd(
            b'v',
            |grid, (x, y)| (x, (y + 1) % grid.height),
        )
    }

    fn step(&mut self) -> bool {
        self.step_east() | self.step_south()
    }
}

fn main() -> std::process::ExitCode {
    let grid = match util::Grid::load(&mut std::io::stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
        Ok(grid) => grid,
    };

    let mut state = State::new(grid);

    println!("{}\n", state.grid);

    for steps in 1.. {
        let result = state.step();

        println!("{}\n", state.grid);

        if !result {
            println!("part 1: {}", steps);
            break;
        }
    }

    std::process::ExitCode::SUCCESS
}
