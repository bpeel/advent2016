mod util;
mod walker;

use std::io::stdin;
use std::process::ExitCode;
use util::Grid;
use walker::QuadDirection;
use std::fmt;

#[derive(Clone)]
struct Scene {
    pos: (i32, i32),
    grid: Grid,
}

impl Scene {
    fn new(mut grid: Grid) -> Scene {
        let pos = match grid.values.iter().position(|&ch| ch == b'@') {
            Some(index) => {
                grid.values[index] = b'.';
                (
                    (index % grid.width) as i32,
                    (index / grid.width) as i32,
                )
            },
            None => (0, 0),
        };

        Scene {
            pos,
            grid,
        }
    }

    fn can_move_vertically(&self, dir: QuadDirection) -> bool {
        let mut stack = vec![dir.move_pos(self.pos)];

        while let Some(pos) = stack.pop() {
            let next_pos = dir.move_pos(pos);

            match self.grid.get(pos) {
                Some(b'O') => stack.push(next_pos),
                Some(b'[') => {
                    stack.push(next_pos);
                    stack.push((next_pos.0 + 1, next_pos.1));
                },
                Some(b']') => {
                    stack.push(next_pos);
                    stack.push((next_pos.0 - 1, next_pos.1));
                },
                Some(b'.') => (),
                _ => return false,
            }
        }

        true
    }

    fn can_move_horizontally(&self, dir: QuadDirection)  -> bool {
        let mut pos = self.pos;

        loop {
            pos = dir.move_pos(pos);

            match self.grid.get(pos) {
                Some(b'.') => break true,
                Some(b'O' | b'[' | b']') => (),
                _ => break false,
            }
        }
    }

    fn move_horizontally(&mut self, dir: QuadDirection) {
        self.pos = dir.move_pos(self.pos);

        let line_start = self.pos.1 as usize * self.grid.width;
        let line = &mut self.grid.values[
            line_start..line_start + self.grid.width
        ];

        if dir.offset().0 < 0 {
            let dot = line[0..=self.pos.0 as usize].iter().rposition(|&ch| {
                ch == b'.'
            }).unwrap();
            line.copy_within(dot + 1..self.pos.0 as usize + 1, dot);
        } else {
            let dot = line[self.pos.0 as usize..].iter().position(|&ch| {
                ch == b'.'
            }).unwrap() + self.pos.0 as usize;
            line.copy_within(
                self.pos.0 as usize..dot,
                self.pos.0 as usize + 1,
            );
        };

        line[self.pos.0 as usize] = b'.';
    }

    fn set_grid(&mut self, (x, y): (i32, i32), v: u8) {
        self.grid.values[y as usize * self.grid.width + x as usize] = v;
    }

    fn move_vertically(&mut self, dir: QuadDirection) {
        self.pos = dir.move_pos(self.pos);
        let mut stack = vec![(b'.', self.pos)];

        while let Some((new_ch, pos)) = stack.pop() {
            let next_pos = dir.move_pos(pos);

            match self.grid.get(pos) {
                Some(b'O') => {
                    self.set_grid(pos, new_ch);
                    stack.push((b'O', next_pos));
                },
                Some(b'[') => {
                    self.set_grid(pos, new_ch);
                    self.set_grid((pos.0 + 1, pos.1), b'.');
                    stack.push((b'[', next_pos));
                    stack.push((b']', (next_pos.0 + 1, next_pos.1)));
                },
                Some(b']') => {
                    self.set_grid(pos, new_ch);
                    self.set_grid((pos.0 - 1, pos.1), b'.');
                    stack.push((b']', next_pos));
                    stack.push((b'[', (next_pos.0 - 1, next_pos.1)));
                },
                Some(b'.') => {
                    self.set_grid(pos, new_ch);
                },
                _ => unreachable!("bad map character"),
            }
        }
    }

    fn step(&mut self, dir: QuadDirection) {
        if dir.offset().1 == 0 {
            if self.can_move_horizontally(dir) {
                self.move_horizontally(dir);
            }
        } else {
            if self.can_move_vertically(dir) {
                self.move_vertically(dir);
            }
        }
    }

    fn expand(&self) -> Scene {
        let mut values = Vec::with_capacity(self.grid.values.len() * 2);

        for &ch in self.grid.values.iter() {
            values.extend_from_slice(
                &if ch == b'O' {
                    b"[]".clone()
                } else {
                    [ch, ch]
                }
            );
        }

        Scene {
            pos: (self.pos.0 * 2, self.pos.1),
            grid: Grid {
                values: values.into_boxed_slice(),
                width: self.grid.width * 2,
                height: self.grid.height,
            }
        }
    }
}

impl fmt::Display for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                if (x as i32, y as i32) == self.pos {
                    write!(f, "@")?;
                } else {
                    write!(
                        f,
                        "{}",
                        self.grid.values[y * self.grid.width + x] as char
                    )?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn read_directions() -> Result<Vec<QuadDirection>, String> {
    let mut dirs = Vec::new();

    for result in stdin().lines() {
        let line = result.map_err(|e| e.to_string())?;

        for ch in line.chars() {
            match QuadDirection::from_char(ch) {
                Some(dir) => dirs.push(dir),
                None => return Err(format!("bad dir: {}", ch)),
            }
        }
    }

    Ok(dirs)
}

fn run_scene(mut scene: Scene, dirs: &[QuadDirection]) -> usize {
    for &dir in dirs {
        scene.step(dir);
    }

    scene.grid.values.iter().enumerate()
        .filter_map(|(index, &ch)| {
            (ch == b'O' || ch == b'[').then(|| {
                index / scene.grid.width * 100 + index % scene.grid.width
            })
        }).sum::<usize>()
}

fn main() -> ExitCode {
    let scene = match Grid::load(&mut stdin().lock()) {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(grid) => Scene::new(grid),
    };

    let dirs = match read_directions() {
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        },
        Ok(dirs) => dirs,
    };

    println!("Part 1: {}", run_scene(scene.clone(), &dirs));
    let scene = scene.expand();
    println!("Part 2: {}", run_scene(scene.clone(), &dirs));

    ExitCode::SUCCESS
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::VecDeque;

    fn test_move(
        grid_str: &str,
        dir: QuadDirection,
        result: &str,
    ) {
        let mut grid = VecDeque::from(grid_str.to_owned().into_bytes());
        let grid = Grid::load(&mut grid).unwrap();
        let mut scene = Scene::new(grid);
        scene.step(dir);

        if &scene.to_string() != result {
            println!("{}->\n{}", grid_str, scene.to_string());
        }

        assert_eq!(&scene.to_string(), result);
    }

    #[test]
    fn move_left() {
        test_move(
            "....\n\
             ..O@\n",
            QuadDirection::Left,
            "....\n\
             .O@.\n",
        );
        test_move(
            "....\n\
             .OO@\n",
            QuadDirection::Left,
            "....\n\
             OO@.\n",
        );
        test_move(
            "....\n\
             OOO@\n",
            QuadDirection::Left,
            "....\n\
             OOO@\n",
        );
    }

    #[test]
    fn move_right()  {
        test_move(
            ".....\n\
             .@OO.\n",
            QuadDirection::Right,
            ".....\n\
             ..@OO\n",
        );
        test_move(
            ".@.##..\n",
            QuadDirection::Right,
            "..@##..\n",
        );
    }

    #[test]
    fn move_up() {
        test_move(
            "....\n\
             ..O.\n\
             ..O.\n\
             ..@.\n",
            QuadDirection::Up,
            "..O.\n\
             ..O.\n\
             ..@.\n\
             ....\n",
        );
        test_move(
            "....\n\
             .[]O\n\
             ..[]\n\
             ..@.\n",
            QuadDirection::Up,
            ".[]O\n\
             ..[]\n\
             ..@.\n\
             ....\n",
        );
        test_move(
            ".......\n\
             ..[]...\n\
             .[][]..\n\
             [][][]O\n\
             .[][][]\n\
             ..[][].\n\
             ...[]..\n\
             ...@...\n",
            QuadDirection::Up,
            "..[]...\n\
             .[][]..\n\
             [][][]O\n\
             .[][][]\n\
             ..[][].\n\
             ...[]..\n\
             ...@...\n\
             .......\n"
        );
        test_move(
            ".......\n\
             ..[]...\n\
             .[][].#\n\
             [][][]O\n\
             .[][][]\n\
             ..[][].\n\
             ...[]..\n\
             ...@...\n",
            QuadDirection::Up,
            ".......\n\
             ..[]...\n\
             .[][].#\n\
             [][][]O\n\
             .[][][]\n\
             ..[][].\n\
             ...[]..\n\
             ...@...\n",
        );
    }
}
