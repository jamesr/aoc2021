use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

type Rules = HashMap<(char, char), char>;
type Pairs = HashMap<(char, char), i64>;

#[derive(Default)]
struct Polymer {
    pairs: Pairs,
    counts: [i64; 26],
}

fn c_idx(c: char) -> usize {
    c as usize - 'A' as usize
}

fn add_count(pairs: &mut Pairs, pair: (char, char), count: i64) {
    let val = match pairs.get(&pair) {
        Some(c) => *c,
        None => 0,
    };
    pairs.insert(pair, val + count);
}

impl Polymer {
    fn step(&mut self, rules: &Rules) {
        let mut new_pairs = HashMap::new();
        for k in self.pairs.keys() {
            let count = *self.pairs.get(&k).unwrap();
            match rules.get(&k) {
                Some(insert) => {
                    add_count(&mut new_pairs, (k.0, *insert), count);
                    add_count(&mut new_pairs, (*insert, k.1), count);
                    self.counts[c_idx(*insert)] += count;
                }
                None => {
                    add_count(&mut new_pairs, *k, count);
                }
            };
        }
        self.pairs = new_pairs;
    }

    fn most(&self) -> i64 {
        *self.counts.iter().max().unwrap()
    }

    fn least(&self) -> i64 {
        *self.counts.iter().filter(|c| **c > 0).min().unwrap()
    }
}

fn doit(lines: &[&str], steps: i32) -> i64 {
    let template = lines[0].chars().collect::<Vec<_>>();

    let mut rules = HashMap::new();

    for l in &lines[2..] {
        let words = l.split(" -> ").collect::<Vec<_>>();
        let pair = words[0].chars().collect::<Vec<_>>();
        let insert = words[1].chars().collect::<Vec<_>>();

        rules.insert((pair[0], pair[1]), insert[0]);
    }

    let mut polymer = Polymer::default();

    template
        .windows(2)
        .map(|p| (p[0], p[1]))
        .for_each(|p| add_count(&mut polymer.pairs, p, 1));

    template.iter().for_each(|c| polymer.counts[c_idx(*c)] += 1);

    (0..steps).for_each(|_| polymer.step(&rules));

    return polymer.most() - polymer.least();
}

fn part1(lines: &[&str]) -> i32 {
    doit(lines, 10) as i32
}

fn part2(lines: &[&str]) -> i64 {
    doit(lines, 40)
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
                "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B",
                "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N",
                "BC -> B", "CC -> N", "CN -> C",
            ],
            part1: 1588,
            part2: 2188189693529,
        }];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }
}
