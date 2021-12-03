use std::io::BufRead;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn part1(input: &[String]) {
    let num_bits = input[0].len();
    let mut counts = vec![(0, 0); num_bits];

    for line in input {
        let chars: Vec<&str> = line.split("").collect();
        for i in 0..num_bits {
            match chars[i + 1] {
                "0" => {
                    let (zeros, ones) = counts[i];
                    counts[i] = (zeros + 1, ones);
                }
                "1" => {
                    let (zeros, ones) = counts[i];
                    counts[i] = (zeros, ones + 1);
                }
                _ => (),
            }
        }
    }

    let (gamma, epsilon) =
        counts
            .iter()
            .enumerate()
            .fold((0, 0), |(gamma, epsilon), (i, (zeros, ones))| {
                let power = (num_bits - i - 1) as u32;

                if zeros > ones {
                    (gamma, epsilon + 2i32.pow(power))
                } else {
                    (gamma + 2i32.pow(power), epsilon)
                }
            });

    println!("Part 1: {}", gamma * epsilon);
}

fn part2(input: &[String]) {
    let find_result = |most_common: bool| {
        let mut copy: Vec<String> = input.to_vec();
        let mut result = String::new();

        for i in 0..(input[0].len()) {
            let (count_zero, count_one) =
                copy.iter().fold((0, 0), |(zeros, ones), line| {
                    match line.chars().nth(i).unwrap() {
                        '0' => (zeros + 1, ones),
                        '1' => (zeros, ones + 1),
                        _ => unreachable!(),
                    }
                });

            copy.retain(|line| {
                line.chars().nth(i).unwrap()
                    == char::from_digit(!((count_one >= count_zero) ^ most_common) as u32, 10)
                        .unwrap()
            });

            if copy.len() == 1 {
                result = copy.pop().unwrap();
            }
        }

        isize::from_str_radix(&result, 2).unwrap()
    };

    let oxygen = find_result(true);
    let co2 = find_result(false);

    println!("Part 2: {}", oxygen * co2);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
