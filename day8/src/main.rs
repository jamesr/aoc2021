use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

fn part1(lines: &[&str]) -> i32 {
    lines
        .iter()
        .map(|l| {
            l.split("|")
                .last()
                .unwrap()
                .split(" ")
                .map(|s| s.len())
                .filter(|l| *l == 2 || *l == 3 || *l == 4 || *l == 7)
                .count()
        })
        .sum::<usize>() as i32
}

//  "aaaa",
// "b    c",
// "b    c",
// "dddd",
// "e    f",
// "e    f",
// "gggg",

// 0: abc efg (6 digits)
// 1:   c  f  (2 digits)
// 2: a cde g (5 digits)
// 3: a cd fg (5 digits)
// 4:  bcd f  (4 digits)
// 5: ab d fg (5 digits)
// 6: ab defg (6 digits)
// 7: a c  f  (3 digits)
// 8: abcdefg (7 digits)
// 9: abcd fg (6 digits)
// + ---------
//    8687497
//    abcdefg

fn idx(c: char) -> usize {
    return c as usize - 'a' as usize;
}

fn identify(signals: &[&str]) -> [u8; 7] {
    let mut count: [u8; 7] = [0; 7];
    for o in signals {
        for b in o.chars() {
            count[idx(b)] += 1;
        }
    }
    return count;
}

// counts:
//  8687497
//  abcdefg
//
// unique: b=6, f=9
fn decode(signals: &str, count: &[u8; 7]) -> i32 {
    match signals.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => {
            // one of:
            //  2: a cde g (5 digits)
            //  3: a cd fg (5 digits)
            //  5: ab d fg (5 digits)
            let has_e = signals.chars().filter(|c| count[idx(*c)] == 4).count() == 1;
            if has_e {
                return 2;
            }
            let has_b = signals.chars().filter(|c| count[idx(*c)] == 6).count() == 1;
            if has_b {
                return 5;
            }
            return 3;
        }
        6 => {
            // one of:
            // 0: abc efg (6 digits)
            // 6: ab defg (6 digits)
            // 9: abcd fg (6 digits)
            let has_e = signals.chars().filter(|c| count[idx(*c)] == 4).count() == 1;
            if !has_e {
                return 9;
            }
            let has_d = signals.chars().filter(|c| count[idx(*c)] == 7).count() == 2; // g and c
            if has_d {
                return 6;
            }
            return 0;
        }
        7 => 8,
        _ => panic!("bad signals "),
    }
}

fn part2(lines: &[&str]) -> i32 {
    let mut result = 0;
    for l in lines {
        let mut p = l.split("|");
        let signals = p.next().unwrap().split_whitespace().collect::<Vec<_>>();
        let outputs = p.next().unwrap().split_whitespace().collect::<Vec<_>>();

        let count = identify(&signals);

        result += outputs.iter().fold(0, |a, o| a * 10 + decode(o, &count));
    }
    return result;
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
"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
"edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
"fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
"fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
"aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
"fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
"dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
"bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
"egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
"gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
           ],
            part1: 26,
            part2: 61229,
        }];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }
}
