use std::collections::HashMap;

type Template = HashMap<(char, char), usize>;
type Rules = HashMap<(char, char), char>;

fn update_template(template: Template, counters: &mut HashMap<char, usize>, rules: &Rules) -> Template {
    let mut res = Template::new();
    for ((k1, k2), c) in template {
        let k3 = rules[&(k1, k2)];
        *res.entry((k1, k3)).or_default() += c;
        *res.entry((k3, k2)).or_default() += c;
        *counters.entry(k3).or_default() += c;
    }
    res
}

fn solution(template: &str, rules: &Rules, count: usize) -> usize {
    let mut res: HashMap<char, usize> = HashMap::new();
    let mut temp: Template = Template::new();
    let mut prev = template.chars().next().unwrap();
    *res.entry(prev).or_default() += 1;
    for c in template.chars().skip(1) {
        *temp.entry((prev, c)).or_default() += 1;
        *res.entry(c).or_default() += 1;
        prev = c;
    }

    for _ in 0..count {
        temp = update_template(temp, &mut res, rules);
    }

    let (mx, mn) = res.values().fold((usize::MIN, usize::MAX), |(mx, mn), v| (mx.max(*v), mn.min(*v)));
    mx - mn
}

fn parse_input<'a>(input: &[&'a str]) -> (&'a str, Rules) {
    let mut rules = HashMap::new();
    for line in &input[2..] {
        if let Some(rule) = line.split_once(" -> ") {
            let mut key = rule.0.chars();
            rules.insert((key.next().unwrap(), key.next().unwrap()), rule.1.chars().next().unwrap());
        }
    }

    (input[0], rules)
}

fn main() {
    let contents = std::fs::read_to_string("day14.input").unwrap();

    let input: Vec<&str> = contents.split('\n').collect();
    let (template, rules) = parse_input(&input);

    println!("1: {}", solution(template, &rules, 10));
    println!("2: {}", solution(template, &rules, 40));
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "NNCB",
            "",
            "CH -> B",
            "HH -> N",
            "CB -> H",
            "NH -> C",
            "HB -> C",
            "HC -> B",
            "HN -> C",
            "NN -> C",
            "BH -> H",
            "NC -> B",
            "NB -> B",
            "BN -> B",
            "BB -> N",
            "BC -> B",
            "CC -> N",
            "CN -> C",
        ]
    }

    #[test]
    fn test1() {
        let (template, rules) = parse_input(&input());
        assert_eq!(1588, solution(template, &rules, 10));
    }

    #[test]
    fn test2() {
        let (template, rules) = parse_input(&input());
        assert_eq!(2188189693529, solution(template, &rules, 40));
    }
}
