use std::fs;

fn part1(input: &[u32]) -> u32 {
    input
        .iter()
        .zip(input[1..].iter())
        .fold(0, |acc, (a, b)| if b > a { acc + 1 } else { acc })
}

fn part2(input: &[u32]) -> u32 {
    input
        .iter()
        .zip(input[3..].iter())
        .fold(0, |acc, (a, b)| if b > a { acc + 1 } else { acc })
}

fn main() {
    let contents = fs::read_to_string("day1.input").unwrap();

    let input: Vec<u32> = contents.split('\n').map(|s| s.parse().unwrap()).collect();

    println!("1: {}", part1(&input));
    println!("2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Vec<u32> {
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }

    #[test]
    fn test1() {
        assert_eq!(7, part1(&input()));
    }

    #[test]
    fn test2() {
        assert_eq!(5, part2(&input()));
    }
}
