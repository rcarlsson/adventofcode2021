use std::{collections::HashMap, fs};

type Coord = (i16, i16);
type Line = (Coord, Coord);
type VentsMap = HashMap<Coord, u8>;

fn parse_line(line: &str) -> Line {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let start: Vec<i16> = parts[0].split(',').map(|s| s.parse().unwrap()).collect();
    let stop: Vec<i16> = parts[2].split(',').map(|s| s.parse().unwrap()).collect();

    ((start[0], start[1]), (stop[0], stop[1]))
}

fn generate_map(input: &[&str], diagonals: bool) -> VentsMap {
    let mut vents: VentsMap = HashMap::new();

    for line in input {
        let coords = parse_line(line);
        let xdir: i16 = match (coords.0 .0, coords.1 .0) {
            (a, b) if b > a => 1,
            (a, b) if b < a => -1,
            _ => 0,
        };
        let ydir: i16 = match (coords.0 .1, coords.1 .1) {
            (a, b) if b > a => 1,
            (a, b) if b < a => -1,
            _ => 0,
        };

        if !diagonals && xdir != 0 && ydir != 0 {
            continue;
        }

        let (mut x, mut y) = coords.0;

        loop {
            let val = vents.get(&(x, y)).unwrap_or(&0) + 1;
            vents.insert((x, y), val);

            if (x, y) == coords.1 {
                break;
            }

            x += xdir;
            y += ydir;
        }
    }

    vents
}

fn part1(input: &[&str]) -> usize {
    let mut vents = generate_map(input, false);
    vents.retain(|_, v| *v >= 2);
    vents.len()
}

fn part2(input: &[&str]) -> usize {
    let mut vents = generate_map(input, true);
    vents.retain(|_, v| *v >= 2);
    vents.len()
}

fn main() {
    let contents = fs::read_to_string("day05.input").unwrap();

    let input: Vec<&str> = contents.split('\n').collect();

    println!("1: {}", part1(&input));
    println!("2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
    }

    #[test]
    fn test1() {
        assert_eq!(5, part1(&input()));
    }

    #[test]
    fn test2() {
        assert_eq!(12, part2(&input()));
    }
}
