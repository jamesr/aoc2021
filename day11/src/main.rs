use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

type Grid = [[i32; 10]; 10];

fn neighbors(i: usize, j: usize) -> Vec<(usize, usize)> {
    (-1..=1)
        .flat_map(|e| std::iter::repeat(e).zip(-1..=1))
        .filter(|d| d.0 != 0 || d.1 != 0)
        .map(|d| (i as i32 + d.0, j as i32 + d.1))
        .filter(|p| (0..10).contains(&p.0))
        .filter(|p| (0..10).contains(&p.1))
        .map(|p| (p.0 as usize, p.1 as usize))
        .collect()
}

fn step(g: &mut Grid) -> i32 {
    let mut flashed = [[false; 10]; 10];
    let mut to_inc = (0..10)
        .flat_map(|e| std::iter::repeat(e).zip(0..10))
        .collect::<Vec<_>>();

    while to_inc.len() > 0 {
        let (i, j) = to_inc.pop().unwrap();
        g[i][j] += 1;
        if !flashed[i][j] && g[i][j] > 9 {
            flashed[i][j] = true;
            to_inc.extend(neighbors(i, j));
        }
    }

    let mut flashes = 0;

    for i in 0..10 {
        for j in 0..10 {
            if flashed[i][j] {
                flashes += 1;
                g[i][j] = 0;
            }
        }
    }

    return flashes;
}

fn parse(lines: &[&str]) -> [[i32; 10]; 10] {
    let mut g = [[0; 10]; 10];

    for i in 0..10 {
        let mut j = 0;
        lines[i].chars().for_each(|c| {
            g[i][j] = c as i32 - '0' as i32;
            j += 1
        });
    }

    g
}

fn part1(lines: &[&str]) -> i32 {
    let mut g = parse(lines);

    return (0..100).fold(0, |a, _| {
        return a + step(&mut g);
    });
}

fn done(g: &[[i32; 10]; 10]) -> bool {
    let val = g[0][0];
    for i in 0..10 {
        for j in 0..10 {
            if g[i][j] != val {
                return false;
            }
        }
    }
    return true;
}

fn part2(lines: &[&str]) -> i32 {
    let mut g = parse(lines);
    let mut steps = 0;
    loop {
        steps += 1;
        step(&mut g);
        if done(&g) {
            return steps;
        }
    }
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
                "5483143223",
                "2745854711",
                "5264556173",
                "6141336146",
                "6357385478",
                "4167524645",
                "2176841721",
                "6882881134",
                "4846848554",
                "5283751526",
            ],
            part1: 1656,
            part2: 195,
        }];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }
}
