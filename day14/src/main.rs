use std::{cmp::{ max, min }, collections::HashMap, io::BufRead};

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn parse(input: &Vec<String>) -> (Vec<char>, HashMap<(char, char), char>) {
    let template = input[0].chars().collect();

    let mut insertions = vec![];

    for i in 2..input.len() {
        let line = &input[i];

        let parts: Vec<&str> = line.split(" -> ").collect();
        let combination: Vec<char> = parts[0].chars().collect();
        let to: Vec<char> = parts[1].chars().collect();

        insertions.push(((combination[0], combination[1]), to[0]));
    }

    let map = HashMap::from_iter(insertions);

    (template, map)
}

fn part1(input: &(Vec<char>, HashMap<(char, char), char>)) {
    let (template, insertion_rules) = input;

    let mut current = template.clone();

    for _ in 0..10 {
        let mut new = vec![];
        new.reserve(current.len() * 2);

        for i in 0..(current.len() - 1) {
            new.push(current[i]);

            let pair = (current[i], current[i + 1]);

            match insertion_rules.get(&pair) {
                Some(letter) => new.push(*letter),
                None => ()
            }
        }

        new.push(current[current.len() - 1]);

        current = new;
    }

    let mut frequencies: HashMap<char, u64> = HashMap::new();

    for c in current {
        let entry = frequencies.entry(c).or_insert(0);
        *entry += 1;
    }

    let (min, max) = frequencies
        .iter()
        .fold((u64::MAX, 0), |(current_min, current_max), (_, freq)| {
            return (min(current_min, *freq), max(current_max, *freq))
        });

    println!("Part 1: {}", max - min);
}

fn part2(input: &(Vec<char>, HashMap<(char, char), char>)) {
    let (template, insertion_rules) = input;

    let mut quantities: HashMap<(char, char), u64> = HashMap::new();
    for (combi, _) in insertion_rules {
        quantities.insert(*combi, 0);
    }

    for i in 0..(template.len() - 1) {
        let combi = (template[i], template[i + 1]);

        let entry = quantities.entry(combi).or_insert(0);
        *entry += 1;
    }

    for _ in 0..40 {
        let mut new = HashMap::new();

        for (combi, freq) in quantities {
            new.entry(combi).or_insert(0);

            if freq > 0 {
                let (first, second) = combi;
                let added_letter = insertion_rules.get(&combi).unwrap();

                let first_added = new.entry((first, *added_letter)).or_insert(0);
                *first_added += freq;

                let second_added = new.entry((*added_letter, second)).or_insert(0);
                *second_added += freq;
            }
        }

        quantities = new;
    }


    let mut frequencies: HashMap<char, u64> = HashMap::new();

    for ((first, second), freq) in quantities {
        let first_entry = frequencies.entry(first).or_insert(0);
        *first_entry += freq;

        let second_entry = frequencies.entry(second).or_insert(0);
        *second_entry += freq;
    }

    let (min, max) = frequencies
        .iter()
        .fold((u64::MAX, 0), |(current_min, current_max), (_, freq)| {
            return (min(current_min, *freq), max(current_max, *freq))
        });

    println!("Part 2: {}", (max - min) / 2);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;
    let parsed = parse(&lines);

    part1(&parsed);
    part2(&parsed);

    Ok(())
}
