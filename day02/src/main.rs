use std::io::BufRead;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

enum Direction {
    Forward,
    Up,
    Down,
}

fn parse(input: &[String]) -> Vec<(Direction, i32)> {
    input
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let direction: Direction = match parts[0] {
                "forward" => Direction::Forward,
                "up" => Direction::Up,
                "down" => Direction::Down,
                _ => unreachable!(),
            };
            let distance: i32 = parts[1].parse().unwrap();
            (direction, distance)
        })
        .collect()
}

fn solve(
    input: &[(Direction, i32)],
    f: &dyn Fn((i32, i32, i32), &(Direction, i32)) -> (i32, i32, i32),
) -> i32 {
    let (hori, vert, _) = input.iter().fold((0, 0, 0), f);

    hori * vert
}

fn part1(input: &[(Direction, i32)]) {
    let f = |(hori, vert, _), (direction, distance): &(Direction, i32)| match direction {
        Direction::Forward => (hori + distance, vert, 0),
        Direction::Up => (hori, vert - distance, 0),
        Direction::Down => (hori, vert + distance, 0),
    };
    println!("Part 1: {}", solve(input, &f));
}

fn part2(input: &[(Direction, i32)]) {
    let f = |(hori, vert, aim), (direction, distance): &(Direction, i32)| match direction {
        Direction::Forward => (hori + distance, vert + (aim * distance), aim),
        Direction::Up => (hori, vert, aim - distance),
        Direction::Down => (hori, vert, aim + distance),
    };
    println!("Part 2: {}", solve(input, &f));
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;
    let parsed = parse(&lines);

    part1(&parsed);
    part2(&parsed);

    Ok(())
}
