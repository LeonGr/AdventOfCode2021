use std::{
    // cmp::{max, min},
    collections::{HashMap, HashSet},
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

fn parse(input: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut adjacency_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in input {
        let parts: Vec<String> = line.split("-").map(|c| c.to_string()).collect();

        let source = &parts[0];
        let target = &parts[1];

        let to = adjacency_map.entry(source.to_string()).or_default();
        (*to).push(target.to_string());

        let from = adjacency_map.entry(target.to_string()).or_default();
        (*from).push(source.to_string());
    }

    adjacency_map
}

fn is_uppercase(string: &String) -> bool {
    string.to_ascii_uppercase() == *string
}

fn can_use(string: &String, path: &Vec<String>, use_smaller_twice: bool) -> bool {
    if is_uppercase(string) {
        return true;
    } else if !path.contains(string) {
        return true;
    } else {
        if !use_smaller_twice || string.as_str() == "start" {
            return false;
        }

        let mut uniq = HashSet::new();
        return path
            .iter()
            .filter(|node| {
                !is_uppercase(node) && node.as_str() != "start" && node.as_str() != "end"
            })
            .all(move |x| uniq.insert(x));
    }
}

fn generate_paths(
    input: &HashMap<String, Vec<String>>,
    current_path: Vec<String>,
    smaller_twice: bool,
) -> i32 {
    let mut num_paths = 0;
    let last = &current_path[current_path.len() - 1];

    match input.get(last) {
        Some(reachable) => {
            for node in reachable {
                if node == "end" {
                    num_paths += 1;
                } else if can_use(node, &current_path, smaller_twice) {
                    let mut new_path = current_path.clone();
                    new_path.push(node.to_string());

                    num_paths += generate_paths(input, new_path, smaller_twice);
                }
            }
        }
        None => (),
    }

    return num_paths;
}

fn part1(input: &HashMap<String, Vec<String>>) {
    let num_paths = generate_paths(input, vec!["start".to_string()], false);

    println!("Part 1: {}", num_paths);
}

fn part2(input: &HashMap<String, Vec<String>>) {
    let paths = generate_paths(input, vec!["start".to_string()], true);

    println!("Part 2: {}", paths);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;
    let parsed = parse(&lines);

    part1(&parsed);
    part2(&parsed);

    Ok(())
}
