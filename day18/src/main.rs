use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

#[derive(Copy, Clone, Debug)]
enum E {
    Start, // [
    Sep,   // ,
    End,   // ]
    Val(i32),
}

fn parse(s: &str) -> Vec<E> {
    let mut ret = vec![];

    for c in s.chars() {
        match c {
            '[' => ret.push(E::Start),
            ']' => ret.push(E::End),
            ',' => ret.push(E::Sep),
            v => {
                let val = (v as u8 - '0' as u8) as i32;
                let mut need_pop = false;
                let next_elem = if let Some(E::Val(v)) = ret.last() {
                    need_pop = true;
                    E::Val(v * 10 + val)
                } else {
                    E::Val(val)
                };
                if need_pop {
                    ret.pop();
                }
                ret.push(next_elem);
            }
        }
    }
    return ret;
}

fn magnitude_and_remainder<'a>(sn: &'a [E]) -> (i32, &'a [E]) {
    match sn[0] {
        E::Start => {
            let (left, remainder) = magnitude_and_remainder(&sn[1..sn.len()]);
            let (right, trailing) = magnitude_and_remainder(&remainder[1..remainder.len()]);
            return (left * 3 + right * 2, &trailing[1..trailing.len()]);
        }
        E::Val(v) => {
            return (v, &sn[1..sn.len()]);
        }
        E::Sep => return magnitude_and_remainder(&sn[1..sn.len()]),
        _ => panic!(""),
    }
}

fn magnitude(sn: &[E]) -> i32 {
    let (val, remainder) = magnitude_and_remainder(&sn);
    assert!(remainder.len() == 0);
    return val;
}

fn print(sn: &[E]) -> String {
    let mut ret = String::new();

    for e in sn {
        match e {
            E::Start => ret += "[",
            E::Sep => ret += ",",
            E::End => ret += "]",
            E::Val(v) => {
                ret += &format!("{}", v);
            }
        }
    }

    return ret;
}

fn explode_idx(sn: &[E]) -> Option<usize> {
    let mut depth = 0;
    for i in 0..sn.len() {
        match sn[i] {
            E::Start => {
                depth += 1;
                if depth == 5 {
                    return Some(i);
                }
            }
            E::End => {
                depth -= 1;
            }
            _ => {}
        };
    }
    return None;
}

fn find_val(s: &[E], forward: bool) -> Option<(usize, i32)> {
    for c in 0..s.len() {
        let i = if forward { c } else { s.len() - 1 - c };
        if let E::Val(v) = s[i] {
            return Some((i, v));
        }
    }
    return None;
}

fn val_at(sn: &[E], i: usize) -> i32 {
    if let E::Val(v) = sn[i] {
        v
    } else {
        panic!("expected value at index {} of {:?}", i, sn)
    }
}

fn explode(sn: &[E]) -> Option<Vec<E>> {
    match explode_idx(&sn) {
        None => None,
        Some(idx) => {
            let mut ret = vec![];
            let left_overflow = val_at(&sn, idx + 1);
            let right_overflow = val_at(&sn, idx + 3);

            let left_range = &sn[0..idx];
            match find_val(left_range, false) {
                None => ret.extend(left_range),
                Some((left_val_idx, val)) => {
                    let (before, after) = left_range.split_at(left_val_idx);
                    ret.extend(before);
                    ret.push(E::Val(val + left_overflow));
                    ret.extend(&after[1..after.len()]);
                }
            };

            ret.push(E::Val(0));

            let right_range = &sn[idx + 5..sn.len()];
            match find_val(&right_range, true) {
                None => ret.extend(right_range),
                Some((right_val_idx, val)) => {
                    let (before, after) = right_range.split_at(right_val_idx);
                    ret.extend(before);
                    ret.push(E::Val(val + right_overflow));
                    ret.extend(&after[1..after.len()]);
                }
            }

            return Some(ret);
        }
    }
}

fn add(lhs: &[E], rhs: &[E]) -> Vec<E> {
    let mut ret = vec![];
    ret.push(E::Start);
    ret.extend(lhs);
    ret.push(E::Sep);
    ret.extend(rhs);
    ret.push(E::End);

    return ret;
}

fn split(sn: &[E]) -> Option<Vec<E>> {
    for i in 0..sn.len() {
        match sn[i] {
            E::Start => {}
            E::End => {}
            E::Sep => {}
            E::Val(v) => {
                if v >= 10 {
                    let mut ret = vec![];
                    ret.extend(&sn[0..i]);
                    ret.push(E::Start);
                    ret.push(E::Val((v as f32 / 2.0).floor() as i32));
                    ret.push(E::Sep);
                    ret.push(E::Val((v as f32 / 2.0).ceil() as i32));
                    ret.push(E::End);
                    ret.extend(&sn[i + 1..sn.len()]);
                    return Some(ret);
                }
            }
        };
    }
    return None;
}

fn reduce(mut sn: Vec<E>) -> Vec<E> {
    loop {
        if let Some(exploded) = explode(&sn) {
            sn = exploded;
            continue;
        }
        if let Some(split) = split(&sn) {
            sn = split;
            continue;
        }
        return sn;
    }
}

fn part1(lines: &[&str]) -> i32 {
    let sns = lines.iter().map(|l| parse(&l)).collect::<Vec<_>>();

    return magnitude(
        &sns[1..sns.len()]
            .iter()
            .fold(sns[0].clone(), |acc, x| reduce(add(&acc, &x))),
    );
}

fn part2(lines: &[&str]) -> i32 {
    let sns = lines.iter().map(|l| parse(&l)).collect::<Vec<_>>();

    return (0..sns.len())
        .flat_map(|e| std::iter::repeat(e).zip(0..sns.len()))
        .filter(|p| p.0 != p.1)
        .map(|p| (&sns[p.0], &sns[p.1]))
        .map(|p| add(p.0, p.1))
        .map(reduce)
        .map(|sn| magnitude(&sn))
        .max()
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
    fn parse_tests() {
        let cases = vec![
            "[0,1]",
            "[[1,2],0]",
            "[0,[1,3]]",
            "[[2,2],[1,1]]",
            "[[[[2,2],[1,1]],0],0]",
        ];
        for c in cases {
            let sn = parse(c);
            assert_eq!(print(&sn), c);
        }
    }

    #[test]
    fn test_magnitude() {
        let cases = vec![(
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
            4140,
        )];
        for c in cases {
            let sn = parse(c.0);
            assert_eq!(magnitude(&sn), c.1);
        }
    }

    #[test]
    fn test_add() {
        let cases = vec![
            (("[1,2]", "[[3,4],5]"), "[[1,2],[[3,4],5]]"), //
        ];
        for c in cases {
            let lhs = parse(c.0 .0);
            let rhs = parse(c.0 .1);
            let sum = add(&lhs, &rhs);
            assert_eq!(print(&sum), c.1);
        }
    }

    #[test]
    fn test_explode() {
        let cases = vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"), //
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"), //
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"), //
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ), //
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ), //
        ];
        for c in cases {
            let sn = parse(c.0);
            let exploded = explode(&sn).unwrap();
            assert_eq!(print(&exploded), c.1);
        }
    }

    #[test]
    fn test_split() {
        let cases = vec![
            (
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            ),
            (
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            ),
        ];
        for c in cases {
            let sn = parse(c.0);
            let split = split(&sn).unwrap();
            assert_eq!(print(&split), c.1);
        }
    }

    #[test]
    fn tests() {
        let cases = vec![TestCase {
            lines: vec![
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
                "[[[5,[2,8]],4],[5,[[9,9],0]]]",
                "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
                "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
                "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
                "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
                "[[[[5,4],[7,7]],8],[[8,3],8]]",
                "[[9,3],[[9,9],[6,[4,9]]]]",
                "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
                "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
            ],
            part1: 4140,
            part2: 3993,
        }];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }
}
