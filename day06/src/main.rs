use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn read_line_to_ints() -> io::Result<Vec<i64>> {
    let input_file = fs::File::open("input")?;
    let file_reader = BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(io::Result::ok)
        .collect::<Vec<String>>()[0]
        .split(',')
        .map(|string| string.parse::<i64>().unwrap())
        .collect())
}

fn solve(input: &Vec<i64>, days: i64) -> i64 {
    let mut counts = vec![0; 9];
    let mut new_counts = vec![0; 9];

    for &fish in input {
        counts[fish as usize] += 1;
    }

    for _ in 0..days {
        for i in (0..=8).rev() {
            new_counts[i] = counts[(i + 1) % 9];

            if i == 6 {
                new_counts[i] += counts[0];
            }
        }

        counts = new_counts.clone();
    }

    counts.iter().sum()
}

fn main() -> io::Result<()> {
    let input = read_line_to_ints()?;

    println!("Part 1: {}", solve(&input, 80));
    println!("Part 2: {}", solve(&input, 256));

    Ok(())
}
