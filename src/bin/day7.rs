fn part1(input: &mut [i32]) -> i32 {
    input.sort_unstable();
    let median = input[input.len() / 2];
    input.iter().fold(0, |acc, x| acc + (x - median).abs())
}

fn part2(input: &[i32]) -> i32 {
    let arithmetic_sum = |n| (n + 1) * n / 2;

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    (min..=max).fold(i32::MAX, |opt, i| {
        opt.min(
            input
                .iter()
                .fold(0, |acc, x| acc + arithmetic_sum((x - i).abs())),
        )
    })
}

fn main() {
    let contents = std::fs::read_to_string("day7.input").unwrap();

    let mut input: Vec<i32> = contents.split(',').map(|s| s.parse().unwrap()).collect();

    println!("1: {}", part1(&mut input));
    println!("2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [i32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test1() {
        let mut input = INPUT;
        assert_eq!(37, part1(&mut input));
    }

    #[test]
    fn test2() {
        assert_eq!(168, part2(&INPUT));
    }
}
