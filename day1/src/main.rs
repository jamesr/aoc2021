use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    println!("count is {}", count(contents.lines()));
    println!("rolling sum count is {}", count_rolling(contents.lines()));
}

fn count<'a>(lines: impl Iterator<Item = &'a str>) -> i32 {
    let mut last: Option<i32> = None;
    let mut result = 0;
    for line in lines {
        let value: i32 = line
            .parse()
            .expect(&format!("could not parse line {}", line));

        if last.is_some() && value > last.unwrap() {
            result = result + 1
        }
        last = Some(value);
    }
    return result;
}

fn count_rolling<'a>(lines: impl Iterator<Item = &'a str>) -> i32 {
    let mut recent: [Option<i32>; 3] = [None, None, None];
    let mut result = 0;
    let mut idx = 0;
    for line in lines {
        let value: i32 = line
            .parse()
            .expect(&format!("could not parse line {}", line));
        let prev = recent[idx];
        recent[idx] = Some(value);
        idx = (idx + 1) % 3;
        if prev.is_some() && value > prev.unwrap() {
            result = result + 1
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        expected_count: i32,
        lines: Vec<&'a str>,
    }

    #[test]
    fn count_tests() {
        let cases = vec![
            TestCase {
                expected_count: 0,
                lines: vec![],
            },
            TestCase {
                expected_count: 0,
                lines: vec!["120"],
            },
            TestCase {
                expected_count: 1,
                lines: vec!["120", "130"],
            },
            TestCase {
                expected_count: 0,
                lines: vec!["130", "120"],
            },
            TestCase {
                expected_count: 7,
                lines: vec![
                    "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
                ],
            },
        ];

        for test in cases {
            assert_eq!(
                test.expected_count,
                count(test.lines.iter().map(AsRef::as_ref)),
                "expected count {} for lines {:?}",
                test.expected_count,
                test.lines
            );
        }
    }
    #[test]
    fn rolling_sum_tests() {
        let cases = vec![
            TestCase {
                expected_count: 0,
                lines: vec![],
            },
            TestCase {
                expected_count: 5,
                lines: vec![
                    "199", //  A
                    "200", //  A B
                    "208", //  A B C
                    "210", //    B C D
                    "200", //  E   C D
                    "207", //  E F   D
                    "240", //  E F G
                    "269", //    F G H
                    "260", //      G H
                    "263",
                ],
            },
        ];
        for t in cases.iter() {
            assert_eq!(
                t.expected_count,
                count_rolling(t.lines.iter().map(AsRef::as_ref)),
                "test case {:?}",
                t.lines
            );
        }
    }
}
