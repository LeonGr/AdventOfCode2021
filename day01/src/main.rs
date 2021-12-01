use std::io;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;

fn read_lines_to_int() -> io::Result<Vec<i32>> {
    let input_file = fs::File::open("input")?;
    let file_reader = BufReader::new(input_file);

    Ok(file_reader
       .lines()
       .filter_map(io::Result::ok)
       .map(|string| string.parse::<i32>().unwrap())
       .collect())
}

fn part1(input: &Vec<i32>) -> i32 {
    let mut increments = -1;
    let mut previous = 0;

    for line in input {
        if *line > previous {
            increments += 1;
        }

        previous = *line;
    }

    increments
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut increments = -1;
    let mut previous = 0;

    for i in 2..input.len() {
        let sum = input[i] + input[i-1] + input[i-2];

        if sum > previous {
            increments += 1;
        }

        previous = sum;
    }

    increments
}

fn main() -> io::Result<()> {
    let input = read_lines_to_int()?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}
