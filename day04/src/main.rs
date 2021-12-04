use std::{collections::HashSet, io::BufRead};

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

type Board = Vec<Vec<i8>>;

fn parse(input: &[String]) -> (Vec<i8>, Vec<Board>) {
    let draws = input[0]
        .split(",")
        .map(|x| x.parse::<i8>().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];

    for i in (2..input.len()).step_by(6) {
        let mut board: Board = vec![];

        for j in 0..5 {
            let line = input[i + j].split(' ');
            let numbers: Vec<i8> = line
                .filter(|&x| x.ne(""))
                .map(|x| x.parse::<i8>().unwrap())
                .collect();
            board.push(numbers);
        }

        boards.push(board);
    }

    (draws, boards)
}

fn transpose(board: &Board) -> Board {
    let mut copy = board.clone();

    for i in 0..board.len() {
        for j in 0..i {
            let temp = copy[i][j];
            copy[i][j] = copy[j][i];
            copy[j][i] = temp;
        }
    }

    copy
}

fn check_win(drawn: &HashSet<i8>, board: &Board) -> bool {
    for line in board {
        let mut win = true;
        for number in line {
            if !drawn.contains(&number) {
                win = false;
            }
        }

        if win {
            return true;
        }
    }

    false
}

fn calculate_score(drawn: &HashSet<i8>, board: &Board, last: i8) -> i32 {
    let mut sum: i32 = 0;

    for line in board {
        for number in line {
            if !drawn.contains(&number) {
                sum += *number as i32;
            }
        }
    }

    sum * (last as i32)
}

fn part1(input: &(Vec<i8>, Vec<Board>)) {
    let (draws, boards) = input;

    let mut drawn: HashSet<i8> = HashSet::new();

    'outer: for draw in draws {
        drawn.insert(*draw);

        for board in boards {
            if check_win(&drawn, &board) || check_win(&drawn, &transpose(&board)) {
                println!("Part 1: {}", calculate_score(&drawn, &board, *draw));
                break 'outer;
            }
        }
    }
}

fn part2(input: &(Vec<i8>, Vec<Board>)) {
    let (draws, boards) = input;
    let mut drawn: HashSet<i8> = HashSet::new();

    let mut wins = 0;
    let mut won: HashSet<usize> = HashSet::new();

    for draw in draws {
        drawn.insert(*draw);

        for i in 0..boards.len() {
            let board = boards[i].clone();

            if !won.contains(&i)
                && (check_win(&drawn, &board) || check_win(&drawn, &transpose(&board)))
            {
                wins += 1;
                won.insert(i);

                if wins == boards.len() {
                    println!("Part 2: {}", calculate_score(&drawn, &board, *draw));
                    return;
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;
    let parsed = parse(&lines);

    part1(&parsed);
    part2(&parsed);

    Ok(())
}
