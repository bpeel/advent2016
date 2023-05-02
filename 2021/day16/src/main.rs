mod bitvec;

use bitvec::BitVec;

#[derive(Copy, Clone, Debug)]
enum PacketIterRemaining {
    EndPos(usize),
    PacketsRemaining(usize),
}

#[derive(Clone, Debug)]
struct PacketIter<'a> {
    bv: &'a BitVec,
    pos: usize,
    stack: Vec<PacketIterRemaining>,
}

impl<'a> PacketIter<'a> {
    fn new(bv: &BitVec) -> PacketIter {
        PacketIter {
            bv,
            pos: 0,
            stack: vec![PacketIterRemaining::EndPos(bv.size())],
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
        let type_id = self.read_bits(3) as u8;

        let data = if type_id == 4 {
            PacketData::Literal(self.read_literal())
        } else if self.read_bits(1) == 0 {
            let remaining = self.read_bits(15) as usize;
            self.stack.push(PacketIterRemaining::EndPos(self.pos + remaining));
            PacketData::BitOperator(remaining)
        } else {
            let remaining = self.read_bits(11) as usize;
            self.stack.push(PacketIterRemaining::PacketsRemaining(remaining));
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
    type_id: u8,
    data: PacketData,
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = Packet;

    fn next(&mut self) -> Option<Packet> {
        while let Some(tail) = self.stack.pop() {
            match tail {
                PacketIterRemaining::EndPos(end_pos) => {
                    if self.pos < end_pos &&
                        (!self.stack.is_empty() ||
                         !self.bv.is_trailing_zeroes(self.pos))
                    {
                        self.stack.push(PacketIterRemaining::EndPos(end_pos));

                        return Some(self.read_packet());
                    }
                },
                PacketIterRemaining::PacketsRemaining(remaining) => {
                    if remaining > 0 {
                        self.stack.push(PacketIterRemaining::PacketsRemaining(
                            remaining - 1
                        ));
                        return Some(self.read_packet());
                    }
                },
            }
        }

        None
    }
}

fn process_packets(bv: &BitVec) -> u32 {
    PacketIter::new(bv).map(|p| p.version as u32).sum()
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
                        println!("{}", process_packets(&bv))
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
