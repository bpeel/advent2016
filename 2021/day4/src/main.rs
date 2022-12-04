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

    fn row_wins(&self) -> bool {
        for &row_bits in self.bits.iter() {
            if row_bits == (1u8 << BOARD_SIZE) - 1 {
                return true;
            }
        }

        false
    }

    fn column_wins(&self) -> bool {
        'column: for column in 0..BOARD_SIZE {
            let column_bit = 1u8 << column;

            for &row_bits in self.bits.iter() {
                if (row_bits & column_bit) == 0 {
                    continue 'column;
                }
            }

            return true;
        }

        false
    }

    fn wins(&self) -> bool {
        self.row_wins() || self.column_wins()
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

fn get_unmarked_sum(board: &Board, marks: &Marks) -> u32 {
    let mut sum = 0;

    for (row, &(mut row_bits)) in marks.bits.iter().enumerate() {
        row_bits = row_bits ^ ((1u8 << BOARD_SIZE) - 1);

        loop {
            let column = row_bits.trailing_zeros();

            if column >= BOARD_SIZE as u32 {
                break;
            }

            sum += board.nums[row * BOARD_SIZE + column as usize] as u32;

            row_bits &= !(1u8 << column);
        }
    }

    sum
}

fn part1(nums: &[u8], boards: &[Board]) {
    let mut marks: Vec<Marks> =
        (0..boards.len()).map(|_| Marks::new()).collect();

    for &num in nums.iter() {
        for (board_num, board) in boards.iter().enumerate() {
            if let Some(index) = board.nums.iter().position(|&n| n == num) {
                marks[board_num].set(index % BOARD_SIZE,
                                     index / BOARD_SIZE);

                if marks[board_num].wins() {
                    let unmarked_sum =
                        get_unmarked_sum(board, &marks[board_num]);
                    println!("score = {}, num = {}", unmarked_sum, num);
                    println!("part 1: {}", unmarked_sum * num as u32);
                    return;
                }
            }
        }
    }
}

fn part2(nums: &[u8], boards: &[Board]) {
    let mut marks: Vec<Marks> =
        (0..boards.len()).map(|_| Marks::new()).collect();

    let mut winner_count = 0;

    for &num in nums.iter() {
        for (board_num, board) in boards.iter().enumerate() {
            if let Some(index) = board.nums.iter().position(|&n| n == num) {
                let was_losing = !marks[board_num].wins();

                marks[board_num].set(index % BOARD_SIZE,
                                     index / BOARD_SIZE);

                if was_losing && marks[board_num].wins() {
                    winner_count += 1;

                    if winner_count == boards.len() {
                        let unmarked_sum =
                            get_unmarked_sum(&boards[board_num],
                                             &marks[board_num]);
                        println!("score = {}, num = {}", unmarked_sum, num);
                        println!("part 2: {}", unmarked_sum * num as u32);
                        return;
                    }
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
    part2(&nums, &boards);
}
