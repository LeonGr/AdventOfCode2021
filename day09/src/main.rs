use std::{
    // cmp::{max, min},
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

fn parse(input: &Vec<String>) -> Vec<Vec<i8>> {
    let parsed: Vec<Vec<i8>> = input
        .iter()
        .map(|s| {
            s.split("")
                .filter(|c| !c.is_empty())
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();

    parsed
}

fn is_low(input: &Vec<Vec<i8>>, i: usize, j: usize) -> bool {
    let displacements: Vec<i8> = vec![-1, 0, 1];

    let mut lower = 0;

    let rows = input.len() as i32;
    let cols = input[0].len() as i32;

    for dx in &displacements {
        for dy in &displacements {
            if *dx == 0 && *dy == 0 {
                continue;
            }

            let x = (j as i32) + (*dx as i32);
            let y = (i as i32) + (*dy as i32);
            if x >= 0 && x < cols && y >= 0 && y < rows {
                if input[y as usize][x as usize] < input[i][j] {
                    lower += 1
                }
            }
        }
    }

    lower == 0
}

fn part1(input: &Vec<Vec<i8>>) {
    let mut total = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if is_low(input, i, j) {
                let current = input[i][j] as i32;
                let risk_level = current + 1;
                total += risk_level;
            }
        }
    }

    println!("Part 1: {}", total);
}

fn find_lower_neighbour(input: &Vec<Vec<i8>>, i: usize, j: usize) -> (usize, usize) {
    let displacements: Vec<i8> = vec![-1, 0, 1];

    let rows = input.len() as i32;
    let cols = input[0].len() as i32;

    for dx in &displacements {
        for dy in &displacements {
            if (*dx == 0 && *dy == 0) || !(*dx == 0 || *dy == 0) {
                continue;
            }

            let x = (j as i32) + (*dx as i32);
            let y = (i as i32) + (*dy as i32);
            if x >= 0 && x < cols && y >= 0 && y < rows {
                if input[y as usize][x as usize] < input[i][j] {
                    return (x as usize, y as usize);
                }
            }
        }
    }

    (i, j)
}

fn find_basin(input: &Vec<Vec<i8>>, i: usize, j: usize) -> (usize, usize) {
    if is_low(input, i, j) {
        return (i, j);
    } else {
        let (x, y) = find_lower_neighbour(input, i, j);
        return find_basin(input, y, x);
    }
}

fn part2(input: &Vec<Vec<i8>>) {
    let mut basins: HashMap<(usize, usize), i32> = HashMap::new();

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == 9 {
                continue;
            }
            let basin = find_basin(input, i, j);
            let element = basins.entry(basin).or_insert(0);
            *element += 1;
        }
    }

    let mut three_largest = vec![0; 3];

    for (_, count) in &basins {
        let min = three_largest.iter().min().unwrap();
        if count > min {
            three_largest.remove(0);
            three_largest.push(*count);
            three_largest.sort();
        }
    }

    println!("Part 2: {}", three_largest.iter().product::<i32>());
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;
    let parsed = parse(&lines);

    part1(&parsed);
    part2(&parsed);

    Ok(())
}
