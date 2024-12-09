use std::process::ExitCode;
use std::str::FromStr;
use std::fmt;

type FileId = u16;

struct Disk {
    blocks: Vec<Option<FileId>>,
}

impl FromStr for Disk {
    type Err = String;

    fn from_str(s: &str) -> Result<Disk, String> {
        let mut blocks = Vec::new();
        let mut file_id: FileId = 0;
        let mut is_file = true;

        for ch in s.chars() {
            let Some(length) = ch.to_digit(10)
            else {
                return Err(format!("invalid character: {}", ch));
            };

            let block = if is_file {
                let this_file_id = file_id;

                let Some(next_file) = file_id.checked_add(1)
                else {
                    return Err("too many files".to_string());
                };

                file_id = next_file;

                Some(this_file_id)
            } else {
                None
            };

            blocks.resize(blocks.len() + length as usize, block);

            is_file = !is_file;
        }

        Ok(Disk {
            blocks
        })
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for block in self.blocks.iter() {
            match block {
                Some(file_id) => write!(f, "{}", file_id)?,
                None => write!(f, ".")?,
            }
        }

        Ok(())
    }
}

fn calculate_checksum(disk: &Disk) -> u64 {
    disk.blocks.iter().enumerate().map(|(i, &block)| {
        block.map(|file_id| i as u64 * file_id as u64).unwrap_or(0)
    }).sum::<u64>()
}

fn compact(disk: &mut Disk) {
    let mut next_free_space = 0;

    'compact_loop: for end in (0..disk.blocks.len()).rev() {
        if let Some(file_id) = disk.blocks[end] {
            loop {
                let space = next_free_space;

                if space >= end {
                    break 'compact_loop;
                }

                next_free_space += 1;

                if disk.blocks[space].is_none() {
                    disk.blocks[space] = Some(file_id);
                    disk.blocks[end] = None;
                    break;
                }
            }
        }
    }
}

fn main() -> ExitCode {
    for (line_num, result) in std::io::stdin().lines().enumerate() {
        let line = match result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            },
        };

        let mut disk = match line.parse::<Disk>() {
            Ok(disk) => disk,
            Err(e) => {
                eprintln!("{}: {}", line_num + 1, e);
                return ExitCode::FAILURE;
            },
        };

        compact(&mut disk);

        println!("Part 1: {}", calculate_checksum(&disk));
    }

    ExitCode::SUCCESS
}
