use std::convert::TryInto;
use std::fs;

fn main() {
    println!("Hello, world!");
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let values = contents
        .lines()
        .map(|l| i32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();
    let width: i32 = contents.lines().next().unwrap().len().try_into().unwrap();

    let (gamma, epsilon) = part1(&values, width);
    println!(
        "product: {}            (gamma {} {:b} epsilon {} {:b})",
        gamma * epsilon,
        gamma,
        gamma,
        epsilon,
        epsilon,
    );
    let (oxy, co2) = part2(&values, width);
    println!(
        "life support rating {} (oxygn {} {:b} co2     {} {:b})",
        oxy * co2,
        oxy,
        oxy,
        co2,
        co2
    );
}

// Count the number of 1s in the given position
fn count_bits(values: &[i32], bit: i32) -> i32 {
    let mut count = 0;
    for value in values {
        if value & (1 << bit) != 0 {
            count += 1
        }
    }
    return count;
}

fn part1(values: &[i32], width: i32) -> (i32, i32) {
    let mut gamma = 0;
    let mut epsilon = 0;
    for bit in 0..width {
        let count = count_bits(values, bit);
        if count * 2 > values.len().try_into().unwrap() {
            gamma |= 1 << bit
        }
        if count * 2 < values.len().try_into().unwrap() {
            epsilon |= 1 << bit
        }
    }

    return (gamma, epsilon);
}

fn find(values: &[i32], width: i32, most: bool) -> i32 {
    let mut filtered = vec![];
    filtered.extend_from_slice(values);

    for bit in (0..width).rev() {
        let count = count_bits(&filtered, bit);
        let common = count * 2 >= filtered.len().try_into().unwrap();
        let mask = if common == most { 1 << bit } else { 0 };
        filtered.retain(|v| return v & (1 << bit) == mask);
        if filtered.len() == 1 {
            return filtered[0];
        }
    }
    panic!("search ended with {} entries left", filtered.len());
}

fn part2(values: &[i32], width: i32) -> (i32, i32) {
    let oxy = find(values, width, true);
    let co2 = find(values, width, false);

    return (oxy, co2);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        lines: Vec<&'a str>,
        gamma: i32,
        epsilon: i32,
        oxy: i32,
        co2: i32,
    }

    #[test]
    fn tests() {
        let cases = vec![TestCase {
            lines: vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ],
            gamma: 22,
            epsilon: 9,
            oxy: 23,
            co2: 10,
        }];
        for t in cases.iter() {
            let values = t
                .lines
                .iter()
                .map(|l| i32::from_str_radix(l, 2).unwrap())
                .collect::<Vec<_>>();
            let width: i32 = t.lines[0].len().try_into().unwrap();
            let (gamma, epsilon) = part1(&values, width);
            assert_eq!(gamma, t.gamma, "got {:b} expected {:b}", gamma, t.gamma);
            assert_eq!(
                epsilon, t.epsilon,
                "got {:b} expected {:b}",
                epsilon, t.epsilon
            );
            let (oxy, co2) = part2(&values, width);
            assert_eq!(oxy, t.oxy, "got {:b} expected {:b}", oxy, t.oxy);
            assert_eq!(co2, t.co2, "got {:b} expected {:b}", co2, t.co2);
        }
    }
}
