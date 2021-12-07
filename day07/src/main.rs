fn read_line_to_ints() -> Vec<i32> {
    include_str!("../input")
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve(input: &Vec<i32>) {
    let mut min_cost_1 = i32::MAX;
    let mut min_cost_2 = i32::MAX;

    for i in 0..=(*input.iter().max().unwrap()) {
        let mut cost_1 = 0;
        let mut cost_2 = 0;

        for num in input {
            cost_1 += (num - i).abs();

            let dist = (num - i).abs();
            cost_2 += (dist * (dist + 1)) / 2;
        }

        if cost_1 < min_cost_1 {
            min_cost_1 = cost_1;
        }

        if cost_2 < min_cost_2 {
            min_cost_2 = cost_2;
        }
    }

    println!("Part 1: {}", min_cost_1);
    println!("Part 2: {}", min_cost_2);
}

fn main() {
    let input = read_line_to_ints();

    solve(&input);
}
