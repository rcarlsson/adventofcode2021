fn match_char(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!(),
    }
}

fn find_illegal(line: &str) -> Option<char> {
    let mut open: Vec<char> = Vec::new();

    for c in line.chars() {
        if "([{<".contains(c) {
            open.push(c);
        } else if match_char(c) != open.pop().unwrap() {
            return Some(c);
        }
    }

    None
}

fn complete_line(line: &str) -> Vec<char> {
    let mut open: Vec<char> = Vec::new();

    for c in line.chars() {
        if "([{<".contains(c) {
            open.push(c);
        } else {
            let _ = open.pop();
        }
    }

    open.reverse();
    open.iter().map(|c| match_char(*c)).collect()
}

fn part1(input: &[&str]) -> u32 {
    let mut res = 0;
    for line in input {
        if let Some(c) = find_illegal(line) {
            res += match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!(),
            }
        }
    }
    res
}

fn part2(input: &[&str]) -> u64 {
    let mut res: Vec<u64> = input
        .iter()
        .filter(|l| find_illegal(l).is_none())
        .map(|l| {
            complete_line(l).iter().fold(0, |acc, c| {
                acc * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!(),
                    }
            })
        })
        .collect();

    res.sort_unstable();
    res[res.len() / 2]
}

fn main() {
    let contents = std::fs::read_to_string("day10.input").unwrap();

    let input: Vec<&str> = contents.split('\n').collect();

    println!("1: {}", part1(&input));
    println!("2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ]
    }

    #[test]
    fn test1() {
        assert_eq!(26397, part1(&input()));
    }

    #[test]
    fn test2() {
        assert_eq!(288957, part2(&input()));
    }
}
