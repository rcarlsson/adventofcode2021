use std::collections::HashMap;

type OctoMap = HashMap<(i8, i8), u32>;

fn parse_input(input: &[&str]) -> OctoMap {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i8, y as i8), c.to_digit(10).unwrap()))
        })
        .collect::<OctoMap>()
}

fn update(octomap: &mut OctoMap) {
    let mut upd: Vec<(i8, i8)> = (0..10).flat_map(|x| (0..10).map(move |y| (x, y))).collect();

    while !upd.is_empty() {
        let mut next_upd: Vec<(i8, i8)> = Vec::new();
        for (x, y) in upd {
            if let Some(v) = octomap.get_mut(&(x, y)) {
                if *v <= 9 {
                    *v += 1;
                    if *v > 9 {
                        for (dx, dy) in [
                            (-1, -1),
                            (-1, 0),
                            (-1, 1),
                            (0, -1),
                            (0, 1),
                            (1, -1),
                            (1, 0),
                            (1, 1),
                        ]
                        .iter()
                        {
                            next_upd.push((x + dx, y + dy));
                        }
                    }
                }
            }
        }
        upd = next_upd;
    }
}

fn part1(octomap: &mut OctoMap) -> u32 {
    let mut res = 0;
    for _ in 0..100 {
        update(octomap);
        for v in octomap.values_mut() {
            *v %= 10;
            if *v == 0 {
                res += 1;
            }
        }
    }

    res
}

fn part2(octomap: &mut OctoMap) -> u32 {
    let mut res = 0;
    while !octomap.values().all(|v| *v == 0) {
        update(octomap);
        for v in octomap.values_mut() {
            *v %= 10;
        }
        res += 1;
    }

    res
}

fn main() {
    let contents = std::fs::read_to_string("day11.input").unwrap();

    let input: Vec<&str> = contents.split('\n').collect();
    let mut octomap = parse_input(&input);

    println!("1: {}", part1(&mut octomap.clone()));
    println!("2: {}", part2(&mut octomap));
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ]
    }

    #[test]
    fn test1() {
        assert_eq!(1656, part1(&mut parse_input(&input())));
    }

    #[test]
    fn test2() {
        assert_eq!(195, part2(&mut parse_input(&input())));
    }
}
