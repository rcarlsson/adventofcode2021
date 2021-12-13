use std::collections::HashMap;

type Tubes = HashMap<(i16, i16), u32>;

fn parse_input(input: &[&str]) -> Tubes {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i16, y as i16), c.to_digit(10).unwrap()))
        })
        .collect::<Tubes>()
}

fn part1(tubes: &Tubes) -> u32 {
    tubes
        .iter()
        .filter_map(|((x, y), h)| {
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .all(|(dx, dy)| h < tubes.get(&(x + dx, y + dy)).unwrap_or(&9))
                .then(|| *h)
        })
        .fold(0, |acc, h| acc + h + 1)
}

fn part2(tubes: &Tubes) -> usize {
    let mut basins: HashMap<(i16, i16), Vec<(i16, i16)>> =
        tubes.iter().map(|t| (*t.0, vec![*t.0])).collect();

    for n in 0..=8 {
        for t in tubes.iter().filter(|t| *t.1 == 8 - n) {
            let (x, y) = *t.0;
            let pos = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for p in pos {
                if let Some(v) = tubes.get(&p) {
                    if t.1 > v {
                        let mut vec = basins.remove(t.0).unwrap();
                        basins.entry(p).and_modify(|v| v.append(&mut vec));
                        break;
                    }
                }
            }
        }
    }

    let mut b: Vec<usize> = basins.values().map(|v| v.len()).collect();
    b.sort_unstable();
    b.reverse();
    b[0] * b[1] * b[2]
}

fn main() {
    let contents = std::fs::read_to_string("day09.input").unwrap();

    let input: Vec<&str> = contents.split('\n').collect();
    let tubes = parse_input(&input);

    println!("1: {}", part1(&tubes));
    println!("2: {}", part2(&tubes));
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ]
    }

    #[test]
    fn test1() {
        assert_eq!(15, part1(&parse_input(&input())));
    }

    #[test]
    fn test2() {
        assert_eq!(1134, part2(&parse_input(&input())));
    }
}
