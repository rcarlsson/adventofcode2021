fn part1(_input: &[&str]) -> u32 {
    0
}

fn part2(_input: &[&str]) -> u32 {
    0
}

fn main() {
    let contents = std::fs::read_to_string("dayX.input").unwrap();

    let input: Vec<&str> = contents.split('\n').collect();

    println!("1: {}", part1(&input));
    println!("2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![""]
    }

    #[test]
    fn test1() {
        assert_eq!(0, part1(&input()));
    }

    #[test]
    fn test2() {
        assert_eq!(0, part2(&input()));
    }
}
