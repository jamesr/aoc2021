use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect(&format!("could not read {}", filename));
    let lines: Vec<&str> = contents.lines().collect();
    println!("part1 is {}", part1(&lines));
    println!("part2 is {}", part2(&lines));
}

struct Cave {
    adjacent: Vec<String>,
}

impl Cave {
    fn new() -> Cave {
        Cave { adjacent: vec![] }
    }
}

struct Path {
    cur: String,
    visited: HashSet<String>,
    visited_small_twice: bool,
}

type Map = HashMap<String, Cave>;

impl Path {
    fn new(cur: String, visited: HashSet<String>, visited_small_twice: bool) -> Path {
        Path {
            cur,
            visited,
            visited_small_twice,
        }
    }

    // returns # of full paths, new paths to try
    fn visit(&self, map: &Map, can_visit_twice: bool) -> (i32, Vec<Path>) {
        let mut paths = vec![];
        let c = map.get(&self.cur).unwrap();
        let mut found = 0;
        for a in &c.adjacent {
            // are we at the end?
            if a == "end" {
                found += 1;
                continue;
            }
            if a == "start" {
                // can't revisit start
                continue;
            }
            // can we go to cave 'a' in this traversal?
            let seen = self.visited.contains(a);
            let is_lower = a.chars().next().unwrap().is_lowercase();
            // if this is a small cave, have we been here before?
            if seen && is_lower {
                if can_visit_twice && !self.visited_small_twice {
                    // can proceed, but must remember we've double-visited a small cave
                    paths.push(Path::new(a.to_string(), self.visited.clone(), true));
                    continue;
                } else {
                    // no good
                    continue;
                }
            }

            if a != "end" {
                // new path
                let mut new_visited = self.visited.clone();
                new_visited.insert(a.to_string());
                paths.push(Path::new(
                    a.to_string(),
                    new_visited,
                    self.visited_small_twice,
                ));
            }
        }
        return (found, paths);
    }
}

fn numpaths(lines: &[&str], can_visit_twice: bool) -> i32 {
    let mut map: Map = HashMap::new();
    for l in lines {
        let names = l.split("-").collect::<Vec<_>>();
        names.iter().map(|s| s.to_string()).for_each(|cave| {
            if !map.contains_key(&cave) {
                map.insert(cave, Cave::new());
            }
        });
        map.get_mut(names[0])
            .unwrap()
            .adjacent
            .push(names[1].to_string());
        map.get_mut(names[1])
            .unwrap()
            .adjacent
            .push(names[0].to_string());
    }

    let mut paths = vec![Path::new("start".to_string(), HashSet::new(), false)];
    let mut found = 0;

    while paths.len() > 0 {
        let p = paths.pop().unwrap();
        let (f, np) = p.visit(&map, can_visit_twice);
        found += f;
        paths.extend(np);
    }

    return found;
}

fn part1(lines: &[&str]) -> i32 {
    numpaths(lines, false)
}

fn part2(lines: &[&str]) -> i32 {
    numpaths(lines, true)
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
        let cases = vec![
            TestCase {
                lines: vec!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"],
                part1: 10,
                part2: 36,
            },
            TestCase {
                lines: vec![
                    "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end",
                    "kj-sa", "kj-HN", "kj-dc",
                ],
                part1: 19,
                part2: 103,
            },
            TestCase {
                lines: vec![
                    "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj",
                    "pj-he", "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he",
                    "pj-fs", "start-RW",
                ],
                part1: 226,
                part2: 3509,
            },
        ];
        for t in cases {
            assert_eq!(part1(&t.lines), t.part1, "part 1 test case {:?}", t.lines);
            assert_eq!(part2(&t.lines), t.part2, "part 2 test case {:?}", t.lines);
        }
    }
}
