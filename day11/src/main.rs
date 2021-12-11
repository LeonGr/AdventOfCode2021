use std::io::BufRead;

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

fn part1(input: &Vec<Vec<i8>>) {
    let mut copy = input.clone();
    let rows = copy.len();
    let cols = copy[0].len();
    let mut flashes = 0;
    let displacements: Vec<i8> = vec![-1, 0, 1];

    let steps = 100;
    for _ in 0..steps {
        // let mut current_step = copy.clone();
        // Increment energy levels
        for i in 0..rows {
            for j in 0..cols {
                copy[i][j] += 1;
            }
        }

        // let mut new = copy.clone();
        loop {
            let mut any_flashed = false;

            for i in 0..rows {
                for j in 0..cols {
                    if copy[i][j] == 0 {
                        continue;
                    }
                    if copy[i][j] > 9 {
                        // flash
                        any_flashed = true;
                        flashes += 1;
                        copy[i][j] = 0;

                        // increment neighbours
                        for dx in &displacements {
                            for dy in &displacements {
                                if *dx == 0 && *dy == 0 {
                                    continue;
                                }

                                let x = (j as i32) + (*dx as i32);
                                let y = (i as i32) + (*dy as i32);
                                if x >= 0 && x < (cols as i32) && y >= 0 && y < (rows as i32) {
                                    if copy[y as usize][x as usize] != 0 {
                                        copy[y as usize][x as usize] += 1;
                                    }
                                }
                            }
                        }

                    }
                }
            }

            if !any_flashed {
                break;
            }
        }
    }

    println!("Part 1: {}", flashes);
}

fn part2(input: &Vec<Vec<i8>>) {
    let mut copy = input.clone();
    let rows = copy.len();
    let cols = copy[0].len();
    let displacements: Vec<i8> = vec![-1, 0, 1];

    let mut steps = 0;
    loop {
        let mut all_zero = true;
        'outer: for i in 0..rows {
            for j in 0..cols {
                if copy[i][j] != 0 {
                    all_zero = false;
                    break 'outer;
                }
            }
        }
        if all_zero {
            break;
        }

        // let mut current_step = copy.clone();
        // Increment energy levels
        for i in 0..rows {
            for j in 0..cols {
                copy[i][j] += 1;
            }
        }

        // let mut new = copy.clone();
        loop {
            let mut any_flashed = false;

            for i in 0..rows {
                for j in 0..cols {
                    if copy[i][j] == 0 {
                        continue;
                    }
                    if copy[i][j] > 9 {
                        // flash
                        any_flashed = true;
                        copy[i][j] = 0;

                        // increment neighbours
                        for dx in &displacements {
                            for dy in &displacements {
                                if *dx == 0 && *dy == 0 {
                                    continue;
                                }

                                let x = (j as i32) + (*dx as i32);
                                let y = (i as i32) + (*dy as i32);
                                if x >= 0 && x < (cols as i32) && y >= 0 && y < (rows as i32) {
                                    if copy[y as usize][x as usize] != 0 {
                                        copy[y as usize][x as usize] += 1;
                                    }
                                }
                            }
                        }

                    }
                }
            }

            if !any_flashed {
                break;
            }
        }

        steps += 1;
    }

    println!("Part 2: {}", steps);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;
    let parsed = parse(&lines);

    part1(&parsed);
    part2(&parsed);

    Ok(())
}
