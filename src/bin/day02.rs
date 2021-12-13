use std::fs;

fn part1(cmds: &[&str]) -> u32 {
    let mut dist = 0;
    let mut depth = 0;

    for cmd in cmds {
        let (dir, val) = cmd.split_once(' ').unwrap();
        let val = val.parse::<u32>().unwrap();
        match dir {
            "forward" => dist += val,
            "up" => depth -= val,
            "down" => depth += val,
            _ => panic!(),
        }
    }

    dist * depth
}

fn part2(cmds: &[&str]) -> u32 {
    let mut dist = 0;
    let mut depth = 0;
    let mut aim = 0;

    for cmd in cmds {
        let (dir, val) = cmd.split_once(' ').unwrap();
        let val = val.parse::<u32>().unwrap();
        match dir {
            "forward" => {
                dist += val;
                depth += aim * val;
            }
            "up" => aim -= val,
            "down" => aim += val,
            _ => panic!(),
        }
    }

    dist * depth
}

fn main() {
    let contents = fs::read_to_string("day02.input").unwrap();

    let input: Vec<&str> = contents.split('\n').collect();

    println!("1: {}", part1(&input));
    println!("2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
    }

    #[test]
    fn test1() {
        assert_eq!(150, part1(&input()));
    }

    #[test]
    fn test2() {
        assert_eq!(900, part2(&input()));
    }
}
