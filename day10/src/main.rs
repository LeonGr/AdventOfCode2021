use std::{
    collections::HashMap,
    collections::HashSet,
    io::BufRead, iter::FromIterator,
};

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn parse(input: &[String]) -> Vec<Vec<char>> {
    input
        .iter()
        .map(|l| l.chars().collect())
        .collect()
}

fn part1(input: &Vec<Vec<char>>) {
    let characters: HashMap<char, (char, u64)> = HashMap::from_iter([
        (')', ('(', 3)),
        (']', ('[', 57)),
        ('}', ('{', 1197)),
        ('>', ('<', 25137)),
    ]);

    let mut total = 0;
    let closing: HashSet<&char> = characters.keys().collect();

    for line in input {
        let mut stack: Vec<char> = vec![];

        for c in line {
            if !closing.contains(c) {
                stack.push(*c);
            } else {
                match stack.pop() {
                    None => unreachable!(),
                    Some(s) => {
                        let (opening, score) = characters.get(&c).unwrap();
                        if *opening != s {
                            total += score;
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", total);
}

fn part2(input: &Vec<Vec<char>>) {
    let characters: HashMap<char, (char, u64)> = HashMap::from_iter([
        (')', ('(', 3)),
        (']', ('[', 57)),
        ('}', ('{', 1197)),
        ('>', ('<', 25137)),
    ]);

    let closing: HashSet<&char> = characters.keys().collect();

    let valid: Vec<&Vec<char>> = input
        .iter()
        .filter(|line| {
            let mut stack: Vec<char> = vec![];

            for c in *line {
                if !closing.contains(c) {
                    stack.push(*c);
                } else {
                    match stack.pop() {
                        None => unreachable!(),
                        Some(s) => {
                            let (opening, _) = characters.get(&c).unwrap();
                            if *opening != s {
                                return false;
                            }
                        }
                    }
                }
            }

            return true;
        })
        .collect();

    let values: HashMap<char, u64> = HashMap::from_iter([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);

    let mut scores: Vec<u64> = valid
        .iter()
        .map(|line| {
            let mut stack: Vec<char> = vec![];

            for c in *line {
                if !closing.contains(c) {
                    stack.push(*c);
                } else {
                    match stack.pop() {
                        None => unreachable!(),
                        Some(_) => {
                            continue;
                        }
                    }
                }
            }

            stack
        })
        .map(|stack| {
            let score = stack
                .iter()
                .rev()
                .fold(0, |acc, b| {
                    acc * 5 + (values.get(b).unwrap())
                });

            score
        })
        .collect();

    scores.sort();

    let middle = scores[(scores.len() as f64 / 2f64).floor() as usize];

    println!("Part 2: {}", middle);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;
    let parsed = parse(&lines);

    part1(&parsed);
    part2(&parsed);

    Ok(())
}
