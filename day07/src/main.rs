fn read_line_to_ints() -> Vec<i32> {
    include_str!("../input")
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part1(input: &Vec<i32>) -> i32 {
    let mut min_cost = i32::MAX;

    for i in 0..=(*input.iter().max().unwrap()) {
        let mut cost = 0;
        for num in input {
            cost += (num - i).abs()
        }
        if cost < min_cost {
            min_cost = cost;
        }
    }

    min_cost
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut min_cost = i32::MAX;

    for i in 0..=(*input.iter().max().unwrap()) {
        let mut cost = 0;
        for num in input {
            let dist = (num - i).abs();
            cost += (dist * (dist + 1)) / 2;
        }
        if cost < min_cost {
            min_cost = cost;
        }
    }

    min_cost
}

fn main() {
    let input = read_line_to_ints();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
