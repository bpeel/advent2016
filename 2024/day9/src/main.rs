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

#[derive(Clone, Copy, PartialEq, Eq)]
enum SpaceType {
    File(u16),
    Free,
}

#[derive(Clone, Copy)]
struct Space {
    space_type: SpaceType,
    length: u32,
    next: u16,
    prev: u16,
}

struct SpacedDisk {
    spaces: Vec<Space>,
    file_indices: Vec<u16>,
}

impl FromStr for SpacedDisk {
    type Err = String;

    fn from_str(s: &str) -> Result<SpacedDisk, String> {
        // First entry in the array is the header link
        let mut spaces = vec![Space {
            space_type: SpaceType::Free,
            length: 0,
            next: 1,
            prev: 0,
        }];
        let mut file_indices = Vec::new();

        for (i, ch) in s.chars().enumerate() {
            let Some(length) = ch.to_digit(10)
            else {
                return Err(format!("invalid character: {}", ch));
            };

            let space_type = if i & 1 == 0 {
                let Ok(file_id) = FileId::try_from(i / 2)
                else {
                    return Err("too many files".to_string());
                };

                file_indices.push(spaces.len() as u16);

                SpaceType::File(file_id)
            } else {
                SpaceType::Free
            };

            spaces.push(Space {
                space_type,
                length,
                next: i as u16 + 2,
                prev: i as u16,
            });
        }

        spaces.last_mut().unwrap().next = 0;

        let len = spaces.len();

        if len > 0 {
            spaces[0].next = 1;
            spaces[0].prev = len as u16 - 1;
        }

        Ok(SpacedDisk {
            spaces,
            file_indices,
        })
    }
}

impl fmt::Display for SpacedDisk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut index = self.spaces[0].next;

        while index != 0 {
            let space = self.spaces[index as usize];

            let ch = match space.space_type {
                SpaceType::File(file_id) => {
                    if file_id > 9 {
                        'x'
                    } else {
                        (file_id as u8 + b'0') as char
                    }
                },
                SpaceType::Free => '.',
            };

            for _ in 0..space.length {
                write!(f, "{}", ch)?;
            }

            index = space.next;
        }

        Ok(())
    }
}

fn unlink_node(disk: &mut SpacedDisk, pos: u16) {
    let next = disk.spaces[pos as usize].next;
    let prev = disk.spaces[pos as usize].prev;

    disk.spaces[prev as usize].next = next;
    disk.spaces[next as usize].prev = prev;
}

fn unlink_file(disk: &mut SpacedDisk, pos: u16) {
    let space = disk.spaces[pos as usize];

    unlink_node(disk, pos);

    if space.prev != 0 &&
        disk.spaces[space.prev as usize].space_type == SpaceType::Free
    {
        disk.spaces[space.prev as usize].length += space.length;

        if space.next != 0 &&
            disk.spaces[space.next as usize].space_type == SpaceType::Free
        {
            disk.spaces[space.prev as usize].length +=
                disk.spaces[space.next as usize].length;
            unlink_node(disk, space.next);
        }
    } else if space.next != 0 &&
        disk.spaces[space.next as usize].space_type == SpaceType::Free
    {
        disk.spaces[space.next as usize].length += space.length;
    } else {
        let new_node = disk.spaces.len() as u16;

        disk.spaces.push(Space {
            space_type: SpaceType::Free,
            length: space.length,
            next: 0,
            prev: 0,
        });

        insert_node(disk, space.prev, new_node);
    }
}

fn insert_node(disk: &mut SpacedDisk, before: u16, node: u16) {
    let next = disk.spaces[before as usize].next;
    disk.spaces[node as usize].prev = before;
    disk.spaces[node as usize].next = next;
    disk.spaces[before as usize].next = node;
    disk.spaces[next as usize].prev = node;
}

fn move_file(disk: &mut SpacedDisk, file_index: u16) {
    let mut space_index = disk.spaces[0].next;
    let file_length = disk.spaces[file_index as usize].length;

    while space_index != 0 && space_index != file_index {
        let space = disk.spaces[space_index as usize];

        if space.space_type == SpaceType::Free && space.length >= file_length {
            unlink_file(disk, file_index);

            insert_node(disk, space.prev, file_index);

            if space.length == file_length {
                unlink_node(disk, space_index);
            } else {
                disk.spaces[space_index as usize].length -= file_length;
            }

            break;
        }

        space_index = space.next;
    }
}

fn compact_spaced_disk(disk: &mut SpacedDisk) {
    let file_indices = std::mem::take(&mut disk.file_indices);

    println!("{}", disk);

    for &file_index in file_indices.iter().rev() {
        move_file(disk, file_index);
        println!("{}", disk);
    }

    disk.file_indices = file_indices;
}

fn calculate_spaced_checksum(disk: &SpacedDisk) -> u64 {
    let mut sum = 0u64;
    let mut position = 0u64;
    let mut space_index = disk.spaces[0].next;

    while space_index != 0 {
        let space = &disk.spaces[space_index as usize];

        if let SpaceType::File(file_id) = space.space_type {
            sum += (position..position + space.length as u64).map(|pos| {
                file_id as u64 * pos
            }).sum::<u64>();
        }

        position += space.length as u64;
        space_index = space.next;
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
