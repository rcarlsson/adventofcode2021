use std::fs;

fn get_fish_count(fishes: &[usize], ndays: usize) -> usize {
    let mut l = [0; 9];
    for (i, r) in l.iter_mut().enumerate() {
        *r = fishes.iter().filter(|x| **x == i).count();
    }

    for d in 0..ndays {
        l[(d + 7) % 9] += l[d % 9];
    }
    l.iter().sum()
}

fn main() {
    let contents = fs::read_to_string("day6.input").unwrap();

    let input: Vec<usize> = contents.split(',').map(|s| s.parse().unwrap()).collect();

    println!("1: {}", get_fish_count(&input, 80));
    println!("2: {}", get_fish_count(&input, 256));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = vec![3, 4, 3, 1, 2];
        assert_eq!(26, get_fish_count(&input, 18));
        assert_eq!(5934, get_fish_count(&input, 80));
        assert_eq!(26984457539, get_fish_count(&input, 256));
    }
}
