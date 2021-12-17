pub type Input = Vec<u8>;
pub type InputRef<'a> = &'a [u8];

enum Packet {
    Operator(PacketOperator),
    LiteralValue(PacketLiteralValue),
}

enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equal,
}

struct PacketOperator {
    version: u8,
    packet_type: PacketType,
    sub_packets: Vec<Packet>,
}

struct PacketLiteralValue {
    version: u8,
    value: u128,
}

pub fn solution1(input: InputRef) -> u128 {
    let (packet, _) = parse_packet(0, input);
    sum_versions(&packet)
}

pub fn solution2(input: InputRef) -> u128 {
    let (packet, _) = parse_packet(0, input);
    evaluate(&packet)
}

fn evaluate(packet: &Packet) -> u128 {
    match packet {
        Packet::LiteralValue(p) => p.value,
        Packet::Operator(p) => match p.packet_type {
            PacketType::Sum => p.sub_packets.iter().map(evaluate).sum(),
            PacketType::Product => p.sub_packets.iter().map(evaluate).product(),
            PacketType::Minimum => p.sub_packets.iter().map(evaluate).min().unwrap(),
            PacketType::Maximum => p.sub_packets.iter().map(evaluate).max().unwrap(),
            PacketType::Greater => {
                let values: Vec<u128> = p.sub_packets.iter().map(evaluate).collect();
                assert_eq!(values.len(), 2);
                (values[0] > values[1]) as u128
            },
            PacketType::Less => {
                let values: Vec<u128> = p.sub_packets.iter().map(evaluate).collect();
                assert_eq!(values.len(), 2);
                (values[0] < values[1]) as u128
            },
            PacketType::Equal => {
                let values = p.sub_packets.iter().map(evaluate).collect::<Vec<u128>>();
                assert_eq!(values.len(), 2);
                (values[0] == values[1]) as u128
            },
        }
    }
}

fn sum_versions(packet: &Packet) -> u128 {
    match packet {
        Packet::Operator(p) => p.version as u128 + p.sub_packets.iter().map(sum_versions).sum::<u128>(),
        Packet::LiteralValue(p) => p.version.into(),
    }
}

fn parse_packet(idx: usize, input: InputRef) -> (Packet, usize) {
    let mut idx = idx;

    let version = get_nth_bits(idx, 3, input);
    idx += 3;
    let packet_type = get_nth_bits(idx, 3, input) as u8;
    idx += 3;

    if packet_type == 4 {
        let (value, idx) = parse_literal_value(idx, input);
        let packet = Packet::LiteralValue(PacketLiteralValue {
            version: version as u8,
            value,
        });
        (packet, idx)
    } else {
        let length_identifier = get_nth_bits(idx, 1, input);
        idx += 1;
        let mut sub_packets = vec![];

        match length_identifier {
            1 => {
                let number_of_packets = get_nth_bits(idx, 11, input);
                idx += 11;

                for _ in 0..number_of_packets {
                    let (packet, new_idx) = parse_packet(idx, input);
                    idx = new_idx;
                    sub_packets.push(packet);
                }
            },
            0 => {
                let total_length = get_nth_bits(idx, 15, input) as usize;
                idx += 15;
                let starting_idx = idx;

                while idx - starting_idx < total_length {
                    let (packet, new_idx) = parse_packet(idx, input);
                    idx = new_idx;
                    sub_packets.push(packet);
                }
            },
            _ => panic!("this shouldnt happen, NEVER!"),
        }

        let packet = Packet::Operator(PacketOperator {
            version: version as u8,
            packet_type: parse_packet_type(packet_type),
            sub_packets,
        });
        (packet, idx)
    }
}

fn parse_packet_type(packet_type: u8) -> PacketType {
    match packet_type {
        0 => PacketType::Sum,
        1 => PacketType::Product,
        2 => PacketType::Minimum,
        3 => PacketType::Maximum,
        5 => PacketType::Greater,
        6 => PacketType::Less,
        7 => PacketType::Equal,
        _ => panic!("je tam tma!")
    }
}

fn parse_literal_value(idx: usize, input: InputRef) -> (u128, usize) {
    let mut idx = idx;
    let mut values = vec![];

    loop {
        let bits = get_nth_bits(idx, 5, input);
        idx += 5;
        values.push(bits & 0b01111);

        if bits & 0b10000 == 0 {
            break
        }
    }

    (values.iter().fold(0, |acc, i| (acc << 4) + i), idx)
}

#[test]
fn test_parse_literal_value() {
    let input = vec![0b10111111, 0b10001010, 0b0];
    assert_eq!(parse_literal_value(0, &input), (2021, 15))
}

fn get_nth_bits(position: usize, size: usize, input: InputRef) -> u128 {
    let input_position = position / 8;
    let bit_position = position % 8;
    let value = (input[input_position] & (0b11111111 >> bit_position)) as u128;

    let remaining = size as i16 - (8 - bit_position as i16);

    if remaining > 0 {
        return (value << remaining) as u128
            + get_nth_bits((input_position + 1) * 8, remaining as usize, input);
    }

    value >> -remaining
}

#[test]
fn test_get_nth_bits() {
    let params = vec![
        (vec![0b11001100, 0b10101010], 0, 3, 0b110),
        (vec![0b11001100, 0b10101010], 8, 3, 0b101),
        (vec![0b11001100, 0b10101010], 3, 8, 0b01100101),
        (vec![0b11001100, 0b10101010], 0, 16, 0b1100110010101010),
        (vec![0b10111111, 0b10001010, 0b0], 10, 5, 0b00101),
        (
            vec![0b11001100, 0b10101010, 0b11001100],
            3,
            16,
            0b0110010101010110,
        ),
    ];

    for (input, position, size, expected) in params {
        assert_eq!(get_nth_bits(position, size, &input), expected);
    }
}

pub fn parse_input(input: &str) -> Input {
    let mut numbers = input.chars().map(|i| i.to_digit(16).unwrap() as u8);
    let mut result = vec![];

    loop {
        let first = numbers.next();

        if first.is_none() {
            break;
        }

        let number = first.unwrap() << 4;

        let second = numbers.next();
        if let Some(second) = second {
            result.push(number + second);
        } else {
            result.push(number);
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_solution1_real_data() {
        let content = read_to_string("input.txt").unwrap();
        let input = parse_input(&content.trim());
        assert_eq!(solution1(&input), 860);
    }

    #[test]
    fn test_solution2_real_data() {
        let content = read_to_string("input.txt").unwrap();
        let input = parse_input(&content.trim());
        assert_eq!(solution2(&input), 470949537659);
    }
/*
    #[test]
    fn test_solution1_1() {
        let input = parse_input("8A004A801A8002F478");
        assert_eq!(solution1(&input), 16);
    }

    #[test]
    fn test_solution1_2() {
        let input = parse_input("620080001611562C8802118E34");
        assert_eq!(solution1(&input), 12);
    }

    #[test]
    fn test_solution1_3() {
        let input = parse_input("C0015000016115A2E0802F182340");
        assert_eq!(solution1(&input), 23);
    }

    #[test]
    fn test_solution1_4() {
        let input = parse_input("A0016C880162017C3686B18A3D4780");
        assert_eq!(solution1(&input), 31);
    }

    #[test]
    fn test_solution2_1() {
        let input = parse_input("C200B40A82");
        assert_eq!(solution2(&input), 3);
    }

    #[test]
    fn test_solution2_2() {
        let input = parse_input("04005AC33890");
        assert_eq!(solution2(&input), 54);
    }

    #[test]
    fn test_solution2_3() {
        let input = parse_input("880086C3E88112");
        assert_eq!(solution2(&input), 7);
    }

    #[test]
    fn test_solution2_4() {
        let input = parse_input("CE00C43D881120");
        assert_eq!(solution2(&input), 9);
    }

    #[test]
    fn test_solution2_5() {
        let input = parse_input("D8005AC2A8F0");
        assert_eq!(solution2(&input), 1);
    }

    #[test]
    fn test_solution2_6() {
        let input = parse_input("F600BC2D8F");
        assert_eq!(solution2(&input), 0);
    }

    #[test]
    fn test_solution2_7() {
        let input = parse_input("9C005AC2F8F0");
        assert_eq!(solution2(&input), 0);
    }

    #[test]
    fn test_solution2_8() {
        let input = parse_input("9C0141080250320F1802104A08");
        assert_eq!(solution2(&input), 1);
    }
*/
}
