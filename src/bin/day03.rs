use std::fs;

fn part1(nrs: &[&str]) -> u32 {
    let mut ones = Vec::new();
    let mut zeros = Vec::new();

    for _ in 0..nrs[0].len() {
        ones.push(0);
        zeros.push(0);
    }

    for nr in nrs {
        for (idx, c) in nr.chars().enumerate() {
            match c {
                '1' => ones[idx] += 1,
                '0' => zeros[idx] += 1,
                _ => panic!(),
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for idx in 0..ones.len() {
        gamma <<= 1;
        epsilon <<= 1;
        if ones[idx] > zeros[idx] {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    gamma * epsilon
}

fn part2(oxygen: &mut Vec<&str>, co2: &mut Vec<&str>) -> u32 {
    for i in 0.. {
        let ones = oxygen.iter().fold(0, |acc, x| {
            if x.chars().nth(i).unwrap() == '1' {
                acc + 1
            } else {
                acc
            }
        });
        let zeros = oxygen.len() - ones;
        let most_common = if ones >= zeros { '1' } else { '0' };
        oxygen.retain(|o| o.chars().nth(i).unwrap() == most_common);
        if oxygen.len() == 1 {
            break;
        }
    }

    for i in 0.. {
        let ones = co2.iter().fold(0, |acc, x| {
            if x.chars().nth(i).unwrap() == '1' {
                acc + 1
            } else {
                acc
            }
        });
        let zeros = co2.len() - ones;
        let most_common = if ones >= zeros { '0' } else { '1' };
        co2.retain(|c| c.chars().nth(i).unwrap() == most_common);
        if co2.len() == 1 {
            break;
        }
    }

    let mut o = 0;
    for x in oxygen[0].chars() {
        o <<= 1;
        if x == '1' {
            o += 1
        }
    }
    let mut c = 0;
    for x in co2[0].chars() {
        c <<= 1;
        if x == '1' {
            c += 1
        }
    }
    o * c
}

fn main() {
    let contents = fs::read_to_string("day03.input").unwrap();

    let input: Vec<&str> = contents.split('\n').collect();

    println!("1: {}", part1(&input));
    println!("2: {}", part2(&mut input.clone(), &mut input.clone()));
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
    }

    #[test]
    fn test1() {
        assert_eq!(198, part1(&input()));
    }

    #[test]
    fn test2() {
        let input = input();
        assert_eq!(230, part2(&mut input.clone(), &mut input.clone()));
    }
}
