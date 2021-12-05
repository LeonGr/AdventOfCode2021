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

type Coordinate = (i32, i32);
type Line = (Coordinate, Coordinate);

fn parse(input: &[String]) -> Vec<Line> {
    input
        .iter()
        .map(|line| {
            let coords = line
                .split(" -> ")
                .map(|coord| {
                    let parsed: Vec<i32> = coord
                        .split(',')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect();

                    (parsed[0], parsed[1])
                })
                .collect::<Vec<Coordinate>>();

            (coords[0], coords[1])
        })
        .collect()
}

fn solve(input: &Vec<Line>, diagonal: bool) -> i32 {
    let mut fields: HashMap<Coordinate, i32> = HashMap::new();

    for line in input {
        let ((s_x, s_y), (e_x, e_y)) = *line;

        for x in min(s_x, e_x)..=max(s_x, e_x) {
            for y in min(s_y, e_y)..=max(s_y, e_y) {
                if s_x == e_x || s_y == e_y {
                    let vents = fields.entry((x, y)).or_insert(0);
                    *vents += 1;
                } else if diagonal {
                    if (s_x - x).pow(2) == (s_y - y).pow(2) {
                        let vents = fields.entry((x, y)).or_insert(0);
                        *vents += 1;
                    }
                }
            }
        }
    }

    fields
        .iter()
        .fold(0, |acc, (_, value)| if *value >= 2 { acc + 1 } else { acc })
}

fn part1(input: &Vec<Line>) {
    println!("Part 1: {}", solve(input, false));
}

fn part2(input: &Vec<Line>) {
    println!("Part 2: {}", solve(input, true));
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;
    let parsed = parse(&lines);

    part1(&parsed);
    part2(&parsed);

    Ok(())
}
