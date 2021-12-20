use std::collections::HashSet;
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

#[derive(Default)]
struct Image {
    known: HashSet<(i32, i32)>,
    width: i32,
    height: i32,
    surround: i32,
    outside: bool,
}

impl Image {
    fn contains(&self, p: (i32, i32)) -> bool {
        let last_surround = self.surround - 2;
        let x_range = -last_surround..self.width as i32 + last_surround;
        let y_range = -last_surround..self.height as i32 + last_surround;
        if x_range.contains(&p.0) && y_range.contains(&p.1) {
            self.known.contains(&p)
        } else {
            self.outside
        }
    }

    fn count(&self, x: i32, y: i32) -> i32 {
        (-1..=1)
            .flat_map(|r| std::iter::repeat(r).zip(-1..=1))
            .map(|p| (x + p.0, y + p.1))
            .map(|p| self.contains(p))
            .map(|b| if b { 1 } else { 0 })
            .fold(0, |a, i| i | a << 1)
    }

    fn print(&self) {
        println!(
            "width {} height {} surround {} outside {}",
            self.width, self.height, self.surround, self.outside
        );
        for x in -self.surround..self.width as i32 + self.surround {
            for y in -self.surround..self.height as i32 + self.surround {
                print!("{}", if self.contains((x, y)) { '#' } else { '.' });
            }
            println!();
        }
    }
}

fn to_bit(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => panic!("unknown char"),
    }
}

fn apply(image: &Image, filter: &Vec<bool>) -> Image {
    let mut ret = Image::default();
    ret.width = image.width;
    ret.height = image.height;
    ret.surround = image.surround + 2;
    for x in -image.surround..image.width as i32 + image.surround {
        for y in -image.surround..image.height as i32 + image.surround {
            let val = image.count(x, y);
            if filter[val as usize] {
                ret.known.insert((x, y));
            }
        }
    }
    if image.outside {
        ret.outside = filter[511]
    } else {
        ret.outside = filter[0]
    }
    return ret;
}

fn part1(lines: &[&str]) -> i32 {
    let filter = lines[0].chars().map(to_bit).collect::<Vec<_>>();

    let image_vec = lines[2..lines.len()]
        .iter()
        .map(|l| l.chars().map(to_bit).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut image = Image::default();
    image.height = image_vec.len() as i32;
    image.width = image_vec[0].len() as i32;
    image.surround = 2;
    for r in 0..image.height {
        for c in 0..image.width {
            if image_vec[r as usize][c as usize] {
                image.known.insert((r, c));
            }
        }
    }

    image = apply(&image, &filter);
    image = apply(&image, &filter);
    return image.known.len() as i32;
}

fn part2(lines: &[&str]) -> i32 {
    let filter = lines[0].chars().map(to_bit).collect::<Vec<_>>();

    let image_vec = lines[2..lines.len()]
        .iter()
        .map(|l| l.chars().map(to_bit).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut image = Image::default();
    image.height = image_vec.len() as i32;
    image.width = image_vec[0].len() as i32;
    image.surround = 2;
    for r in 0..image.height {
        for c in 0..image.width {
            if image_vec[r as usize][c as usize] {
                image.known.insert((r, c));
            }
        }
    }

    for _ in 0..50 {
        image = apply(&image, &filter);
    }
    return image.known.len() as i32;
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
                "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
                "",
                "#..#.",
                "#....",
                "##..#",
                "..#..",
                "..###",
            ],
            part1: 35,
            part2: 3351,
        }];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }
}
