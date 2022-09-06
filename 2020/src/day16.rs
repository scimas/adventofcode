use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::{
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule<T: PartialOrd> {
    name: String,
    ranges: Vec<RangeInclusive<T>>,
}

impl<T: FromStr + PartialOrd> From<&str> for Rule<T>
where
    T::Err: Debug,
{
    fn from(s: &str) -> Self {
        let mut name_range_split = s.split(": ");
        let name = name_range_split.next().expect("Rule string not okay");
        let ranges_str = name_range_split.next().expect("Rule string not okay");
        let mut ranges: Vec<RangeInclusive<T>> = Vec::new();
        for range in ranges_str.split(" or ") {
            let mut bounds = range.split('-');
            let start: T = bounds.next().unwrap().parse().unwrap();
            let end: T = bounds.next().unwrap().parse().unwrap();
            ranges.push(start..=end);
        }
        Rule {
            name: name.to_string(),
            ranges,
        }
    }
}

impl<T: PartialOrd> Rule<T> {
    fn follows(&self, val: T) -> bool {
        for range in &self.ranges {
            if range.contains(&val) {
                return true;
            }
        }
        false
    }
}

fn load_data() -> (Vec<Rule<usize>>, Vec<u64>, Vec<Vec<u64>>) {
    let f = File::open("res/input16.txt").expect("Couldn't open day 16 input");
    let reader = BufReader::new(f);
    let mut line_iter = reader.lines();

    let mut rules: Vec<Rule<usize>> = Vec::new();
    for line in line_iter.by_ref() {
        let s = line.expect("Couldn't read line from day 16 input");
        if s.is_empty() {
            break;
        }
        rules.push(Rule::from(s.as_str()));
    }
    let mut line_iter = line_iter.skip(1);
    let my_ticket: Vec<u64> = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let line_iter = line_iter.skip(2);
    let nearby_tickets: Vec<Vec<u64>> = line_iter
        .map(|line| {
            line.expect("Couldn't read line from day 16 input")
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    (rules, my_ticket, nearby_tickets)
}

pub fn part1() -> u64 {
    let (rules, _, nearby_tickets) = load_data();
    let mut invalid_tickets: Vec<Vec<u64>> = Vec::new();
    for ticket in nearby_tickets {
        let mut invalid_values: Vec<u64> = Vec::new();
        for val in ticket {
            let mut follows = false;
            for rule in &rules {
                if rule.follows(val as usize) {
                    follows = true;
                    break;
                }
            }
            if !follows {
                invalid_values.push(val);
            }
        }
        if !invalid_values.is_empty() {
            invalid_tickets.push(invalid_values);
        }
    }
    invalid_tickets
        .iter()
        .fold(0, |acc, x| acc + x.iter().sum::<u64>())
}

pub fn part2() -> u64 {
    let (rules, my_ticket, nearby_tickets) = load_data();
    let mut place_rules: Vec<HashSet<Rule<usize>>> =
        vec![rules.iter().cloned().collect(); my_ticket.len()];
    let mut valid_tickets: Vec<usize> = Vec::new();

    for (idx, ticket) in nearby_tickets.iter().enumerate() {
        let mut invalid = false;
        for val in ticket {
            let mut follows = false;
            for rule in &rules {
                if rule.follows(*val as usize) {
                    follows = true;
                    break;
                }
            }
            if !follows {
                invalid = true;
            }
        }
        if !invalid {
            valid_tickets.push(idx);
        }
    }
    for idx in valid_tickets {
        let mut this_ticket_rules: Vec<HashSet<Rule<usize>>> =
            vec![HashSet::new(); my_ticket.len()];
        for (i, val) in nearby_tickets[idx].iter().enumerate() {
            for rule in &rules {
                if rule.follows(*val as usize) {
                    this_ticket_rules[i].insert(rule.clone());
                }
            }
        }
        for i in 0..my_ticket.len() {
            place_rules[i] = place_rules[i]
                .intersection(&this_ticket_rules[i])
                .cloned()
                .collect();
        }
    }
    let mut final_rules: Vec<(usize, HashSet<Rule<usize>>)> = Vec::new();
    while place_rules.iter().any(|set| !set.is_empty()) {
        let single_rule = place_rules
            .iter()
            .cloned()
            .enumerate()
            .filter(|(_, set)| !set.is_empty())
            .min_by_key(|(_, set)| set.len())
            .unwrap();
        final_rules.push(single_rule.clone());
        for set in place_rules.iter_mut() {
            *set = set.difference(&single_rule.1).cloned().collect();
        }
    }
    let mut prod: u64 = 1;
    for (idx, ruleset) in final_rules {
        let rule = ruleset.iter().next().unwrap();
        if rule.name.starts_with("departure") {
            prod *= my_ticket[idx];
        }
    }
    prod
}
