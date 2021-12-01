use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

fn is_open(c: char) -> bool {
    match c {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false,
    }
}

fn matches(open: char, close: char) -> bool {
    match open {
        '(' => close == ')',
        '[' => close == ']',
        '{' => close == '}',
        '<' => close == '>',
        _ => panic!("unexpected char {}", open),
    }
}

fn ill(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unexpected char {}", c),
    }
}

fn score1(line: &str) -> i32 {
    let mut expected: Vec<char> = vec![];

    for c in line.chars() {
        if is_open(c) {
            expected.push(c);
        } else {
            match expected.pop() {
                None => return ill(c),
                Some(open) => {
                    if !matches(open, c) {
                        //println!("found mismatch {} vs {} on line {}", open, c, line);
                        return ill(c);
                    }
                }
            }
        }
    }
    return 0; // no syntax error
}

fn part1(lines: &[&str]) -> i32 {
    return lines.iter().map(|l| score1(l)).sum();
}

fn compl(c: &char) -> i64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("unexpected char {}", c),
    }
}

fn score2(line: &str) -> Option<i64> {
    let mut expected: Vec<char> = vec![];

    for c in line.chars() {
        if is_open(c) {
            expected.push(c);
        } else {
            match expected.pop() {
                None => return None,
                Some(open) => {
                    if !matches(open, c) {
                        //println!("found mismatch {} vs {} on line {}", open, c, line);
                        return None;
                    }
                }
            }
        }
    }
    expected.reverse();
    //println!("incomplete line still need to match {:?}", expected);
    return Some(expected.iter().map(compl).fold(0, |a, i| a * 5 + i));
}

fn part2(lines: &[&str]) -> i64 {
    let mut compl = vec![];

    for line in lines {
        match score2(line) {
            Some(s) => compl.push(s),
            None => {}
        }
    }

    compl.sort();
    return compl[compl.len() / 2];
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        lines: Vec<&'a str>,
        part1: i32,
        part2: i64,
    }

    #[test]
    fn tests() {
        let cases = vec![TestCase {
            lines: vec![
                "[({(<(())[]>[[{[]{<()<>>",
                "[(()[<>])]({[<{<<[]>>(",
                "{([(<{}[<>[]}>{[]{[(<()>",
                "(((({<>}<{<{<>}{[]{[]{}",
                "[[<[([]))<([[{}[[()]]]",
                "[{[{({}]{}}([{[{{{}}([]",
                "{<[[]]>}<{[{[{[]{()[[[]",
                "[<(<(<(<{}))><([]([]()",
                "<{([([[(<>()){}]>(<<{{",
                "<{([{{}}[<[[[<>{}]]]>[]]",
            ],
            part1: 26397,
            part2: 288957,
        }];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }
}
