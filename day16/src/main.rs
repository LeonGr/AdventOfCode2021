fn read_input() -> Vec<&'static str> {
    include_str!("../input")
        .trim()
        .split("")
        .filter(|c| !c.is_empty())
        .collect()
}

fn parse(input: &Vec<&str>) -> Vec<String> {
    input
        .iter()
        .map(|hex| u64::from_str_radix(hex, 16).unwrap())
        .fold(String::new(), |acc, dec| {
            let binary = format!("{:04b}", dec);
            acc + &binary
        })
        .split("")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

#[derive(Debug)]
enum Content {
    Literal(u64),
    SubPackets(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u64,
    type_id: u64,
    content: Content,
}

fn binary_string_to_u64(input: &String) -> u64 {
    u64::from_str_radix(input.as_str(), 2).unwrap()
}

fn parse_packets(input: &Vec<String>, number: Option<usize>) -> (Vec<Packet>, usize) {
    let mut packets: Vec<Packet> = vec![];

    let mut index = 0;
    loop {
        let mut trailing_zeroes = true;
        for d in index..input.len() {
            if input[d] != "0" {
                trailing_zeroes = false;
                break;
            }
        }

        if let Some(n) = number {
            if n == packets.len() {
                break;
            }
        }

        if trailing_zeroes || index > input.len() {
            break;
        }

        let version = binary_string_to_u64(&input[index..index + 3].concat());
        let type_id = binary_string_to_u64(&input[index + 3..index + 3 + 3].concat());

        let content = match type_id {
            4 => {
                let mut content = String::new();

                let mut literal_index = index + 6;
                loop {
                    let start = &input[literal_index];

                    let number_bits = &input[literal_index + 1..literal_index + 1 + 4];
                    content += &number_bits.concat();

                    literal_index += 5;

                    if start.as_str() == "0" {
                        break;
                    }
                }

                index = literal_index;

                Content::Literal(binary_string_to_u64(&content))
            }
            _ => {
                let start_bit = input[index + 6].as_str();

                let (sub_packets, index_increment) = {
                    let start_number = index + 6 + 1;

                    let bits = match start_bit {
                        "0" => 15,
                        "1" => 11,
                        _ => unreachable!(),
                    };

                    let bit_range = &input[start_number..start_number + bits];
                    let number = binary_string_to_u64(&bit_range.concat()) as usize;

                    match start_bit {
                        "0" => {
                            let sub_input =
                                &input[start_number + bits..start_number + bits + number].to_vec();
                            let (sub_packets, _) = parse_packets(sub_input, None);

                            (sub_packets, start_number + bits + number)
                        }
                        "1" => {
                            let sub_input = &input[start_number + bits..].to_vec();
                            let (sub_packets, final_index) = parse_packets(sub_input, Some(number));

                            (sub_packets, start_number + bits + final_index)
                        }
                        _ => unreachable!(),
                    }
                };

                index = index_increment;

                Content::SubPackets(sub_packets)
            }
        };

        let packet = Packet { version, type_id, content };

        packets.push(packet);
    }

    (packets, index)
}

fn get_version_sum(packets: &Vec<Packet>) -> u64 {
    packets
        .iter()
        .map(|packet| {
            (match &packet.content {
                Content::Literal(_) => 0,
                Content::SubPackets(sub_packets) => get_version_sum(sub_packets),
            }) + packet.version
        })
        .sum()
}

fn part1(input: &Vec<String>) {
    let (packets, _) = parse_packets(input, None);
    println!("Part 1: {:?}", get_version_sum(&packets));
}

fn evaluate(packet: &Packet) -> u64 {
    match &packet.content {
        Content::Literal(value) => *value,
        Content::SubPackets(subpackets) => {
            let mut map = subpackets.iter().map(|packet| evaluate(packet));
            match packet.type_id {
                0 => map.sum(),
                1 => map.product(),
                2 => map.min().unwrap(),
                3 => map.max().unwrap(),
                5 => (map.next().unwrap() > map.next().unwrap()) as u64,
                6 => (map.next().unwrap() < map.next().unwrap()) as u64,
                7 => (map.next().unwrap() == map.next().unwrap()) as u64,
                _ => unreachable!(),
            }
        },
    }
}

fn part2(input: &Vec<String>) {
    let (packets, _) = parse_packets(input, None);
    println!("Part 2: {:?}", evaluate(&packets[0]));
}

fn main() {
    let input = read_input();
    let parsed = parse(&input);

    part1(&parsed);
    part2(&parsed);
}
