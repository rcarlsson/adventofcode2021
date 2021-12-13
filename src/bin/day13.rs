use std::collections::HashSet;

type Dots = HashSet<(u16, u16)>;
type Folds = Vec<(char, u16)>;

fn parse_input(input: &[&str]) -> (Dots, Folds) {
    let mut dots: Dots = Dots::new();
    let mut folds: Folds = Folds::new();

    for line in input {
        if let Some((prefix, n)) = line.split_once('=') {
            folds.push((prefix.chars().last().unwrap(), n.parse().unwrap()));
        } else if let Some((x, y)) = line.split_once(',') {
            dots.insert((x.parse().unwrap(), y.parse().unwrap()));
        }
    }

    (dots, folds)
}

fn fold_paper(dots: Dots, fold: (char, u16)) -> Dots {
    dots.iter()
        .map(|(x, y)| {
            (
                if fold.0 == 'x' && *x > fold.1 {
                    2 * fold.1 - *x
                } else {
                    *x
                },
                if fold.0 == 'y' && *y > fold.1 {
                    2 * fold.1 - *y
                } else {
                    *y
                },
            )
        })
        .collect()
}

fn print_dots(dots: &Dots) {
    let (xmax, _) = dots.iter().max_by_key(|(x, _)| x).unwrap();
    let (_, ymax) = dots.iter().max_by_key(|(_, y)| y).unwrap();
    for y in 0..=*ymax {
        for x in 0..=*xmax {
            print!(
                "{}",
                if dots.contains(&(x, y)) {
                    "\u{2588}\u{2588}"
                } else {
                    "  "
                }
            );
        }
        println!();
    }
}

fn main() {
    let contents = std::fs::read_to_string("day13.input").unwrap();

    let input: Vec<&str> = contents.split('\n').collect();
    let (mut dots, folds) = parse_input(&input);

    dots = fold_paper(dots, folds[0]);
    println!("1: {}", dots.len());
    dots = folds[1..].iter().fold(dots, |d, f| fold_paper(d, *f));
    println!("2:");
    print_dots(&dots);
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
        ]
    }

    #[test]
    fn test() {
        let (mut dots, folds) = parse_input(&input());
        assert_eq!(18, dots.len());
        dots = fold_paper(dots, folds[0]);
        assert_eq!(17, dots.len());
        dots = folds[1..].iter().fold(dots, |d, f| fold_paper(d, *f));
        print_dots(&dots);
    }
}
