use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let (horiz, depth) = simulate(contents.lines());
    println!("simulation product is {}", horiz * depth);
    let (horiz_aim, depth_aim) = simulate_aim(contents.lines());
    println!("simulation with aim product is {}", horiz_aim * depth_aim);
}

fn simulate<'a>(lines: impl Iterator<Item = &'a str>) -> (i64, i64) {
    let mut horiz = 0;
    let mut depth = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() != 2 {
            panic!("unexpected line format: {}", line);
        }
        let amount = parts[1].parse::<i64>().unwrap();
        match parts[0] {
            "forward" => horiz = horiz + amount,
            "up" => depth = depth - amount,
            "down" => depth = depth + amount,
            _ => {
                panic!("unknown command {}", parts[0]);
            }
        }
    }
    return (horiz, depth);
}

fn simulate_aim<'a>(lines: impl Iterator<Item = &'a str>) -> (i64, i64) {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() != 2 {
            panic!("unexpected line format: {}", line);
        }
        let amount = parts[1].parse::<i64>().unwrap();
        match parts[0] {
            "forward" => {
                horiz = horiz + amount;
                depth = depth + aim * amount
            }
            "up" => aim = aim - amount,
            "down" => aim = aim + amount,
            _ => {
                panic!("unknown command {}", parts[0]);
            }
        }
    }
    return (horiz, depth);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        expected_product: i64,
        lines: Vec<&'a str>,
    }

    #[test]
    fn tests() {
        let cases = vec![TestCase {
            expected_product: 150,
            lines: vec![
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2",
            ],
        }];

        for t in cases.iter() {
            let (horiz, depth) = simulate(t.lines.iter().map(AsRef::as_ref));
            assert_eq!(
                t.expected_product,
                horiz * depth,
                "test case got horiz {} depth {} lines {:?}",
                horiz,
                depth,
                t.lines
            );
        }
    }

    #[test]
    fn tests_aim() {
        let cases = vec![TestCase {
            expected_product: 900,
            lines: vec![
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2",
            ],
        }];

        for t in cases.iter() {
            let (horiz, depth) = simulate_aim(t.lines.iter().map(AsRef::as_ref));
            assert_eq!(
                t.expected_product,
                horiz * depth,
                "test case got horiz {} depth {} lines {:?}",
                horiz,
                depth,
                t.lines
            );
        }
    }
}
