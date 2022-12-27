#[derive(Debug, Clone)]
struct Num {
    value: i64,
    prev: usize,
    next: usize,
}

fn read_nums<I>(lines: &mut I) -> Result<Vec<Num>, String>
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let mut nums = Vec::<Num>::new();

    for (line_num, result) in lines.enumerate() {
        let line = match result {
            Err(e) => return Err(e.to_string()),
            Ok(line) => line,
        };

        let value = match line.parse::<i64>() {
            Ok(v) => v,
            Err(e) => return Err(format!("line {}: {}", line_num + 1, e)),
        };

        nums.push(Num {
            value,
            prev: if line_num == 0 { 0 } else { line_num - 1},
            next: line_num + 1,
        });
    }

    let len = nums.len();

    if len < 1 {
        return Err("no numbers".to_string());
    }

    nums[0].prev = len - 1;
    nums[len - 1].next = 0;

    Ok(nums)
}

fn print_nums(nums: &[Num], start_pos: usize) {
    let mut pos = start_pos;
    let mut count = 0;

    loop {
        if count > 0 {
            print!(",");
        }
        print!("{}", nums[pos].value);
        count += 1;
        assert_eq!(nums[nums[pos].prev].next, pos);
        assert_eq!(nums[nums[pos].next].prev, pos);
        pos = nums[pos].next;
        if pos == start_pos {
            break;
        }
    }

    assert_eq!(count, nums.len());

    println!("");
}

fn move_nums(nums: &mut [Num]) -> usize {
    let mut start = 0;

    for i in 0..nums.len() {
        if i == start {
            start = nums[i].next;
        }

        let mut prev = nums[i].prev;

        // unlink the number from the list
        nums[nums[i].prev].next = nums[i].next;
        nums[nums[i].next].prev = nums[i].prev;

        let offset = if nums[i].value < 0 {
            nums.len() - 1 - (-nums[i].value % (nums.len() as i64 - 1)) as usize
        } else {
            nums[i].value as usize % (nums.len() - 1)
        };

        for _ in 0..offset {
            prev = nums[prev].next;
        }

        // relink the number at the new position
        nums[i].prev = prev;
        nums[i].next = nums[prev].next;
        nums[prev].next = i;
        nums[nums[i].next].prev = i;

        if nums.len() <= 20 {
            print!("{}: ", nums[i].value);
            print_nums(nums, start);
        }
    }

    start
}

fn get_grove_coordinates(nums: &[Num]) -> i64 {
    let start_pos = nums.iter().position(|n| n.value == 0).unwrap();
    let mut pos = start_pos;
    let mut count = 0;
    let mut sum = 0;

    loop {
        for i in 1..=3 {
            if count == i * 1000 % nums.len() {
                sum += nums[pos].value;
            }
        }

        pos = nums[pos].next;

        if pos == start_pos {
            break;
        }

        count += 1;
    }

    sum
}


fn main() -> std::process::ExitCode {
    let mut nums = match read_nums(&mut std::io::stdin().lines()) {
        Ok(nums) => nums,
        Err(e) => {
            eprintln!("{}", e);
            return std::process::ExitCode::FAILURE;
        },
    };

    move_nums(&mut nums);

    println!("part 1: {}", get_grove_coordinates(&nums));

    std::process::ExitCode::SUCCESS
}
