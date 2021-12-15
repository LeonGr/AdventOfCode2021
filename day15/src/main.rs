use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
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

fn parse(input: &[String]) -> Vec<Vec<usize>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

type Pos = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    distance: usize,
    position: Pos,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &Vec<Vec<usize>>) -> Vec<Pos> {
    let rows = graph.len();
    let cols = graph[0].len();
    let source = (0, 0);
    let target = (rows - 1, cols - 1);

    let mut prev: HashMap<Pos, Pos> = HashMap::new();
    let mut dist: HashMap<Pos, usize> = HashMap::new();
    let mut q = BinaryHeap::new();

    for i in 0..rows {
        for j in 0..rows {
            if i == 0 && j == 0 {
                continue;
            }

            dist.insert((i, j), usize::MAX);
            q.push(Node {
                distance: usize::MAX,
                position: (i, j),
            })
        }
    }

    dist.insert(source, 0);
    q.push(Node {
        distance: 0,
        position: (0, 0),
    });

    let displacements: Vec<i8> = vec![-1, 0, 1];

    while let Some(Node { distance, position }) = q.pop() {
        if position == target {
            let mut path: Vec<Pos> = vec![];
            let mut position = target;

            while position != source {
                path.push(position);
                position = *prev.get(&position).unwrap();
            }

            path.reverse();
            return path;
        }

        for dx in &displacements {
            for dy in &displacements {
                if (*dx == 0 && *dy == 0) || !(*dx == 0 || *dy == 0) {
                    continue;
                }

                let x = (position.0 as i32) + (*dx as i32);
                let y = (position.1 as i32) + (*dy as i32);
                if x >= 0 && x < (cols as i32) && y >= 0 && y < (rows as i32) {
                    let x = x as usize;
                    let y = y as usize;

                    let neighbour = (x, y);

                    let alt = distance + graph[y][x];

                    if alt < *dist.get(&neighbour).unwrap() {
                        q.push(Node {
                            distance: alt,
                            position: neighbour,
                        });
                        dist.insert(neighbour, alt);
                        prev.insert(neighbour, position);
                    }
                }
            }
        }
    }

    unreachable!();
}

fn part1(input: &Vec<Vec<usize>>) {
    let path = dijkstra(input);
    println!("{:?}", path);
    let mut cost = 0;
    for node in path {
        cost += input[node.1][node.0];
    }

    println!("Part 1: {}", cost);
}

fn part2(input: &Vec<Vec<usize>>) {
    let rows = input.len();
    let cols = input[0].len();
    let mut big_graph = vec![vec![0; cols * 5]; rows * 5];

    for i in 0..rows {
        for j in 0..cols {
            for n in 0..5 {
                for p in 0..5 {
                    let first = n * rows + i;
                    let second = p * cols + j;

                    let new_value = input[i][j] + (1 * (n + p)) % 10;
                    if new_value > 9 {
                        big_graph[first][second] = new_value % 10 + 1;
                    } else {
                        big_graph[first][second] = new_value;
                    }
                }
            }
        }
    }

    let path = dijkstra(&big_graph);
    let mut cost = 0;
    for node in path {
        cost += big_graph[node.1][node.0];
    }

    println!("Part 2: {}", cost);
}

fn main() {
    let lines = read_input_lines().unwrap();
    let parsed = parse(&lines);

    part1(&parsed);
    part2(&parsed);
}
