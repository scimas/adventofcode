use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Rule {
    Base(char),
    SubRule(Vec<Vec<usize>>),
}

impl Rule {
    fn check<'a>(&self, s: &'a str, rules: &HashMap<usize, Rule>) -> (bool, &'a str) {
        match self {
            Rule::Base(c) => {
                if let Some(rem) = s.strip_prefix(*c) {
                    (true, rem)
                } else {
                    (false, s)
                }
            }
            Rule::SubRule(opts) => {
                let mut res = false;
                let mut rem = s;
                for opt in opts {
                    let mut inter_rem = s;
                    let mut inter_res = true;
                    for rule_num in opt {
                        let (this_res, stripped) = rules[rule_num].check(inter_rem, rules);
                        inter_res = inter_res && this_res;
                        if !inter_res {
                            break;
                        }
                        inter_rem = stripped;
                    }
                    res = res || inter_res;
                    if res {
                        rem = inter_rem;
                        break;
                    }
                }
                (res, rem)
            }
        }
    }
}

fn rule_from_str(s: &str) -> (usize, Rule) {
    let mut parts = s.split(": ");
    let rule_num: usize = parts.next().unwrap().parse().unwrap();
    let rules = parts.next().unwrap();
    if (rules == "\"a\"") || (rules == "\"b\"") {
        (
            rule_num,
            Rule::Base(rules.trim_matches('"').chars().nth(0).unwrap()),
        )
    } else {
        (
            rule_num,
            Rule::SubRule(
                rules
                    .split(" | ")
                    .map(|p| p.split(' ').map(|sub| sub.parse().unwrap()).collect())
                    .collect(),
            ),
        )
    }
}

fn load_data() -> (HashMap<usize, Rule>, Vec<String>) {
    let f = File::open("res/input19.txt").expect("Couldn't open day 19 input");
    let reader = BufReader::new(f);
    let mut line_iter = reader.lines();

    let mut rules: HashMap<usize, Rule> = HashMap::new();
    for line in line_iter.by_ref() {
        let s = line.expect("Couldn't read line from day 19 input");
        if &s == "" {
            break;
        }
        let (rule_num, rule) = rule_from_str(&s);
        rules.insert(rule_num, rule);
    }

    let messages: Vec<String> = line_iter
        .map(|line| line.expect("Couldn't read line from day 19 input"))
        .collect();
    (rules, messages)
}

pub fn part1() -> usize {
    let mut count = 0;
    let (rules, messages) = load_data();

    for m in messages {
        let (mut res, used) = rules[&0].check(&m, &rules);
        if used != "" {
            res = false;
        }
        if res {
            count += 1;
        }
    }
    count
}

pub fn part2() -> usize {
    let mut count = 0;
    let (mut rules, messages) = load_data();

    rules.insert(
        8,
        Rule::SubRule(
            vec![vec![42], vec![42, 8]],
        ),
    );
    rules.insert(
        11,
        Rule::SubRule(
            vec![
                vec![42, 31],
                vec![42, 11, 31],
            ],
        ),
    );

    for m in messages {
        let (mut res, used) = rules[&0].check(&m, &rules);
        if used != "" {
            res = false;
        }
        if res {
            count += 1;
        }
    }
    count
}

// #[test]
// fn test_case1() {
//     let reader = std::io::Cursor::new(b"0: 1 2\n1: \"a\"\n2: 1 3 | 3 1\n3: \"b\"\n");
//     let mut line_iter = reader.lines();

//     let mut rules: HashMap<usize, Rule> = HashMap::new();
//     for line in line_iter.by_ref() {
//         let s = line.unwrap();
//         if &s == "" {
//             break;
//         }
//         let (rule_num, rule) = rule_from_str(&s);
//         rules.insert(rule_num, rule);
//     }

//     println!("{:?}", rules);
//     let message = "aab";
//     assert!(rules[&0].check(message, &rules));

//     let message = "aba";
//     assert!(rules[&0].check(message, &rules));

//     let message = "a";
//     assert!(!rules[&0].check(message, &rules));
// }

#[test]
fn test_case2() {
    let reader = std::io::Cursor::new(
        b"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb",
    );
    let mut line_iter = reader.lines();

    let mut rules: HashMap<usize, Rule> = HashMap::new();
    for line in line_iter.by_ref() {
        let s = line.unwrap();
        if &s == "" {
            break;
        }
        let (rule_num, rule) = rule_from_str(&s);
        rules.insert(rule_num, rule);
    }

    let messages: Vec<String> = line_iter
        .map(|line| line.expect("Couldn't read line from day 19 input"))
        .collect();

    // println!("{:?}", rules);
    // println!("{:?}", messages);
    // println!("{:?}", rules[&0].check(&messages[4], &rules));
    for m in messages {
        let (mut res, used) = rules[&0].check(&m, &rules);
        if used != "" {
            res = false;
        }
        println!("{}", res);
    }
}
