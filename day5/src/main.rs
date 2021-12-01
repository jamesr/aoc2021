use std::cmp;
use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

type Pt = (i32, i32);
type Grid = HashMap<Pt, i32>;

fn parse_point(s: &str) -> Pt {
    let parts = s.split(",");
    let points: Vec<i32> = parts.map(|v| v.parse().unwrap()).collect();
    return (points[0], points[1]);
}

fn walk(start: Pt, end: Pt, grid: &mut Grid) {
    let dx = match start.0.cmp(&end.0) {
        cmp::Ordering::Less => 1,
        cmp::Ordering::Equal => 0,
        cmp::Ordering::Greater => -1,
    };
    let dy = match start.1.cmp(&end.1) {
        cmp::Ordering::Less => 1,
        cmp::Ordering::Equal => 0,
        cmp::Ordering::Greater => -1,
    };
    let mut pos = start;
    loop {
        let v = grid.entry(pos).or_insert(0);
        *v += 1;
        if pos == end {
            break;
        }
        pos.0 += dx;
        pos.1 += dy;
    }
}

fn findvents(lines: &[&str], diag: bool) -> i32 {
    let mut grid: HashMap<Pt, i32> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines {
        let mut parts = line.split(" -> ");
        let start = parse_point(parts.next().unwrap());
        let end = parse_point(parts.next().unwrap());
        max_x = cmp::max(max_x, start.0);
        max_x = cmp::max(max_x, end.0);
        max_y = cmp::max(max_y, end.1);
        max_y = cmp::max(max_y, end.1);
        if diag || start.0 == end.0 || start.1 == end.1 {
            walk(start, end, &mut grid);
        }
    }
    return grid.values().filter(|v| **v >= 2).count() as i32;
}

fn part1(lines: &[&str]) -> i32 {
    return findvents(lines, false);
}

fn part2(lines: &[&str]) -> i32 {
    return findvents(lines, true);
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
                "0,9 -> 5,9",
                "8,0 -> 0,8",
                "9,4 -> 3,4",
                "2,2 -> 2,1",
                "7,0 -> 7,4",
                "6,4 -> 2,0",
                "0,9 -> 2,9",
                "3,4 -> 1,4",
                "0,0 -> 8,8",
                "5,5 -> 8,2",
            ],
            part1: 5,
            part2: 12,
        }];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }
}
