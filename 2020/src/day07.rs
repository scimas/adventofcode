use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Bag(String, String);

impl Bag {
    fn new(quality: &str, colour: &str) -> Self {
        Bag(quality.to_string(), colour.to_string())
    }
}

#[derive(PartialEq, Debug)]
struct Rule {
    bag: Bag,
    contents: Vec<(Bag, u32)>,
}

impl Rule {
    fn new(bag: Bag, contents: Vec<(Bag, u32)>) -> Self {
        Rule { bag, contents }
    }
}

fn string_to_rule(s: &str) -> Rule {
    let s = s.strip_suffix('.').unwrap();
    let mut outer_inner_split = s.split(" bags contain ");
    let mut quality_colour_split = outer_inner_split.next().unwrap().split(' ');
    let quality = quality_colour_split.next().unwrap().to_string();
    let colour = quality_colour_split.next().unwrap().to_string();
    let outer_bag = Bag(quality, colour);
    let mut contents: Vec<(Bag, u32)> = Vec::new();
    let inner_bags = outer_inner_split.next().unwrap();
    if !inner_bags.contains("no other bags") {
        for bag_quant_string in inner_bags.split(", ") {
            let mut char_iter = bag_quant_string.chars();
            let quantity: u32 = char_iter
                .by_ref()
                .take_while(|ch| ch.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap();
            let bag_string: String = char_iter.collect();
            let bag_string = match bag_string.strip_suffix(" bags") {
                Some(s) => s,
                None => bag_string.strip_suffix(" bag").unwrap(),
            };
            let mut bag_iter = bag_string.split(' ');
            let quality = bag_iter.next().unwrap().to_string();
            let colour = bag_iter.next().unwrap().to_string();
            let bag = Bag(quality, colour);
            contents.push((bag, quantity));
        }
    }
    Rule::new(outer_bag, contents)
}

fn bag_contains_bag<'a>(
    outer_bag: &'a Bag,
    inner_bag: &Bag,
    rules: &'a [Rule],
    found_bags: &mut HashSet<&'a Bag>,
) -> bool {
    if found_bags.contains(&outer_bag) {
        return true;
    }
    let mut contains = false;
    for rule in rules {
        if &rule.bag == outer_bag {
            for content in &rule.contents {
                contains |= &content.0 == inner_bag
                    || bag_contains_bag(&content.0, inner_bag, rules, found_bags);
                if contains {
                    found_bags.insert(outer_bag);
                    return contains;
                }
            }
            break;
        }
    }
    contains
}

fn load_rules() -> Vec<Rule> {
    let f = File::open("res/input07.txt").expect("Couldn't read day 7 input");
    let reader = BufReader::new(f);
    reader
        .lines()
        .map(|line| string_to_rule(&line.expect("Couldn't read line from day 7 input")))
        .collect()
}

pub fn part1() -> u32 {
    let rules = load_rules();
    let mut count: u32 = 0;
    let shiny_gold = Bag::new("shiny", "gold");
    let mut found_bags: HashSet<&Bag> = HashSet::new();
    for rule in &rules {
        count += bag_contains_bag(&rule.bag, &shiny_gold, &rules, &mut found_bags) as u32;
    }
    count
}

fn contents(bag: &Bag, rules: &[Rule]) -> usize {
    let top_rule = rules.iter().find(|rule| &rule.bag == bag).unwrap();
    let mut count: usize = 0;
    for (inner_bag, quantity) in &top_rule.contents {
        count += (*quantity as usize) * (1 + contents(&inner_bag, rules));
    }
    count
}

pub fn part2() -> usize {
    let rules = load_rules();
    let shiny_gold = Bag::new("shiny", "gold");
    contents(&shiny_gold, &rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example() -> &'static str {
        "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"
    }

    #[test]
    fn rule_creating() {
        let rules_string = get_example();
        let rules = vec![
            Rule::new(
                Bag::new("light", "red"),
                vec![
                    (Bag::new("bright", "white"), 1),
                    (Bag::new("muted", "yellow"), 2),
                ],
            ),
            Rule::new(
                Bag::new("dark", "orange"),
                vec![
                    (Bag::new("bright", "white"), 3),
                    (Bag::new("muted", "yellow"), 4),
                ],
            ),
            Rule::new(
                Bag::new("bright", "white"),
                vec![(Bag::new("shiny", "gold"), 1)],
            ),
            Rule::new(
                Bag::new("muted", "yellow"),
                vec![
                    (Bag::new("shiny", "gold"), 2),
                    (Bag::new("faded", "blue"), 9),
                ],
            ),
            Rule::new(
                Bag::new("shiny", "gold"),
                vec![
                    (Bag::new("dark", "olive"), 1),
                    (Bag::new("vibrant", "plum"), 2),
                ],
            ),
            Rule::new(
                Bag::new("dark", "olive"),
                vec![
                    (Bag::new("faded", "blue"), 3),
                    (Bag::new("dotted", "black"), 4),
                ],
            ),
            Rule::new(
                Bag::new("vibrant", "plum"),
                vec![
                    (Bag::new("faded", "blue"), 5),
                    (Bag::new("dotted", "black"), 6),
                ],
            ),
            Rule::new(Bag::new("faded", "blue"), vec![]),
            Rule::new(Bag::new("dotted", "black"), vec![]),
        ];
        rules_string
            .lines()
            .zip(rules.iter())
            .for_each(|(s, r)| assert_eq!(string_to_rule(s), *r));
    }

    #[test]
    fn shiny_in_examples() {
        let rules_string = get_example();
        let rules: Vec<Rule> = rules_string
            .lines()
            .map(|line| string_to_rule(line))
            .collect();
        let shiny_gold = Bag::new("shiny", "gold");
        let bright_white = Bag::new("bright", "white");
        let muted_yellow = Bag::new("muted", "yellow");
        let dark_orange = Bag::new("dark", "orange");
        let light_red = Bag::new("light", "red");
        let dark_olive = Bag::new("dark", "olive");
        let vibrant_plum = Bag::new("vibrant", "plum");
        let faded_blue = Bag::new("faded", "blue");
        let dotted_black = Bag::new("dotted", "black");

        let mut found_bags: HashSet<&Bag> = HashSet::new();
        assert!(bag_contains_bag(
            &bright_white,
            &shiny_gold,
            &rules,
            &mut found_bags
        ));
        let mut found_bags: HashSet<&Bag> = HashSet::new();
        assert!(bag_contains_bag(
            &muted_yellow,
            &shiny_gold,
            &rules,
            &mut found_bags
        ));
        let mut found_bags: HashSet<&Bag> = HashSet::new();
        assert!(bag_contains_bag(
            &dark_orange,
            &shiny_gold,
            &rules,
            &mut found_bags
        ));
        let mut found_bags: HashSet<&Bag> = HashSet::new();
        assert!(bag_contains_bag(
            &light_red,
            &shiny_gold,
            &rules,
            &mut found_bags
        ));
        let mut found_bags: HashSet<&Bag> = HashSet::new();
        assert!(!bag_contains_bag(
            &dark_olive,
            &shiny_gold,
            &rules,
            &mut found_bags
        ));
        let mut found_bags: HashSet<&Bag> = HashSet::new();
        assert!(!bag_contains_bag(
            &vibrant_plum,
            &shiny_gold,
            &rules,
            &mut found_bags
        ));
        let mut found_bags: HashSet<&Bag> = HashSet::new();
        assert!(!bag_contains_bag(
            &faded_blue,
            &shiny_gold,
            &rules,
            &mut found_bags
        ));
        let mut found_bags: HashSet<&Bag> = HashSet::new();
        assert!(!bag_contains_bag(
            &dotted_black,
            &shiny_gold,
            &rules,
            &mut found_bags
        ));
    }

    #[test]
    fn content_quantity() {
        let rules_string = get_example();
        let rules: Vec<Rule> = rules_string
            .lines()
            .map(|line| string_to_rule(line))
            .collect();
        let shiny_gold = Bag::new("shiny", "gold");
        assert_eq!(contents(&shiny_gold, &rules), 32);

        let rules_string = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";
        let rules: Vec<Rule> = rules_string
            .lines()
            .map(|line| string_to_rule(line))
            .collect();
        assert_eq!(contents(&shiny_gold, &rules), 126);
    }
}
