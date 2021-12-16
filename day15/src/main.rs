use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

struct Grid {
    risks: HashMap<(i32, i32), i32>,
    expanded: bool,
    height: i32,
    width: i32,
}

impl Grid {
    fn parse(lines: &[&str], expanded: bool) -> Grid {
        let mut risks = HashMap::new();

        for r in 0..lines.len() {
            let chars = lines[r].chars().collect::<Vec<_>>();
            for c in 0..lines[r].len() {
                risks.insert((r as i32, c as i32), (chars[c] as u8 - '0' as u8) as i32);
            }
        }

        let height = lines.len() as i32 * if expanded { 5 } else { 1 };
        let width = lines[0].len() as i32 * if expanded { 5 } else { 1 };

        return Grid {
            risks,
            expanded,
            height,
            width,
        };
    }

    fn risk_at(&self, r: i32, c: i32) -> i32 {
        let p = match self.expanded {
            true => (r % (self.height / 5), c % (self.width / 5)),
            false => (r, c),
        };
        let val = *self.risks.get(&p).unwrap();
        return match self.expanded {
            true => (val + r / (self.height / 5) + c / (self.width / 5) - 1) % 9 + 1,
            false => val,
        };
    }

    fn least_risk(&self) -> i32 {
        let mut min_risks = HashMap::new();
        let mut heap = BinaryHeap::new();

        heap.push(Reverse((0, 0, 0)));

        while let Some(Reverse((risk, r, c))) = heap.pop() {
            if r + 1 == self.height && c + 1 == self.width {
                return risk;
            }

            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .map(|d| (r as i32 + d.0, c as i32 + d.1))
                .filter(|p| (0..self.height as i32).contains(&p.0))
                .filter(|p| (0..self.width as i32).contains(&p.1))
                .for_each(|p| {
                    let new_risk = risk + self.risk_at(p.0, p.1);

                    let prev_risk = match min_risks.get(&p) {
                        Some(r) => *r,
                        None => i32::MAX,
                    };

                    if new_risk < prev_risk {
                        min_risks.insert(p, new_risk);
                        heap.push(Reverse((new_risk, p.0, p.1)));
                    }
                });
        }
        panic!("did not find");
    }

    #[allow(dead_code)]
    fn print(&self) {
        (0..self.height).for_each(|r| {
            (0..self.width).for_each(|c| print!("{}", self.risk_at(r, c)));
            println!("");
        })
    }
}

fn part1(lines: &[&str]) -> i32 {
    let grid = Grid::parse(lines, false);

    return grid.least_risk();
}

fn part2(lines: &[&str]) -> i32 {
    let grid = Grid::parse(lines, true);

    return grid.least_risk();
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
        let cases = vec![
            TestCase {
                lines: vec!["8"],
                part1: 0,
                part2: 37,
            },
            TestCase {
                lines: vec![
                    "1163751742",
                    "1381373672",
                    "2136511328",
                    "3694931569",
                    "7463417111",
                    "1319128137",
                    "1359912421",
                    "3125421639",
                    "1293138521",
                    "2311944581",
                ],
                part1: 40,
                part2: 315,
            },
        ];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }
}
