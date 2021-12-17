fn read_input() -> &'static str {
    include_str!("../input")
        .trim()
}

type Range = (i32, i32);

fn parse(input: &str) -> (Range, Range) {
    let ranges: Vec<Vec<i32>> = input
        .replace("target area: ", "")
        .split(", ")
        .map(|range| {
            range
                .get(2..)
                .unwrap()
                .split("..")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let x = &ranges[0];
    let y = &ranges[1];

    ((x[0], x[1]), (y[0], y[1]))
}

fn part1(input: &(Range, Range)) {
    let (x_range, y_range) = input;

    let mut max_y = 0;

    for ax in 0..100 {
        for ay in -1000..1000 {
            let mut vx = ax;
            let mut vy = ay;

            let mut x = 0;
            let mut y = 0;

            let mut in_area = false;

            let mut local_max_y = max_y;
            loop {
                x += vx;
                y += vy;

                if y > local_max_y {
                    local_max_y = y;
                }

                if (x_range.0 <= x && x <= x_range.1) && (y_range.0 <= y && y <= y_range.1) {
                    in_area = true;
                    break;
                }
                else if x > x_range.1 || y < y_range.0 {
                    break;
                }

                if vx > 0 {
                    vx -= 1;
                } else if vx < 0 {
                    vx += 1;
                }

                vy -= 1;
            }

            if in_area {
                max_y = local_max_y;
            }
        }
    }

    println!("Part 1: {}", max_y);
}

fn part2(input: &(Range, Range)) {
    let (x_range, y_range) = input;

    let mut velocities = vec![];

    for ax in 0..100 {
        for ay in -1000..1000 {
            let mut vx = ax;
            let mut vy = ay;

            let mut x = 0;
            let mut y = 0;

            let mut in_area = false;

            loop {
                x += vx;
                y += vy;

                if (x_range.0 <= x && x <= x_range.1) && (y_range.0 <= y && y <= y_range.1) {
                    in_area = true;
                    break;
                }
                else if x > x_range.1 || y < y_range.0 {
                    break;
                }

                if vx > 0 {
                    vx -= 1;
                } else if vx < 0 {
                    vx += 1;
                }

                vy -= 1;
            }

            if in_area {
                velocities.push((ax, ay));
            }
        }
    }

    println!("Part 2: {}", velocities.len());
}

fn main() {
    let input = read_input();
    let parsed = parse(&input);

    part1(&parsed);
    part2(&parsed);
}
