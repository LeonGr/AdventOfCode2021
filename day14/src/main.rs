use std::{
    cmp::{max, min},
    collections::HashMap,
    io::BufRead,
};

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn parse(input: &[String]) -> (Vec<char>, HashMap<(char, char), char>) {
    let template = input[0].chars().collect();

    let map = HashMap::from_iter(input.iter().skip(2).map(|line| {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let combination: Vec<char> = parts[0].chars().collect();
        let to: Vec<char> = parts[1].chars().collect();

        ((combination[0], combination[1]), to[0])
    }));

    (template, map)
}

fn solve(
    (template, insertion_rules): &(Vec<char>, HashMap<(char, char), char>),
    days: usize,
) -> u64 {
    let mut quantities = HashMap::from_iter(insertion_rules.iter().map(|(&combi, _)| (combi, 0)));

    for i in 0..(template.len() - 1) {
        quantities.insert((template[i], template[i + 1]), 1);
    }

    for _ in 0..days {
        let mut new = HashMap::new();

        for (combi, freq) in quantities {
            if freq > 0 {
                let added_letter = insertion_rules.get(&combi).unwrap();

                *new.entry((combi.0, *added_letter)).or_insert(0) += freq;
                *new.entry((*added_letter, combi.1)).or_insert(0) += freq;
            }
        }

        quantities = new;
    }

    let mut frequencies: HashMap<char, u64> = HashMap::new();

    for ((first, _second), freq) in quantities {
        *frequencies.entry(first).or_insert(0) += freq;
    }

    *frequencies.entry(*template.last().unwrap()).or_default() += 1;

    let (min, max) = frequencies
        .iter()
        .fold((u64::MAX, 0), |(current_min, current_max), (_, freq)| {
            (min(current_min, *freq), max(current_max, *freq))
        });

    max - min
}

fn part1(input: &(Vec<char>, HashMap<(char, char), char>)) {
    println!("Part 1: {}", solve(input, 10));
}

fn part2(input: &(Vec<char>, HashMap<(char, char), char>)) {
    println!("Part 2: {}", solve(input, 40));
}

fn main() {
    let lines = read_input_lines().unwrap();
    let parsed = parse(&lines);

    part1(&parsed);
    part2(&parsed);
}
