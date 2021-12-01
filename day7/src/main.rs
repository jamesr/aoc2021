use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

fn part1(lines: &[&str]) -> i32 {
    let crabs = lines[0]
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    let max = crabs.iter().max().unwrap();

    return (1..max - 1)
        .map(|pos| crabs.iter().map(|c| (c - pos).abs()).sum())
        .min()
        .unwrap();
}

fn cost(dist: i32) -> i32 {
    return dist * (dist + 1) / 2;
}

fn part2(lines: &[&str]) -> i32 {
    let crabs = lines[0]
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    let max = crabs.iter().max().unwrap();

    return (1..max - 1)
        .map(|pos| crabs.iter().map(|c| cost((c - pos).abs())).sum())
        .min()
        .unwrap();
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
            lines: vec!["16,1,2,0,4,2,7,1,2,14"],
            part1: 37,
            part2: 168,
        }];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }

    #[test]
    fn test_cost() {
        assert_eq!(cost(1), 1);
        assert_eq!(cost(2), 1 + 2);
        assert_eq!(cost(3), 1 + 2 + 3);
        assert_eq!(cost(4), 1 + 2 + 3 + 4);
        assert_eq!(cost(5), 1 + 2 + 3 + 4 + 5);
        assert_eq!(cost(6), 1 + 2 + 3 + 4 + 5 + 6);
    }
}
