use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

fn is_low(heights: &Vec<Vec<i32>>, r: i32, c: i32) -> bool {
    let height = heights[r as usize][c as usize];
    let deltas: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    return deltas
        .iter()
        .map(|d| (d.0 + r, d.1 + c))
        .filter(|p| (0..heights.len() as i32).contains(&p.0))
        .filter(|p| (0..heights[0].len() as i32).contains(&p.1))
        .filter(|p| heights[p.0 as usize][p.1 as usize] < height)
        .count()
        == 0;
}

fn part1(lines: &[&str]) -> i32 {
    let heights = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c as i32 - '0' as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    let mut ret = 0;

    for r in 0..heights.len() {
        for c in 0..heights[r].len() {
            if is_low(&heights, r as i32, c as i32) {
                ret += 1 + heights[r][c];
            }
        }
    }

    return ret;
}

fn fill(heights: &mut Vec<Vec<i32>>, r: i32, c: i32) -> i32 {
    if !(0..(heights.len() as i32)).contains(&r) {
        return 0;
    }
    if !(0..heights[r as usize].len() as i32).contains(&c) {
        return 0;
    }

    if heights[r as usize][c as usize] == 9 {
        return 0;
    }

    heights[r as usize][c as usize] = 9;
    return [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|d| fill(heights, r + d.0, c + d.1))
        .sum::<i32>()
        + 1;
}

fn part2(lines: &[&str]) -> i32 {
    let mut heights = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c as i32 - '0' as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    let mut sizes = vec![];

    for r in 0..heights.len() {
        for c in 0..heights[r].len() {
            if is_low(&heights, r as i32, c as i32) {
                let size = fill(&mut heights, r as i32, c as i32);
                sizes.push(size);
            }
        }
    }
    sizes.sort();

    return sizes[sizes.len() - 1] * sizes[sizes.len() - 2] * sizes[sizes.len() - 3];
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
                "2199943210",
                "3987894921",
                "9856789892",
                "8767896789",
                "9899965678",
            ],
            part1: 15,
            part2: 1134,
        }];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }
}
