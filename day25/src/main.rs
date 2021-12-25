use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Thing {
    Down,
    Left,
    Empty,
}

struct Grid {
    vals: Vec<Vec<Thing>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn parse(lines: &[&str]) -> Grid {
        let vals = lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        'v' => Thing::Down,
                        '>' => Thing::Left,
                        '.' => Thing::Empty,
                        _ => panic!("unknown char"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let height = vals.len();
        let width = vals[0].len();
        Grid {
            vals,
            height,
            width,
        }
    }
    fn next(&self, r: usize, c: usize, dir: Thing) -> (usize, usize) {
        match dir {
            Thing::Down => ((r + 1) % self.height, c),
            Thing::Left => (r, (c + 1) % self.width),
            Thing::Empty => panic!(),
        }
    }

    fn move_herd(&mut self, kind: Thing) -> bool {
        let mut moved_anything = false;

        let mut can_move = vec![];
        can_move.resize(self.height, vec![]);
        for r in 0..self.height {
            can_move[r].resize(self.width, false);
        }
        // check which can move
        for r in 0..self.height {
            for c in 0..self.width {
                if self.vals[r][c] == kind {
                    let n = self.next(r, c, kind);
                    can_move[r][c] = self.vals[n.0][n.1] == Thing::Empty;
                }
            }
        }
        // now move
        for r in 0..self.height {
            for c in 0..self.width {
                if can_move[r][c] {
                    let n = self.next(r, c, kind);
                    self.vals[r][c] = Thing::Empty;
                    self.vals[n.0][n.1] = kind;
                    moved_anything = true;
                }
            }
        }

        return moved_anything;
    }

    fn step(&mut self) -> bool {
        self.move_herd(Thing::Left) | self.move_herd(Thing::Down)
    }

    fn print(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                print!(
                    "{}",
                    match self.vals[r][c] {
                        Thing::Down => "v",
                        Thing::Left => ">",
                        Thing::Empty => ".",
                    }
                );
            }
            println!("")
        }
    }
}

fn part1(lines: &[&str]) -> i32 {
    let mut grid = Grid::parse(lines);

    for i in 0.. {
        //println!("before step {}", i + 1);
        //grid.print();
        let moved = grid.step();
        if !moved {
            return i as i32 + 1;
        }
    }

    panic!("unreachable");
}

fn part2(lines: &[&str]) -> i32 {
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
                "v...>>.vv>",
                ".vv>>.vv..",
                ">>.>v>...v",
                ">>v>>.>.v.",
                "v>v.vv.v..",
                ">.>>..v...",
                ".vv..>.>v.",
                "v.v..>>v.v",
                "....v..v.>",
            ],
            part1: 58,
            part2: 0,
        }];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }
}
