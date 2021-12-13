use std::{cmp::max, collections::HashSet, io::BufRead};

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

type Paper = HashSet<(i32, i32)>;

#[derive(Debug)]
enum Instruction {
    Horizontal(i32),
    Vertical(i32),
}

type Instructions = Vec<Instruction>;

fn parse(input: &Vec<String>) -> (Paper, Instructions) {
    let mut dots = HashSet::new();

    let mut n = 0;
    for i in 0..input.len() {
        let line = &input[i];

        if line.is_empty() {
            n = i + 1;
            break;
        }

        let coords: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();

        dots.insert((coords[0], coords[1]));
    }

    let mut instructions = vec![];

    for i in n..input.len() {
        let line = &input[i].replace("fold along ", "");

        let parts: Vec<&str> = line.split("=").collect();

        let dir = parts[0];
        let at = parts[1].parse().unwrap();

        let instruction = {
            match dir {
                "x" => Instruction::Horizontal(at),
                "y" => Instruction::Vertical(at),
                _ => unreachable!(),
            }
        };

        instructions.push(instruction);
    }

    (dots, instructions)
}

fn fold(paper: &Paper, instruction: &Instruction) -> Paper {
    let mut new_paper: Paper = HashSet::new();

    for (dot_x, dot_y) in paper {
        match instruction {
            Instruction::Horizontal(x) => {
                if dot_x < x {
                    new_paper.insert((*dot_x, *dot_y));
                } else {
                    let distance = (x - dot_x).abs();
                    new_paper.insert((x - distance, *dot_y));
                }
            }
            Instruction::Vertical(y) => {
                if dot_y < y {
                    new_paper.insert((*dot_x, *dot_y));
                } else {
                    let distance = (y - dot_y).abs();
                    new_paper.insert((*dot_x, y - distance));
                }
            }
        }
    }

    new_paper
}

fn part1(input: &(Paper, Instructions)) {
    let (paper, instructions) = input;
    let first_instruction = instructions.first().unwrap();

    let new_paper = fold(paper, first_instruction);

    println!("Part 1: {}", new_paper.len());
}

fn part2(input: &(Paper, Instructions)) {
    let (paper, instructions) = input;

    let mut new_paper: HashSet<(i32, i32)> = paper.clone();
    for instruction in instructions {
        new_paper = fold(&new_paper, instruction);
    }

    let (max_x, max_y) = new_paper.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
        let new_x = max(max_x, *x);
        let new_y = max(max_y, *y);

        (new_x, new_y)
    });

    let mut paper = vec![vec![" "; (max_x + 1) as usize]; (max_y + 1) as usize];

    for (x, y) in &new_paper {
        paper[*y as usize][*x as usize] = "#";
    }

    println!("Part 2:");
    for row in paper {
        println!("{:?}", row.concat());
    }
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;
    let parsed = parse(&lines);

    part1(&parsed);
    part2(&parsed);

    Ok(())
}
