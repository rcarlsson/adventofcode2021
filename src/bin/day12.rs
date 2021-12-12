use std::collections::{HashMap, HashSet};

type Links<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn parse_input<'a>(input: &[&'a str]) -> Links<'a> {
    let mut links: Links<'a> = HashMap::new();
    for line in input {
        if let Some((a, b)) = line.split_once('-') {
            if b != "start" {
                links.entry(a).or_default().insert(b);
            }
            if a != "start" {
                links.entry(b).or_default().insert(a);
            }
        }
    }
    links
}

fn part1(links: &Links) -> usize {
    let mut paths: Vec<(&str, HashSet<&str>)> = Vec::new();
    let mut init_set = HashSet::new();
    init_set.insert("start");
    paths.push(("start", init_set));

    let mut res = 0;
    while let Some((pos, small)) = paths.pop() {
        for l in &links[pos] {
            if *l == "end" {
                res += 1;
            } else if !small.contains(l) {
                let mut new_small = small.clone();
                if *l == l.to_lowercase() {
                    new_small.insert(l);
                }
                paths.push((l, new_small));
            }
        }
    }
    res
}

fn part2(links: &Links) -> usize {
    let mut paths: Vec<(&str, HashSet<&str>, bool)> = Vec::new();
    let mut init_set = HashSet::new();
    init_set.insert("start");
    paths.push(("start", init_set, false));

    let mut res = 0;
    while let Some((pos, small, dup_small)) = paths.pop() {
        for l in &links[pos] {
            if *l == "end" {
                res += 1;
            } else if !dup_small || !small.contains(l) {
                let mut new_small = small.clone();
                let mut new_dup_small = dup_small;
                if *l == l.to_lowercase() {
                    new_small.insert(l);
                    new_dup_small |= small.contains(l);
                }
                paths.push((l, new_small, new_dup_small));
            }
        }
    }
    res
}

fn main() {
    let contents = std::fs::read_to_string("day12.input").unwrap();

    let input: Vec<&str> = contents.split('\n').collect();
    let links = parse_input(&input);

    println!("1: {}", part1(&links));
    println!("2: {}", part2(&links));
}

#[cfg(test)]
mod test {
    use super::*;

    fn ex1() -> Vec<&'static str> {
        vec!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"]
    }

    fn ex2() -> Vec<&'static str> {
        vec![
            "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa",
            "kj-HN", "kj-dc",
        ]
    }

    fn ex3() -> Vec<&'static str> {
        vec![
            "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
            "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
        ]
    }

    #[test]
    fn test1() {
        assert_eq!(10, part1(&parse_input(&ex1())));
        assert_eq!(19, part1(&parse_input(&ex2())));
        assert_eq!(226, part1(&parse_input(&ex3())));
    }

    #[test]
    fn test2() {
        assert_eq!(36, part2(&parse_input(&ex1())));
        assert_eq!(103, part2(&parse_input(&ex2())));
        assert_eq!(3509, part2(&parse_input(&ex3())));
    }
}
