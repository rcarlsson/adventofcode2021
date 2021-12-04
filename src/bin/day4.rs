use std::{collections::HashMap, fs};

type Board = HashMap<(usize, usize), (u8, bool)>;

fn bingo(board: &Board) -> bool {
    for row in 0..5 {
        if board.get(&(0, row)).unwrap().1
            && board.get(&(1, row)).unwrap().1
            && board.get(&(2, row)).unwrap().1
            && board.get(&(3, row)).unwrap().1
            && board.get(&(4, row)).unwrap().1
        {
            return true;
        }
    }

    for col in 0..5 {
        if board.get(&(col, 0)).unwrap().1
            && board.get(&(col, 1)).unwrap().1
            && board.get(&(col, 2)).unwrap().1
            && board.get(&(col, 3)).unwrap().1
            && board.get(&(col, 4)).unwrap().1
        {
            return true;
        }
    }
    false
}

fn calc_res(board: &Board, nr: u8) -> u32 {
    let mut res = 0;
    for x in board.iter() {
        if !x.1 .1 {
            res += x.1 .0 as u32;
        }
    }
    res * nr as u32
}

fn part1(nrs: &[u8], boards: &mut [Board]) -> u32 {
    for nr in nrs {
        for board in boards.iter_mut() {
            for x in board.iter_mut() {
                if x.1 .0 == *nr {
                    x.1 .1 = true;
                    break;
                }
            }

            if bingo(board) {
                return calc_res(board, *nr);
            }
        }
    }
    panic!()
}

fn part2(nrs: &[u8], boards: &mut Vec<Board>) -> u32 {
    for nr in nrs {
        for board in boards.iter_mut() {
            for x in board.iter_mut() {
                if x.1 .0 == *nr {
                    x.1 .1 = true;
                    break;
                }
            }
        }

        if boards.len() == 1 {
            if bingo(&boards[0]) {
                return calc_res(&boards[0], *nr);
            }
        } else {
            boards.retain(|b| !bingo(b));
        }
    }
    panic!()
}

fn parse_input(input: Vec<&str>) -> (Vec<u8>, Vec<Board>) {
    let nrs: Vec<u8> = input[0]
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    let mut board: Board = HashMap::new();
    let mut row = 0;
    for line in &input[1..] {
        if line.is_empty() {
            continue;
        }

        for (idx, val) in line.split_whitespace().enumerate() {
            board.insert((idx, row), (val.parse().unwrap(), false));
        }

        row += 1;
        if row == 5 {
            boards.push(board);
            board = HashMap::new();
            row = 0;
        }
    }

    (nrs, boards)
}

fn main() {
    let contents = fs::read_to_string("day4.input").unwrap();
    let input: Vec<&str> = contents.split('\n').collect();
    let (nrs, mut boards) = parse_input(input);

    println!("1: {}", part1(&nrs, &mut boards.clone()));
    println!("2: {}", part2(&nrs, &mut boards));
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
    }

    #[test]
    fn test1() {
        let (nrs, mut boards) = parse_input(input());
        assert_eq!(4512, part1(&nrs, &mut boards));
    }

    #[test]
    fn test2() {
        let (nrs, mut boards) = parse_input(input());
        assert_eq!(1924, part2(&nrs, &mut boards));
    }
}
