fn hex_message_to_binary(hex: &str) -> String {
    fn padded_string(hex_char: u32) -> String {
        let s = format!("{:b}", hex_char);
        match s.len() {
            0 => String::from("0000"),
            1 => String::from(format!("000{}", s)),
            2 => String::from(format!("00{:}", s)),
            3 => String::from(format!("0{}", s)),
            4 => String::from(s),
            _ => panic!("Unexpected hex_value")
        }
    }
    hex.chars().map(|c| padded_string(c.to_digit(16).unwrap()).chars().collect::<Vec<_>>()).flatten().collect()
}

enum PacketDetails {
    Literal {
        value: usize
    },
    Operator {
        sub_packets: Vec<Packet>
    }
}

struct Packet {
    version: usize,
    type_id: usize,
    details: PacketDetails
}

impl Packet {
    fn eval(&self) -> usize {
        match &self.details {
            PacketDetails::Literal{ value } => *value,
            PacketDetails::Operator{ sub_packets} => {
                let sub_values: Vec<_> = sub_packets.iter().map(|sp| sp.eval()).collect();
                match self.type_id {
                    0 => sub_values.iter().sum::<usize>(),
                    1 => sub_values.iter().product::<usize>(),
                    2 => *sub_values.iter().min().unwrap(),
                    3 => *sub_values.iter().max().unwrap(),
                    5 => if sub_values[0] > sub_values[1] { 1 } else { 0 },
                    6 => if sub_values[0] < sub_values[1] { 1 } else { 0 },
                    7 => if sub_values[0]  == sub_values[1] { 1 } else { 0 },
                    _ => panic!("Unexpected type_id: {}", self.type_id)
                }
            }
        }
    }

    fn version_sum(&self) -> usize {
        match &self.details {
            PacketDetails::Literal{ value: _ } => self.version,
            PacketDetails::Operator{ sub_packets} => self.version + sub_packets.iter().fold(0, |acc, sp| acc + sp.version_sum())
        }
    }
}

fn parse_packet(bits: &mut std::str::Chars) -> Packet {
    fn parse_n_bit_int(bits: &mut std::str::Chars, n_bits: usize) -> usize {
        let the_bits: String = bits.take(n_bits).collect();
        usize::from_str_radix(&the_bits, 2).unwrap()
    }

    fn parse_literal(bits: &mut std::str::Chars) -> PacketDetails {
        fn parse_five_bit_group(bits: &mut std::str::Chars) -> (Vec<char>, bool) {
            let last = bits.next().unwrap() == '0';
            (bits.take(4).collect(), last)
        }

        let mut val_bits: Vec<char> = vec!();
        loop {
            let (mut val_contrib, last) = parse_five_bit_group(bits);
            val_bits.append(& mut val_contrib);
            if last {
                break
            }
        }
        let val: String = val_bits.into_iter().collect();
        let value = usize::from_str_radix(&val, 2).unwrap();
        PacketDetails::Literal{value}
    }

    fn parse_operation(bits: &mut std::str::Chars) -> PacketDetails {
        let length_type_id = bits.next().unwrap();
        if length_type_id == '0' {
            let n_packet_bits = parse_n_bit_int(bits, 15);
            let n_remaining_at_start = bits.clone().count();
            let mut sub_packets: Vec<Packet> = vec!();
            loop {
                sub_packets.push(parse_packet(bits));
                let consumed_bits = n_remaining_at_start - bits.clone().count();
                if consumed_bits == n_packet_bits {
                    break
                }
            }
            PacketDetails::Operator{sub_packets}
        } else {
            let n_sub_packets = parse_n_bit_int(bits, 11);
            let mut sub_packets: Vec<Packet> = vec!();
            for _ in 0..n_sub_packets {
                sub_packets.push(parse_packet(bits))
            }
            PacketDetails::Operator{sub_packets}
        }
    }

    let version = parse_n_bit_int(bits, 3);
    let type_id = parse_n_bit_int(bits, 3);
    let details = if type_id == 4 {
        parse_literal(bits)
    } else {
        parse_operation(bits)
    };
    Packet {version, type_id, details}
}

fn main() {
    let binary = &hex_message_to_binary(include_str!("../input"));
    let message = parse_packet(&mut binary.chars());
    println!("Answer part 1: {}", message.version_sum());
    println!("Answer part 2: {}", message.eval());
}

mod test {
    use super::*;

    #[test]
    fn p1_1() {
        let binary = &hex_message_to_binary("D2FE28");
        assert_eq!(
            binary,
            "110100101111111000101000"
        );
        let message = parse_packet(&mut binary.chars());
        assert_eq!(
            message.version,
            6
        );
        assert_eq!(
            message.type_id,
            4
        );
        assert_eq!(
            message.eval(),
            2021
        )
    }
    #[test]
    fn p1_2() {
        let binary = &hex_message_to_binary("38006F45291200");
        assert_eq!(binary,
            "00111000000000000110111101000101001010010001001000000000"
        );
        let message = parse_packet(&mut binary.chars());
        assert_eq!(
            message.version,
            1
        );
        assert_eq!(
            message.type_id,
            6
        );
    }
    #[test]
    fn p1_3() {
        let binary = &hex_message_to_binary("EE00D40C823060");
        assert_eq!(
            binary,
            "11101110000000001101010000001100100000100011000001100000"
        );
        let message = parse_packet(&mut binary.chars());
        assert_eq!(
            message.version,
            7
        );
        assert_eq!(
            message.type_id,
            3
        );
    }

    #[test]
    fn p1_4() {
        let binary = &hex_message_to_binary("8A004A801A8002F478");
        let message = parse_packet(&mut binary.chars());
        assert_eq!(
            message.version_sum(),
            16
        )
    }

    #[test]
    fn p1_5() {
        let binary = &hex_message_to_binary("C0015000016115A2E0802F182340");
        let message = parse_packet(&mut binary.chars());
        assert_eq!(
            message.version_sum(),
            23
        )
    }

    #[test]
    fn p1_6() {
        let binary = &hex_message_to_binary("A0016C880162017C3686B18A3D4780");
        let message = parse_packet(&mut binary.chars());
        assert_eq!(
            message.version_sum(),
            31
        )
    }

    #[test]
    fn p2_1() {
        let binary = &hex_message_to_binary("C200B40A82");
        let message = parse_packet(&mut binary.chars());
        assert_eq!(
            message.eval(),
            3
        )
    }

    #[test]
    fn p2_2() {
        let binary = &hex_message_to_binary("04005AC33890");
        let message = parse_packet(&mut binary.chars());
        assert_eq!(
            message.eval(),
            54
        )
    }

    #[test]
    fn p2_3() {
        let binary = &hex_message_to_binary("880086C3E88112");
        let message = parse_packet(&mut binary.chars());
        assert_eq!(
            message.eval(),
            7
        )
    }

    #[test]
    fn p2_4() {
        let binary = &hex_message_to_binary("CE00C43D881120");
        let message = parse_packet(&mut binary.chars());
        assert_eq!(
            message.eval(),
            9
        )
    }

    #[test]
    fn p2_5() {
        let binary = &hex_message_to_binary("D8005AC2A8F0");
        let message = parse_packet(&mut binary.chars());
        assert_eq!(
            message.eval(),
            1
        )
    }

    #[test]
    fn p2_6() {
        let binary = &hex_message_to_binary("F600BC2D8F");
        let message = parse_packet(&mut binary.chars());
        assert_eq!(
            message.eval(),
            0
        )
    }

    #[test]
    fn p2_7() {
        let binary = &hex_message_to_binary("9C005AC2F8F0");
        let message = parse_packet(&mut binary.chars());
        assert_eq!(
            message.eval(),
            0
        )
    }

    #[test]
    fn p2_8() {
        let binary = &hex_message_to_binary("9C0141080250320F1802104A08");
        let message = parse_packet(&mut binary.chars());
        assert_eq!(
            message.eval(),
            1
        )
    }


}