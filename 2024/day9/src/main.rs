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

#[derive(Clone, Copy)]
struct File {
    id: FileId,
    length: u32,
    space_after: u32,
    prev: u16,
    next: u16,
}

struct SpacedDisk {
    files: Vec<File>,
}

impl FromStr for SpacedDisk {
    type Err = String;

    fn from_str(s: &str) -> Result<SpacedDisk, String> {
        // First entry in the array is the header link
        let mut files = vec![File {
            id: FileId::MAX,
            length: 0,
            space_after: 0,
            prev: 0,
            next: 0,
        }];

        for (i, ch) in s.chars().enumerate() {
            let Some(length) = ch.to_digit(10)
            else {
                return Err(format!("invalid character: {}", ch));
            };

            if i & 1 == 0 {
                files.push(File {
                    id: (i / 2) as u16,
                    length: length,
                    space_after: 0,
                    prev: (i / 2) as u16,
                    next: (i / 2 + 2) as u16,
                });
            } else {
                files.last_mut().unwrap().space_after += length;
            };
        }

        files.last_mut().unwrap().next = 0;

        let len = files.len();

        if len > 0 {
            files[0].next = 1;
            files[0].prev = len as u16 - 1;
        }

        Ok(SpacedDisk {
            files,
        })
    }
}

impl fmt::Display for SpacedDisk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut index = self.files[0].next;

        while index != 0 {
            let file = &self.files[index as usize];

            let ch = if file.id < 10 {
                (file.id as u8 + b'0') as char
            } else {
                'x'
            };

            for _ in 0..file.length {
                write!(f, "{}", ch)?;
            }

            for _ in 0..file.space_after {
                write!(f, ".")?;
            }

            index = file.next;
        }

        Ok(())
    }
}

fn unlink_node(disk: &mut SpacedDisk, pos: u16) {
    let next = disk.files[pos as usize].next;
    let prev = disk.files[pos as usize].prev;

    disk.files[prev as usize].next = next;
    disk.files[next as usize].prev = prev;
}

fn insert_node(disk: &mut SpacedDisk, before: u16, node: u16) {
    let next = disk.files[before as usize].next;
    disk.files[node as usize].prev = before;
    disk.files[node as usize].next = next;
    disk.files[before as usize].next = node;
    disk.files[next as usize].prev = node;
}

fn move_file(disk: &mut SpacedDisk, file_index: u16) {
    let mut index = disk.files[0].next;
    let file_length = disk.files[file_index as usize].length;

    while index != 0 && index != file_index {
        if disk.files[index as usize].space_after >= file_length {
            let prev = disk.files[file_index as usize].prev;

            if prev != 0 {
                disk.files[prev as usize].space_after +=
                    file_length +
                    disk.files[file_index as usize].space_after;
            }

            disk.files[file_index as usize].space_after =
                disk.files[index as usize].space_after -
                file_length;
            disk.files[index as usize].space_after = 0;

            unlink_node(disk, file_index);
            insert_node(disk, index, file_index);

            break;
        }

        index = disk.files[index as usize].next;
    }
}

fn compact_spaced_disk(disk: &mut SpacedDisk) {
    for file_index in (1..disk.files.len()).rev() {
        move_file(disk, file_index as u16);
    }
}

fn calculate_spaced_checksum(disk: &SpacedDisk) -> u64 {
    let mut sum = 0u64;
    let mut position = 0u64;
    let mut file_index = disk.files[0].next;

    while file_index != 0 {
        let file = &disk.files[file_index as usize];

        sum += (position..position + file.length as u64).map(|pos| {
            file.id as u64 * pos
        }).sum::<u64>();

        position += file.length as u64 + file.space_after as u64;
        file_index = file.next;
    }

    sum
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

        let mut disk = match line.parse::<SpacedDisk>() {
            Ok(disk) => disk,
            Err(e) => {
                eprintln!("{}: {}", line_num + 1, e);
                return ExitCode::FAILURE;
            },
        };

        compact_spaced_disk(&mut disk);

        println!("Part 2: {}", calculate_spaced_checksum(&disk));
    }

    ExitCode::SUCCESS
}
