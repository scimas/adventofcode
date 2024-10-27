use std::collections::HashMap;
use std::fs::read_to_string;

use tracing::{instrument, trace};

fn main() {
    let input = read_to_string("res/input19.txt").expect("Couldn't open day 19 input");
    let (rules, messages) = parse_input(&input);
    println!("Part 1: {}", part1(&rules, &messages));
    println!("Part 2: {}", part2(rules, &messages));
}

#[derive(Debug)]
enum Rule {
    Base(char),
    Id(usize),
    And(Vec<Rule>),
    Or(Vec<Rule>),
}

impl Rule {
    #[instrument(skip(rules), parent = None)]
    fn check<'a>(&self, s: &'a str, rules: &HashMap<usize, Rule>) -> Result<&'a str, &'a str> {
        trace!("");
        match self {
            Rule::Base(c) => {
                if let Some(rem) = s.strip_prefix(*c) {
                    trace!("matched with remaining {}", rem);
                    Ok(rem)
                } else {
                    trace!("did not match");
                    Err(s)
                }
            }
            Rule::Id(n) => rules[n].check(s, rules),
            Rule::And(v) => {
                let mut remain = s;
                for rule in v {
                    match rule.check(remain, rules) {
                        Ok(res_remain) => {
                            remain = res_remain;
                        }
                        Err(res_remain) => {
                            trace!("did not match with remaining {}", res_remain);
                            return Err(res_remain);
                        }
                    }
                }
                trace!("matched with remaining {}", remain);
                Ok(remain)
            }
            Rule::Or(v) => {
                for rule in v {
                    match rule.check(s, rules) {
                        Ok(res_remain) => {
                            trace!("matched with remaining {}", res_remain);
                            return Ok(res_remain);
                        }
                        Err(_) => (),
                    }
                }
                trace!("did not match");
                Err(s)
            }
        }
    }
}

fn rule_from_str(s: &str) -> (usize, Rule) {
    let (rule_num_s, rule_s) = s.split_once(": ").unwrap();
    let rule_num: usize = rule_num_s.parse().unwrap();
    match rule_s {
        "\"a\"" => (rule_num, Rule::Base('a')),
        "\"b\"" => (rule_num, Rule::Base('b')),
        rule_s => {
            if let Ok(n) = rule_s.parse() {
                return (rule_num, Rule::Id(n));
            }
            let mut or_parts = vec![];
            for or_part in rule_s.split(" | ") {
                let mut and_parts = vec![];
                for and_part in or_part.split(" ") {
                    and_parts.push(Rule::Id(and_part.parse().unwrap()));
                }
                or_parts.push(Rule::And(and_parts));
            }
            (rule_num, Rule::Or(or_parts))
        }
    }
}

fn parse_input(input: &str) -> (HashMap<usize, Rule>, Vec<String>) {
    let mut line_iter = input.lines();

    let mut rules: HashMap<usize, Rule> = HashMap::new();
    for line in line_iter.by_ref() {
        if line == "" {
            break;
        }
        let (rule_num, rule) = rule_from_str(line);
        rules.insert(rule_num, rule);
    }

    let messages: Vec<String> = line_iter.map(|l| l.to_string()).collect();
    (rules, messages)
}

fn part1(rules: &HashMap<usize, Rule>, messages: &[String]) -> usize {
    messages
        .iter()
        .map(|m| rules[&0].check(m, rules))
        .filter(|res| res.is_ok_and(|remain| remain.is_empty()))
        .count()
}

fn part2(mut rules: HashMap<usize, Rule>, messages: &[String]) -> usize {
    rules.insert(8, rule_from_str("8: 42 | 42 8").1);
    rules.insert(11, rule_from_str("11: 42 31 | 42 11 31").1);

    messages
        .iter()
        .map(|m| rules[&0].check(m, &rules))
        .filter(|res| res.is_ok_and(|remain| remain.is_empty()))
        .count()
}

#[cfg(test)]
mod tests {
    use tracing::Level;

    use crate::{parse_input, part1, part2, rule_from_str};

    #[test]
    fn test_case1() {
        let test_input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        let (rules, messages) = parse_input(test_input);
        let expected = vec![Ok(""), Err("bababa"), Ok(""), Err("aaabbb"), Ok("b")];

        for (idx, ex) in expected.into_iter().enumerate() {
            assert_eq!(
                ex,
                rules[&0].check(&messages[idx], &rules),
                "failed for {}",
                &messages[idx]
            );
        }
    }

    #[test]
    fn test_case2() {
        let test_input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        let (rules, messages) = parse_input(test_input);
        assert_eq!(3, part1(&rules, &messages));
    }

    #[test]
    fn test_case3() {
        tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .init();
        let test_input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        let (mut rules, messages) = parse_input(test_input);
        rules.insert(8, rule_from_str("8: 42 | 42 8").1);
        rules.insert(11, rule_from_str("11: 42 31 | 42 11 31").1);

        // let expected = [
        //     false, true, true, true, true, true, true, true, true, true, true, false, true, false,
        //     true,
        // ];
        // for (idx, ex) in expected.iter().enumerate() {
        //     let res = rules[&0].check(&messages[idx], &rules);
        //     if *ex {
        //         assert!(
        //             res.is_ok_and(|remain| remain.is_empty()),
        //             "failed for {}, remaining {res:?}",
        //             &messages[idx]
        //         );
        //     } else {
        //         assert!(
        //             res.is_err(),
        //             "failed for {}, remaining {res:?}",
        //             &messages[idx]
        //         )
        //     }
        // }
        let res = rules[&0].check(&messages[2], &rules);
        assert!(res.is_ok());
    }

    #[test]
    fn test_case4() {
        let test_input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        let (rules, messages) = parse_input(test_input);
        assert_eq!(12, part2(rules, &messages));
    }
}
