fn takedig(bits: &mut Vec<char>, n: usize) -> u64 {
    let mut tmp = Vec::new();
    for _ in 0..n {
        tmp.push(bits.pop().unwrap());
    }
    tmp.iter().fold(0, |acc, c| { let acc = acc << 1; acc + c.to_digit(2).unwrap() }).into()
}

fn parse_packets(bits: &mut Vec<char>) -> (u64, u64) {
    let version = takedig(bits, 3);
    let type_id = takedig(bits, 3);

    let mut version_sum = version;
    let res = match type_id {
        4 => {
            let mut val = 0;
            while takedig(bits, 1) == 1 {
                val <<= 4;
                val += takedig(bits, 4);
            }
            val <<= 4;
            val += takedig(bits, 4);
            val
        },
        _ => {
            let length_type_id = takedig(bits, 1);
            let mut values: Vec<u64> = Vec::new();
            if length_type_id == 0 {
                let length = takedig(bits, 15);
                let pre = bits.len();
                while bits.len() > pre - length as usize {
                    let (ver, val) = parse_packets(bits);
                    version_sum += ver;
                    values.push(val);
                }
            } else {
                let n_sub_packets = takedig(bits, 11);
                for _ in 0..n_sub_packets {
                    let (ver, val) = parse_packets(bits);
                    version_sum += ver;
                    values.push(val);
                }
            }

            assert!(!values.is_empty());
            if [5, 6, 7].contains(&type_id) {
                assert_eq!(2, values.len());
            }

            match type_id {
                0 => values.iter().sum(),
                1 => values.iter().product(),
                2 => *values.iter().min().unwrap(),
                3 => *values.iter().max().unwrap(),
                5 => if values[0] > values[1] { 1 } else { 0 },
                6 => if values[0] < values[1] { 1 } else { 0 },
                7 => if values[0] == values[1] { 1 } else { 0 },
                _ => panic!(),
            }
        },
    };
    (version_sum, res)
}

fn hex2bin(input: &str) -> Vec<char> {
    let mut res = Vec::new();
    for c in input.chars() {
        let bits = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => continue,
        };
        for c in bits.chars() {
            res.push(c);
        }
    }
    res.reverse();
    res
}

fn main() {
    let contents = std::fs::read_to_string("day16.input").unwrap();

    let input: Vec<&str> = contents.split('\n').collect();
    let mut bits = hex2bin(input[0]);

    let (version_sum, res) = parse_packets(&mut bits);
    println!("1: {}", version_sum);
    println!("2: {}", res);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1_1() {
        let (version_sum, _) = parse_packets(&mut hex2bin("8A004A801A8002F478"));
        assert_eq!(16, version_sum);
    }

    #[test]
    fn test1_2() {
        let (version_sum, _) = parse_packets(&mut hex2bin("620080001611562C8802118E34"));
        assert_eq!(12, version_sum);
    }
    #[test]
    fn test1_3() {
        let (version_sum, _) = parse_packets(&mut hex2bin("C0015000016115A2E0802F182340"));
        assert_eq!(23, version_sum);
    }

    #[test]
    fn test1_4() {
        let (version_sum, _) = parse_packets(&mut hex2bin("A0016C880162017C3686B18A3D4780"));
        assert_eq!(31, version_sum);
    }

    #[test]
    fn test2_1() {
        let (_, res) = parse_packets(&mut hex2bin("C200B40A82"));
        assert_eq!(3, res);

    }

    #[test]
    fn test2_2() {
        let (_, res) = parse_packets(&mut hex2bin("04005AC33890"));
        assert_eq!(54, res);

    }

    #[test]
    fn test2_3() {
        let (_, res) = parse_packets(&mut hex2bin("880086C3E88112"));
        assert_eq!(7, res);

    }

    #[test]
    fn test2_4() {
        let (_, res) = parse_packets(&mut hex2bin("CE00C43D881120"));
        assert_eq!(9, res);

    }

    #[test]
    fn test2_5() {
        let (_, res) = parse_packets(&mut hex2bin("D8005AC2A8F0"));
        assert_eq!(1, res);

    }

    #[test]
    fn test2_6() {
        let (_, res) = parse_packets(&mut hex2bin("F600BC2D8F"));
        assert_eq!(0, res);

    }

    #[test]
    fn test2_7() {
        let (_, res) = parse_packets(&mut hex2bin("9C005AC2F8F0"));
        assert_eq!(0, res);

    }

    #[test]
    fn test2_8() {
        let (_, res) = parse_packets(&mut hex2bin("9C0141080250320F1802104A08"));
        assert_eq!(1, res);

    }
}
