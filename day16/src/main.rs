use std::fmt;
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

enum Contents {
    Literal(u64),
    Operator(Vec<Packet>),
}

struct Packet {
    version: u8,
    typeid: u8,
    contents: Contents,
}

impl Packet {
    fn evaluate(&self) -> u64 {
        match self.typeid {
            0 => {
                // sum
                if let Contents::Operator(sub) = &self.contents {
                    return sub.iter().map(|p| p.evaluate()).sum();
                } else {
                    panic!("typeid is 0 but no subpackets found")
                }
            }
            1 => {
                // product
                if let Contents::Operator(sub) = &self.contents {
                    return sub.iter().map(|p| p.evaluate()).product();
                } else {
                    panic!("typeid is 1 but no subpackets found")
                }
            }
            2 => {
                // min
                if let Contents::Operator(sub) = &self.contents {
                    return sub.iter().map(|p| p.evaluate()).min().unwrap();
                } else {
                    panic!("typeid is 1 but no subpackets found")
                }
            }
            3 => {
                // max
                if let Contents::Operator(sub) = &self.contents {
                    return sub.iter().map(|p| p.evaluate()).max().unwrap();
                } else {
                    panic!("typeid is 1 but no subpackets found")
                }
            }
            4 => {
                // literal
                if let Contents::Literal(value) = &self.contents {
                    return *value as u64;
                } else {
                    panic!("typeid is 4 but no literal value found")
                }
            }
            5 => {
                // greater than
                if let Contents::Operator(sub) = &self.contents {
                    return if sub[0].evaluate() > sub[1].evaluate() {
                        1
                    } else {
                        0
                    };
                } else {
                    panic!("typeid is 1 but no subpackets found")
                }
            }
            6 => {
                // less than
                if let Contents::Operator(sub) = &self.contents {
                    return if sub[0].evaluate() < sub[1].evaluate() {
                        1
                    } else {
                        0
                    };
                } else {
                    panic!("typeid is 1 but no subpackets found")
                }
            }
            7 => {
                // equals
                if let Contents::Operator(sub) = &self.contents {
                    return if sub[0].evaluate() == sub[1].evaluate() {
                        1
                    } else {
                        0
                    };
                } else {
                    panic!("typeid is 1 but no subpackets found")
                }
            }
            _ => panic!("unknown type {}", self.typeid),
        }
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "version: {}", self.version)?;
        writeln!(f, "typeid: {}", self.typeid)?;
        match &self.contents {
            Contents::Literal(v) => writeln!(f, "contents: literal({})", v),
            Contents::Operator(packets) => writeln!(f, "contents: operator[\n  {:?}\n]", packets),
        }?;
        Ok(())
    }
}

fn read_bit(bytes: &[u8], offset: usize) -> u8 {
    let byte_offset = offset / 8;
    let bit_offset = offset % 8;
    return (bytes[byte_offset] & (1 << bit_offset)) >> bit_offset;
}

fn read_bits(bytes: &[u8], offset: usize, len: usize) -> u64 {
    (0..len).fold(0, |a, i| a << 1 | read_bit(bytes, offset + i) as u64)
}

fn parse_literal(bytes: &[u8], mut offset: usize) -> (u64, usize) {
    let mut result: u64 = 0;
    loop {
        let last_bit = read_bit(bytes, offset) == 0;
        offset += 1;
        result = result << 4 | read_bits(bytes, offset, 4);
        offset += 4;
        if last_bit {
            return (result, offset);
        }
    }
}

fn parse_operator(bytes: &[u8], mut offset: usize) -> (Vec<Packet>, usize) {
    let mut operands = vec![];
    let len_type = read_bit(bytes, offset) == 0;
    offset += 1;
    if len_type {
        let len_bytes = read_bits(bytes, offset, 15);
        offset += 15;
        let subpackets_end = offset + len_bytes as usize;
        loop {
            let (packet, new_offset) = parse(bytes, offset);
            offset = new_offset;
            operands.push(packet);
            if offset == subpackets_end {
                break;
            }
        }
    } else {
        let len_packets = read_bits(bytes, offset, 11);
        offset += 11;
        for _ in 0..len_packets {
            let (packet, new_offset) = parse(bytes, offset);
            offset = new_offset;
            operands.push(packet);
        }
    }
    return (operands, offset);
}

fn parse(bytes: &[u8], mut offset: usize) -> (Packet, usize) {
    let version = read_bits(bytes, offset, 3) as u8;
    offset += 3;
    let typeid = read_bits(bytes, offset, 3) as u8;
    offset += 3;
    let contents = match typeid {
        4 => {
            let (value, new_offset) = parse_literal(bytes, offset);
            offset = new_offset;
            Contents::Literal(value)
        }
        _ => {
            let (value, new_offset) = parse_operator(bytes, offset);
            offset = new_offset;
            Contents::Operator(value)
        }
    };
    (
        Packet {
            version,
            typeid,
            contents,
        },
        offset,
    )
}

fn sum_versions(p: &Packet) -> i32 {
    return p.version as i32
        + match &p.contents {
            Contents::Literal(_) => 0,
            Contents::Operator(packets) => packets.iter().map(|p| sum_versions(p)).sum(),
        };
}

fn parse_to_bytes(line: &str) -> Vec<u8> {
    let nibbles = (0..line.len())
        .map(|o| line.get(o..o + 1).unwrap())
        .map(|s| u8::from_str_radix(s, 16).unwrap())
        .collect::<Vec<_>>();
    return nibbles
        .chunks(2)
        .map(|s| s[0] << 4 | if s.len() == 2 { s[1] } else { 0 })
        .map(|b| b.reverse_bits())
        .collect::<Vec<_>>();
}

fn part1(lines: &[&str]) -> i32 {
    let bytes = parse_to_bytes(lines[0]);

    let (packet, _) = parse(&bytes, 0);
    return sum_versions(&packet);
}

fn part2(lines: &[&str]) -> u64 {
    let bytes = parse_to_bytes(lines[0]);

    let (packet, _) = parse(&bytes, 0);
    return packet.evaluate();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        lines: Vec<&'a str>,
        part1: i32,
        part2: u64,
    }

    #[test]
    fn part_1tests() {
        let cases = vec![
            TestCase {
                lines: vec!["D2FE28"],
                part1: 6,
                part2: 0,
            },
            TestCase {
                lines: vec!["38006F45291200"],
                part1: 9,
                part2: 0,
            },
            TestCase {
                lines: vec!["EE00D40C823060"],
                part1: 14,
                part2: 0,
            },
            TestCase {
                lines: vec!["8A004A801A8002F478"],
                part1: 16,
                part2: 0,
            },
            TestCase {
                lines: vec!["620080001611562C8802118E34"],
                part1: 12,
                part2: 0,
            },
            TestCase {
                lines: vec!["C0015000016115A2E0802F182340"],
                part1: 23,
                part2: 0,
            },
            TestCase {
                lines: vec!["A0016C880162017C3686B18A3D4780"],
                part1: 31,
                part2: 0,
            },
        ];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
        }
    }

    #[test]
    fn part_2tests() {
        let cases = vec![
            TestCase {
                // C200B40A82 finds the sum of 1 and 2, resulting in the value 3.
                lines: vec!["C200B40A82"],
                part1: 0,
                part2: 3,
            },
            TestCase {
                // 04005AC33890 finds the product of 6 and 9, resulting in the value 54.
                lines: vec!["04005AC33890"],
                part1: 0,
                part2: 54,
            },
            TestCase {
                // 880086C3E88112 finds the minimum of 7, 8, and 9, resulting in the value 7.
                lines: vec!["880086C3E88112"],
                part1: 0,
                part2: 7,
            },
            TestCase {
                // CE00C43D881120 finds the maximum of 7, 8, and 9, resulting in the value 9.
                lines: vec!["CE00C43D881120"],
                part1: 0,
                part2: 9,
            },
            TestCase {
                // D8005AC2A8F0 produces 1, because 5 is less than 15.
                lines: vec!["D8005AC2A8F0"],
                part1: 0,
                part2: 1,
            },
            TestCase {
                // F600BC2D8F produces 0, because 5 is not greater than 15.
                lines: vec!["F600BC2D8F"],
                part1: 0,
                part2: 0,
            },
            TestCase {
                // 9C005AC2F8F0 produces 0, because 5 is not equal to 15.
                lines: vec!["9C005AC2F8F0"],
                part1: 0,
                part2: 0,
            },
            TestCase {
                // 9C0141080250320F1802104A08 produces 1, because 1 + 3 = 2 * 2.
                lines: vec!["9C0141080250320F1802104A08"],
                part1: 0,
                part2: 1,
            },
        ];
        for t in cases {
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }
}
