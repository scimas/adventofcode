use std::{fs::File, io::Read};

use serde::Deserialize;

fn main() -> Result<(), anyhow::Error> {
    let mut fl = File::open("resources/input13")?;
    let mut input = String::new();
    fl.read_to_string(&mut input)?;
    let packets = parse_input(&input)?;
    println!("Day 13");
    println!("Part 1: {}", part1(&packets));
    println!("Part 2: {}", part2(packets));
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum Data {
    Int(i64),
    List(Vec<Data>),
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Data::Int(i1), Data::Int(i2)) => i1.cmp(i2),
            (Data::List(v1), Data::List(v2)) => {
                match v1
                    .iter()
                    .zip(v2)
                    .map(|(d1, d2)| d1.cmp(d2))
                    .fold(std::cmp::Ordering::Equal, |acc, ord| acc.then(ord))
                {
                    std::cmp::Ordering::Equal => {
                        v1.len().cmp(&v2.len())
                    }
                    ord => ord,
                }
            }
            (Data::Int(i1), l2) => Data::List(vec![Data::Int(*i1)]).cmp(l2),
            (l1, Data::Int(i2)) => l1.cmp(&Data::List(vec![Data::Int(*i2)])),
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Result<Vec<Data>, serde_json::Error> {
    let mut packets = Vec::new();
    for line in input.trim().lines() {
        if line.is_empty() {
            continue;
        }
        packets.push(serde_json::from_str(line)?);
    }
    Ok(packets)
}

fn is_correct_order(left: &Data, right: &Data) -> bool {
    left <= right
}

fn part1(packets: &[Data]) -> usize {
    assert_eq!(packets.len() % 2, 0, "must have even number of packets");
    packets
        .chunks(2)
        .map(|pair| is_correct_order(&pair[0], &pair[1]))
        .enumerate()
        .filter(|(_, correct_ord)| *correct_ord)
        .map(|(idx, _)| idx + 1)
        .sum()
}

fn part2(mut packets: Vec<Data>) -> usize {
    let decoder_packet_1 = Data::List(vec![Data::List(vec![Data::Int(2)])]);
    let decoder_packet_2 = Data::List(vec![Data::List(vec![Data::Int(6)])]);
    packets.push(decoder_packet_1.clone());
    packets.push(decoder_packet_2.clone());
    packets.sort_unstable();
    let idx1 = packets.iter().position(|p| p == &decoder_packet_1).unwrap() + 1;
    let idx2 = packets.iter().position(|p| p == &decoder_packet_2).unwrap() + 1;
    idx1 * idx2
}

#[cfg(test)]
mod tests {
    use crate::{is_correct_order, parse_input, part1, part2, Data};

    fn test_input_1() -> String {
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"
        .to_string()
    }

    #[test]
    fn parse_test() {
        let input = test_input_1();
        let expected = vec![
            Data::List(vec![
                Data::Int(1),
                Data::Int(1),
                Data::Int(3),
                Data::Int(1),
                Data::Int(1),
            ]),
            Data::List(vec![
                Data::Int(1),
                Data::Int(1),
                Data::Int(5),
                Data::Int(1),
                Data::Int(1),
            ]),
            Data::List(vec![
                Data::List(vec![Data::Int(1)]),
                Data::List(vec![Data::Int(2), Data::Int(3), Data::Int(4)]),
            ]),
            Data::List(vec![Data::List(vec![Data::Int(1)]), Data::Int(4)]),
            Data::List(vec![Data::Int(9)]),
            Data::List(vec![Data::List(vec![
                Data::Int(8),
                Data::Int(7),
                Data::Int(6),
            ])]),
            Data::List(vec![
                Data::List(vec![Data::Int(4), Data::Int(4)]),
                Data::Int(4),
                Data::Int(4),
            ]),
            Data::List(vec![
                Data::List(vec![Data::Int(4), Data::Int(4)]),
                Data::Int(4),
                Data::Int(4),
                Data::Int(4),
            ]),
            Data::List(vec![Data::Int(7), Data::Int(7), Data::Int(7), Data::Int(7)]),
            Data::List(vec![Data::Int(7), Data::Int(7), Data::Int(7)]),
            Data::List(vec![]),
            Data::List(vec![Data::Int(3)]),
            Data::List(vec![Data::List(vec![Data::List(vec![])])]),
            Data::List(vec![Data::List(vec![])]),
            Data::List(vec![
                Data::Int(1),
                Data::List(vec![
                    Data::Int(2),
                    Data::List(vec![
                        Data::Int(3),
                        Data::List(vec![
                            Data::Int(4),
                            Data::List(vec![Data::Int(5), Data::Int(6), Data::Int(7)]),
                        ]),
                    ]),
                ]),
                Data::Int(8),
                Data::Int(9),
            ]),
            Data::List(vec![
                Data::Int(1),
                Data::List(vec![
                    Data::Int(2),
                    Data::List(vec![
                        Data::Int(3),
                        Data::List(vec![
                            Data::Int(4),
                            Data::List(vec![Data::Int(5), Data::Int(6), Data::Int(0)]),
                        ]),
                    ]),
                ]),
                Data::Int(8),
                Data::Int(9),
            ]),
        ];
        match parse_input(&input) {
            Ok(packets) => assert_eq!(packets, expected),
            Err(e) => panic!("input did not parse correctly: {e}"),
        }
    }

    #[test]
    fn correct_order_test() {
        let input = test_input_1();
        let packets = parse_input(&input).unwrap();
        let expected = vec![true, true, false, true, false, true, false, false];
        for (pair, exp) in packets.chunks(2).zip(expected) {
            assert_eq!(
                is_correct_order(&pair[0], &pair[1]),
                exp,
                "problem for {pair:?}"
            );
        }
    }

    #[test]
    fn part1_test_1() {
        let input = test_input_1();
        let packets = parse_input(&input).unwrap();
        assert_eq!(part1(&packets), 13);
    }

    #[test]
    fn part2_test_1() {
        let input = test_input_1();
        let packets = parse_input(&input).unwrap();
        assert_eq!(part2(packets), 140);
    }
}
