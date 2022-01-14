use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

type Pos = (isize, isize);
type Risks = HashMap<Pos, u32>;
type Costs = BinaryHeap<(Reverse<u32>, Pos)>;

fn update_costs(costs: &mut Costs, visited: &mut HashSet<Pos>, risks: &Risks, end: Pos) -> Option<u32> {
    let (Reverse(cost), (x, y)) = costs.pop().unwrap();

    for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
        let pos = (x + dx, y + dy);
        if !visited.contains(&pos) && risks.contains_key(&pos) {
            costs.push((Reverse(cost + risks[&pos]), pos));
            visited.insert(pos);
            if pos == end {
                return Some(cost + risks[&pos]);
            }
        }
    }
    None
}

fn solution(risks: &Risks) -> u32 {
    let mut costs = Costs::new();
    let mut visited = HashSet::new();
    costs.push((Reverse(0), (0, 0)));
    let xmax = risks.keys().max_by(|(a, _), (b, _)| a.cmp(b)).unwrap().0;
    let ymax = risks.keys().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap().1;
    let end = (xmax, ymax);
    assert!(risks.contains_key(&end));
    loop {
        if let Some(res) = update_costs(&mut costs, &mut visited, risks, end) {
            return res;
        }
    }
}

fn parse_input(input: &[&str]) -> Risks {
    let mut risks = Risks::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            risks.insert((x as isize, y as isize), c.to_digit(10).unwrap());
        }
    }
    risks
}

fn expand_risks(risks: Risks) -> Risks {
    let mut new_risks = Risks::new();

    let xsize = risks.keys().max_by(|(a, _), (b, _)| a.cmp(b)).unwrap().0 + 1;
    let ysize = risks.keys().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap().1 + 1;

    for ((x, y), r) in risks {
        for dy in 0..5 {
            for dx in 0..5 {
                let new_risk = ((r + dx as u32 + dy as u32 - 1) % 9) + 1;
                new_risks.insert((x + dx * xsize, y + dy * ysize), new_risk);
            }
        }
    }
    new_risks
}

fn main() {
    let contents = std::fs::read_to_string("day15.input").unwrap();

    let input: Vec<&str> = contents.split('\n').collect();
    let risks = parse_input(&input);

    println!("1: {}", solution(&risks));
    println!("2: {}", solution(&expand_risks(risks)));
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581",
        ]
    }

    #[test]
    fn test1() {
        assert_eq!(40, solution(&parse_input(&input())));
    }

    #[test]
    fn test2() {
        let mut risks = parse_input(&input());
        risks = expand_risks(risks);
        assert_eq!(315, solution(&risks));
    }
}
