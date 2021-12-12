use std::collections::HashMap;

type Links<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_input<'a>(input: &[&'a str]) -> Links<'a> {
    let mut links: Links<'a> = HashMap::new();
    for line in input {
        let x: Vec<&str> = line.split('-').collect();
        if x.len() >= 2 {
            links.entry(x[0]).or_default().push(x[1]);
            links.entry(x[1]).or_default().push(x[0]);
        }
    }
    links
}

fn part1(links: &Links) -> usize {
    let mut paths: Vec<Vec<&str>> = vec![vec!["start"]];

    while !paths.iter().all(|p| p.contains(&"end")) {
        let mut new_paths: Vec<Vec<&str>> = Vec::new();
        for p in paths.iter() {
            if p.contains(&"end") {
                new_paths.push(p.clone());
                continue;
            }
            for l in &links[p.last().unwrap()] {
                if **l == l.to_uppercase() || !p.contains(l) {
                    new_paths.push(p.clone());
                    new_paths.last_mut().unwrap().push(l);
                }
            }
        }
        paths = new_paths;
    }
    paths.iter().filter(|p| p.contains(&"end")).count()
}

fn part2(links: &Links) -> usize {
    let mut paths: Vec<(Vec<&str>, bool)> = vec![(vec!["start"], false)];

    while !paths.iter().all(|p| p.0.contains(&"end")) {
        let mut new_paths: Vec<(Vec<&str>, bool)> = Vec::new();
        for p in paths.iter_mut() {
            if p.0.contains(&"end") {
                new_paths.push(p.clone());
                continue;
            }
            for l in links[p.0.last().unwrap()].iter().filter(|x| **x != "start") {
                if **l == l.to_uppercase()
                    || !p.1
                    || !p.0.contains(l)
                {
                    let mut new_path = p.0.clone();
                    let dup_small_cave = p.1 || **l == l.to_lowercase() && new_path.contains(l);
                    new_path.push(l);
                    new_paths.push((new_path, dup_small_cave));
                }
            }
        }
        paths = new_paths;
    }
    paths.iter().filter(|p| p.0.contains(&"end")).count()
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
