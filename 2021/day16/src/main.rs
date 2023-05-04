mod bitvec;

use bitvec::BitVec;
use std::cmp::{min, max};
use num_enum::{TryFromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Debug)]
enum TreeIterRemaining {
    Literal,
    EndPos(usize),
    PacketsRemaining(usize),
}

#[derive(Clone, Debug)]
struct TreeIter<'a> {
    bv: &'a BitVec,
    pos: usize,
    stack: Vec<TreeIterRemaining>,
}

impl<'a> TreeIter<'a> {
    fn new(bv: &BitVec) -> TreeIter {
        TreeIter {
            bv,
            pos: 0,
            stack: vec![TreeIterRemaining::EndPos(bv.size())],
        }
    }

    fn read_bits(&mut self, n_bits: usize) -> u64 {
        let result = self.bv.read_bits(self.pos, n_bits);

        self.pos += n_bits;

        result
    }

    fn read_literal(&mut self) -> u64 {
        let mut result = 0;

        loop {
            let last = !self.bv.read_bit(self.pos);
            let nibble = self.bv.read_bits(self.pos + 1, 4);

            self.pos += 5;

            result = (result << 4) | nibble;

            if last {
                break result;
            }
        }
    }

    fn read_packet(&mut self) -> Packet {
        let version = self.read_bits(3) as u8;
        let type_id = TypeId::try_from(self.read_bits(3) as u8).unwrap();

        let data = if type_id == TypeId::Literal {
            self.stack.push(TreeIterRemaining::Literal);
            PacketData::Literal(self.read_literal())
        } else if self.read_bits(1) == 0 {
            let remaining = self.read_bits(15) as usize;
            self.stack.push(TreeIterRemaining::EndPos(self.pos + remaining));
            PacketData::BitOperator(remaining)
        } else {
            let remaining = self.read_bits(11) as usize;
            self.stack.push(TreeIterRemaining::PacketsRemaining(remaining));
            PacketData::PacketOperator(remaining)
        };

        Packet {
            version,
            type_id,
            data,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum PacketData {
    Literal(u64),
    BitOperator(usize),
    PacketOperator(usize),
}

#[derive(Clone, Copy, Debug)]
struct Packet {
    version: u8,
    type_id: TypeId,
    data: PacketData,
}

#[derive(Clone, Copy, Debug)]
enum TreeDirection {
    Up,
    Down(Packet),
}

impl<'a> Iterator for TreeIter<'a> {
    type Item = TreeDirection;

    fn next(&mut self) -> Option<TreeDirection> {
        if let Some(tail) = self.stack.pop() {
            let is_end = match tail {
                TreeIterRemaining::EndPos(end_pos) => {
                    if self.pos < end_pos &&
                        (!self.stack.is_empty() ||
                         !self.bv.is_trailing_zeroes(self.pos))
                    {
                        self.stack.push(TreeIterRemaining::EndPos(end_pos));
                        false
                    } else {
                        true
                    }
                },
                TreeIterRemaining::PacketsRemaining(remaining) => {
                    if remaining > 0 {
                        self.stack.push(TreeIterRemaining::PacketsRemaining(
                            remaining - 1
                        ));
                        false
                    } else {
                        true
                    }
                },
                TreeIterRemaining::Literal => true,
            };

            if is_end {
                if self.stack.is_empty() {
                    None
                } else {
                    Some(TreeDirection::Up)
                }
            } else {
                Some(TreeDirection::Down(self.read_packet()))
            }
        } else {
            None
        }
    }
}

struct PacketIter<'a> {
    base: TreeIter<'a>,
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = Packet;

    fn next(&mut self) -> Option<Packet> {
        loop {
            match self.base.next() {
                None => break None,
                Some(TreeDirection::Up) => (),
                Some(TreeDirection::Down(packet)) => break Some(packet),
            }
        }
    }
}

impl<'a> PacketIter<'a> {
    fn new(bv: &BitVec) -> PacketIter {
        PacketIter {
            base: TreeIter::new(bv),
        }
    }
}

fn part1(bv: &BitVec) -> u32 {
    PacketIter::new(bv).map(|p| p.version as u32).sum()
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
enum TypeId {
    Add = 0,
    Multiply = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

impl TypeId {
    fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            TypeId::Add => a + b,
            TypeId::Multiply => a * b,
            TypeId::Minimum => min(a, b),
            TypeId::Maximum => max(a, b),
            TypeId::GreaterThan => (a > b) as u64,
            TypeId::LessThan => (a < b) as u64,
            TypeId::EqualTo => (a == b) as u64,
            TypeId::Literal => {
                unreachable!("Tried to use a literal as an operator");
            },
        }
    }
}

#[derive(Clone, Debug, Copy)]
struct EvaluateEntry {
    value: u64,
    type_id: TypeId,
    is_first: bool,
}

fn part2(bv: &BitVec) -> u64 {
    let mut stack = vec![EvaluateEntry {
        value: 0,
        type_id: TypeId::Add,
        is_first: true,
    }];

    for direction in TreeIter::new(bv) {
        match direction {
            TreeDirection::Up => {
                let value = stack.pop().unwrap().value;
                let tail = stack.last_mut().unwrap();

                if tail.is_first {
                    tail.is_first = false;
                    tail.value = value;
                } else {
                    tail.value = tail.type_id.apply(tail.value, value);
                }
            },
            TreeDirection::Down(packet) => {
                let value = if let PacketData::Literal(value) = packet.data {
                    value
                } else {
                    0
                };

                stack.push(EvaluateEntry {
                    value,
                    type_id: packet.type_id,
                    is_first: true,
                });
            },
        }
    }

    assert_eq!(stack.len(), 1);
    stack.last().unwrap().value
}

fn process_lines<I>(lines: I) -> std::process::ExitCode
    where I: Iterator<Item = Result<String, std::io::Error>>
{
    let mut ret = std::process::ExitCode::SUCCESS;

    for (line_num, line) in lines.enumerate() {
        match line {
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
            Ok(line) => {
                match BitVec::new(&line) {
                    Err(e) => {
                        eprintln!("line {}: {}", line_num + 1, e);
                        ret = std::process::ExitCode::FAILURE;
                    },
                    Ok(bv) => {
                        println!(
                            "part1: {}, part2: {}",
                            part1(&bv),
                            part2(&bv),
                        );
                    },
                };
            },
        }
    }

    ret
}

fn main() -> std::process::ExitCode {
    let mut args = std::env::args();

    args.next();

    if args.len() > 0 {
        process_lines(args.map(|arg| Ok(arg)))
    } else {
        process_lines(std::io::stdin().lines())
    }
}
