use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

fn simulate(l: &str, days: i32) -> i64 {
    //println!("simulating {} days", days);
    let mut ages: [i64; 9] = [0; 9];

    l.split(",")
        .for_each(|s| ages[s.parse::<usize>().unwrap()] += 1);

    for _day in 0..days {
        // age fishes down
        ages.rotate_left(1);
        // count for the fishes that reproduced
        ages[6] += ages[8];
        //println!("after day {} ages are {:?}", day + 1, ages);
    }

    return ages.iter().sum();
}

fn part1(lines: &[&str]) -> i64 {
    return simulate(lines[0], 80);
}

fn part2(lines: &[&str]) -> i64 {
    return simulate(lines[0], 256);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        lines: Vec<&'a str>,
        part1: i64,
        part2: i64,
    }

    #[test]
    fn tests() {
        let cases = vec![TestCase {
            lines: vec!["3,4,3,1,2"],
            part1: 5934,
            part2: 26984457539,
        }];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }

    #[test]
    fn test_simulate() {
        assert_eq!(simulate("3,4,3,2,1", 1), 5);
        assert_eq!(simulate("3,4,3,2,1", 2), 6);
        assert_eq!(simulate("3,4,3,2,1", 3), 7);
        assert_eq!(simulate("3,4,3,2,1", 4), 9);
        assert_eq!(simulate("3,4,3,2,1", 5), 10);
        assert_eq!(simulate("3,4,3,2,1", 6), 10);
        assert_eq!(simulate("3,4,3,2,1", 7), 10);
        assert_eq!(simulate("3,4,3,2,1", 8), 10);
        assert_eq!(simulate("3,4,3,2,1", 9), 11);
        assert_eq!(simulate("3,4,3,2,1", 18), 26);
    }
}
