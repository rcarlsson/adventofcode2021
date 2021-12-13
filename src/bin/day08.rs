use itertools::Itertools;
use std::collections::HashMap;

fn part1(input: &[&str]) -> u32 {
    let mut res = 0;
    for x in input.iter() {
        let tmp = x.split(" | ").collect::<Vec<&str>>();
        if tmp.len() < 2 {
            continue;
        }
        let out = tmp[1].split_whitespace();
        for out_dig in out {
            if [2, 3, 4, 7].contains(&out_dig.len()) {
                res += 1;
            }
        }
    }

    res
}

#[cfg(test)]
fn part2_slow(input: &[&str]) -> usize {
    let numbers = [
        vec![0, 1, 2, 4, 5, 6],
        vec![2, 5],
        vec![0, 2, 3, 4, 6],
        vec![0, 2, 3, 5, 6],
        vec![1, 2, 3, 5],
        vec![0, 1, 3, 5, 6],
        vec![0, 1, 3, 4, 5, 6],
        vec![0, 2, 5],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 5, 6],
    ];

    let mut inp: Vec<Vec<&str>> = Vec::new();
    let mut out: Vec<Vec<&str>> = Vec::new();
    for x in input.iter() {
        let tmp = x.split(" | ").collect::<Vec<&str>>();
        if tmp.len() < 2 {
            continue;
        }
        inp.push(tmp[0].split_whitespace().collect());
        out.push(tmp[1].split_whitespace().collect());
    }

    (0..out.len()).fold(0, |acc, idx| {
        let key = "abcdefg"
            .chars()
            .permutations(7)
            .find(|p| {
                inp[idx].iter().chain(out[idx].iter()).all(|dig| {
                    numbers.contains(
                        &dig.chars()
                            .map(|d| p.iter().position(|c| *c == d).unwrap())
                            .sorted()
                            .collect(),
                    )
                })
            })
            .unwrap();
        acc + out[idx].iter().fold(0, |acc, d| {
            acc * 10
                + numbers
                    .iter()
                    .position(|s| {
                        *s == d
                            .chars()
                            .map(|c| key.iter().position(|p| *p == c).unwrap())
                            .sorted()
                            .collect::<Vec<usize>>()
                    })
                    .unwrap()
        })
    })
}

fn part2_fast(input: &[&str]) -> usize {
    let mut inp: Vec<Vec<&str>> = Vec::new();
    let mut out: Vec<Vec<&str>> = Vec::new();
    for x in input.iter() {
        let tmp = x.split(" | ").collect::<Vec<&str>>();
        if tmp.len() < 2 {
            continue;
        }
        inp.push(tmp[0].split_whitespace().collect());
        out.push(tmp[1].split_whitespace().collect());
    }

    let numbers = [
        vec![0, 1, 2, 4, 5, 6],
        vec![2, 5],
        vec![0, 2, 3, 4, 6],
        vec![0, 2, 3, 5, 6],
        vec![1, 2, 3, 5],
        vec![0, 1, 3, 5, 6],
        vec![0, 1, 3, 4, 5, 6],
        vec![0, 2, 5],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 5, 6],
    ];
    let all = vec![0, 1, 2, 3, 4, 5, 6];

    let mut res = 0;
    for idx in 0..out.len() {
        let mut segments: HashMap<char, Vec<u8>> = HashMap::new();

        for dig in inp[idx].iter().chain(out[idx].iter()) {
            let num = match dig.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                _ => continue,
            };

            for c in "abcdefg".chars() {
                segments
                    .entry(c)
                    .or_insert_with(|| all.clone())
                    .retain(|x| dig.chars().contains(&c) == numbers[num].contains(x));
            }
        }

        let mut out_dig = 0;
        for dig in out[idx].iter() {
            let seg: Vec<Vec<u8>> = dig
                .chars()
                .map(|c| segments.get(&c).unwrap_or(&all).to_vec())
                .collect();

            let mut certain: Vec<u8> = Vec::new();
            let mut uncertain: Vec<Vec<u8>> = Vec::new();
            for s in &seg {
                if s.len() == seg.iter().filter(|x| *x == s).count() {
                    certain.append(&mut s.to_vec());
                } else {
                    uncertain.push(s.to_vec());
                }
            }

            certain = certain
                .iter()
                .unique()
                .collect::<Vec<&u8>>()
                .iter()
                .map(|&x| *x)
                .collect();
            assert!(uncertain.len() <= 2);
            out_dig *= 10;
            if uncertain.is_empty() {
                certain.sort_unstable();
                out_dig += numbers.iter().position(|x| *x == certain).unwrap();
            } else {
                for a in &uncertain[0] {
                    certain.push(*a);
                    if uncertain.len() == 1 {
                        certain.sort_unstable();
                        if let Some(n) = numbers.iter().position(|x| *x == certain) {
                            out_dig += n;
                        }
                    } else {
                        for b in &uncertain[1] {
                            certain.push(*b);
                            certain.sort_unstable();
                            if let Some(n) = numbers.iter().position(|x| *x == certain) {
                                out_dig += n;
                            }
                            certain.retain(|x| x != b);
                        }
                    }
                    certain.retain(|x| x != a);
                }
            }
        }
        res += out_dig;
    }
    res
}

fn main() {
    let contents = std::fs::read_to_string("day08.input").unwrap();

    let input: Vec<&str> = contents.split('\n').collect();

    println!("1: {}", part1(&input));
    println!("2: {}", part2_fast(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    fn input() -> Vec<&'static str> {
        vec![
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
        ]
    }

    #[test]
    fn test1() {
        assert_eq!(26, part1(&input()));
    }

    #[test]
    fn test2_slow() {
        let inp = [
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        ];
        assert_eq!(5353, part2_slow(&inp));

        assert_eq!(61229, part2_slow(&input()));
    }

    #[test]
    fn test2_fast() {
        let inp = [
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        ];
        assert_eq!(5353, part2_fast(&inp));

        assert_eq!(61229, part2_fast(&input()));
    }
}
