use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

struct Board {
    nums: Vec<(i32, bool)>,
}

impl Board {
    fn new() -> Board {
        let mut nums = Vec::with_capacity(25);
        nums.resize(25, (0, false));
        return Board { nums };
    }

    fn mark(&mut self, num: i32) {
        for v in self.nums.iter_mut() {
            if v.0 == num {
                v.1 = true
            }
        }
    }

    fn win(&self) -> bool {
        for column in self.nums.chunks(5) {
            let all_true = column.iter().fold(true, |a, b| a && b.1);
            if all_true {
                return true;
            }
        }
        for start_col in 0..5 {
            let all_true = self.nums[start_col..self.nums.len()]
                .iter()
                .step_by(5)
                .fold(true, |a, b| a && b.1);
            if all_true {
                return true;
            }
        }
        return false;
    }

    fn score(&self, num: i32) -> i32 {
        let mut sum = 0;
        for v in self.nums.iter() {
            if !v.1 {
                sum += v.0;
            }
        }
        return sum * num;
    }
}

fn parse_boards(mut lines: &[&str]) -> Vec<Board> {
    let mut boards = vec![];

    while lines.len() > 0 {
        let mut board = Board::new();
        for i in 0..5 {
            let nums = lines[i + 1]
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            for j in 0..5 {
                board.nums[i * 5 + j].0 = nums[j]
            }
        }
        lines = &lines[6..lines.len()];
        boards.push(board)
    }

    return boards;
}

fn part1(lines: &[&str]) -> i32 {
    let draws = lines[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = parse_boards(&lines[1..lines.len()]);

    for draw in draws {
        for board in boards.iter_mut() {
            board.mark(draw);
            if board.win() {
                return board.score(draw);
            }
        }
    }
    return 0;
}

fn part2(lines: &[&str]) -> i32 {
    let draws = lines[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = parse_boards(&lines[1..lines.len()]);
    let num_boards = boards.len();
    let mut known_wins = Vec::<bool>::new();
    let mut num_wins = 0;
    known_wins.resize(boards.len(), false);

    for draw in draws {
        let mut i = 0;
        for board in boards.iter_mut() {
            board.mark(draw);
            if !known_wins[i] && board.win() {
                known_wins[i] = true;
                num_wins += 1;
                if num_wins == num_boards {
                    // last win
                    return board.score(draw);
                }
            }
            i += 1;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        lines: Vec<&'a str>,
        part1: i32,
        part2: i32,
    }

    #[test]
    fn tests() {
        let cases = vec![TestCase {
            lines: vec![
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
                "",
                "22 13 17 11  0",
                " 8  2 23  4 24",
                "21  9 14 16  7",
                " 6 10  3 18  5",
                " 1 12 20 15 19",
                "",
                " 3 15  0  2 22",
                " 9 18 13 17  5",
                "19  8  7 25 23",
                "20 11 10 24  4",
                "14 21 16 12  6",
                "",
                "14 21 17 24  4",
                "10 16 15  9 19",
                "18  8 23 26 20",
                "22 11 13  6  5",
                " 2  0 12  3  7",
            ],
            part1: 4512,
            part2: 1924,
        }];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }
}
