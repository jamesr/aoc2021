use std::collections::HashSet;
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

struct Paper {
    dots: HashSet<(i32, i32)>,
    width: i32,
    height: i32,
}

impl Paper {
    fn new() -> Paper {
        Paper {
            dots: HashSet::new(),
            width: 0,
            height: 0,
        }
    }

    fn add_dot(&mut self, x: i32, y: i32) {
        self.dots.insert((x, y));
        self.width = self.width.max(x + 1);
        self.height = self.height.max(y + 1);
    }

    fn fold(&mut self, f: &str) {
        let axis = f.split(" ").collect::<Vec<_>>()[2]
            .split("=")
            .collect::<Vec<_>>();
        let coord = axis[1].parse::<i32>().unwrap();
        match axis[0] {
            "x" => {
                let to_move = &self
                    .dots
                    .iter()
                    .copied() // XXX - why is this needed?
                    .filter(|d| d.0 > coord)
                    .collect::<Vec<_>>();
                for d in to_move {
                    let new_x = coord - (d.0 - coord);
                    self.dots.remove(&d);
                    self.dots.insert((new_x, d.1));
                }
                self.width = coord;
            }
            "y" => {
                let to_move = &self
                    .dots
                    .iter()
                    .copied() // XXX - why is this needed?
                    .filter(|d| d.1 > coord)
                    .collect::<Vec<_>>();
                for d in to_move {
                    let new_y = coord - (d.1 - coord);
                    self.dots.remove(&d);
                    self.dots.insert((d.0, new_y));
                }
                self.height = coord;
            }
            _ => panic!("unknown axis {:?}", axis),
        }
    }

    fn count(&self) -> i32 {
        self.dots.len() as i32
    }

    fn print(&self) {
        (0..self.height as i32).for_each(|y| {
            (0..self.width as i32).for_each(|x| {
                print!(
                    "{}",
                    if self.dots.contains(&(x, y)) {
                        "#"
                    } else {
                        "."
                    }
                )
            });
            println!("")
        })
    }
}

fn part1(lines: &[&str]) -> i32 {
    let mut paper = Paper::new();
    for l in lines {
        if l.is_empty() {
            break;
        }
        let coords = l
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i32>>();

        paper.add_dot(coords[0], coords[1]);
    }
    //println!("initial:");
    //paper.print();
    let folds = lines
        .iter()
        .filter(|l| l.starts_with("fold"))
        .collect::<Vec<_>>();

    paper.fold(folds[0]);
    //println!("after fold:");
    //paper.print();
    return paper.count();
}

fn part2(lines: &[&str]) -> i32 {
    let mut paper = Paper::new();
    for l in lines {
        if l.is_empty() {
            break;
        }
        let coords = l
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i32>>();

        paper.add_dot(coords[0], coords[1]);
    }
    let folds = lines
        .iter()
        .filter(|l| l.starts_with("fold"))
        .collect::<Vec<_>>();

    for f in folds {
        paper.fold(f);
    }
    println!("after folds:");
    paper.print();
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
                "6,10",
                "0,14",
                "9,10",
                "0,3",
                "10,4",
                "4,11",
                "6,0",
                "6,12",
                "4,1",
                "0,13",
                "10,12",
                "3,4",
                "3,0",
                "8,4",
                "1,10",
                "2,14",
                "8,10",
                "9,0",
                "",
                "fold along y=7",
                "fold along x=5",
            ],
            part1: 17,
            part2: 0,
        }];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }
}
