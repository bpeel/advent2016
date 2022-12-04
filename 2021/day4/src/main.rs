const BOARD_SIZE: usize = 5;

#[derive(Debug, Clone)]
struct Board {
    nums: [u8; BOARD_SIZE * BOARD_SIZE],
}

#[derive(Debug, Clone)]
struct Marks {
    // Each entry in the array represents a row of the board with one
    // bit per column
    bits: [u8; BOARD_SIZE],
}

impl Marks {
    fn new() -> Marks {
        Marks { bits: [0; BOARD_SIZE] }
    }

    fn set(&mut self, x: usize, y: usize) {
        self.bits[y] |= 1u8 << x;
    }

    fn row_wins(&self, board: &Board) -> Option<u32> {
        for (row, &row_bits) in self.bits.iter().enumerate() {
            if row_bits == (1u8 << BOARD_SIZE) - 1 {
                return Some(board.nums[row * BOARD_SIZE..(row + 1) * BOARD_SIZE]
                            .iter()
                            .map(|&val| val as u32)
                            .sum())
            }
        }

        None
    }

    fn column_wins(&self, board: &Board) -> Option<u32> {
        'column: for column in 0..BOARD_SIZE {
            let column_bit = 1u8 << column;

            for &row_bits in self.bits.iter() {
                if (row_bits & column_bit) == 0 {
                    continue 'column;
                }
            }

            return Some((0..BOARD_SIZE)
                        .map(|row| board.nums[row * BOARD_SIZE + column] as u32)
                        .sum());
        }

        None
    }

    fn wins(&self, board: &Board) -> Option<u32> {
        if let Some(score) = self.row_wins(board) {
            return Some(score);
        }

        if let Some(score) = self.column_wins(board) {
            return Some(score);
        }

        None
    }
}

fn read_boards<I>(mut lines: I) -> Vec<Board>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let mut boards = Vec::<Board>::new();

    loop {
        let mut board = Board { nums: [0; BOARD_SIZE * BOARD_SIZE] };

        for row in 0..BOARD_SIZE {
            let line = lines.next().unwrap().unwrap();

            for (column, num_str) in line.split_whitespace().enumerate() {
                board.nums[row * BOARD_SIZE + column] =
                    num_str.parse::<u8>().unwrap();
            }
        }

        boards.push(board);

        match lines.next() {
            None => break,
            Some(result) => result.unwrap(),
        };
    }

    boards
}

fn part1(nums: &[u8], boards: &[Board]) {
    let mut marks: Vec<Marks> =
        (0..boards.len()).map(|_| Marks::new()).collect();

    for &num in nums.iter() {
        for (board_num, board) in boards.iter().enumerate() {
            if let Some(index) = board.nums.iter().position(|&n| n == num) {
                marks[board_num].set(index % BOARD_SIZE,
                                     index / BOARD_SIZE);

                if let Some(score) = marks[board_num].wins(board) {
                    println!("part 1: {}", score);
                    return;
                }
            }
        }
    }
}

fn main() {
    let mut lines = std::io::stdin().lines();

    let nums: Vec<u8> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    lines.next().unwrap().unwrap();

    let boards = read_boards(lines);

    part1(&nums, &boards);
}
