use std::{collections::HashMap, fs::File, io::Read, str::FromStr};

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy)]
struct ParseColorError;

impl FromStr for Color {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(ParseColorError),
        }
    }
}

fn main() {
    let mut input = String::new();
    File::open("res/day02")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let games = parse_input(&input);
    println!("Day 02");
    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));
}

fn parse_input(input: &str) -> Vec<Vec<HashMap<Color, u8>>> {
    let pattern = Regex::new(
        r"([0-9]+) (red|green|blue)(, ([0-9]+) (red|green|blue))?(, ([0-9]+) (red|green|blue))?;?",
    )
    .unwrap();
    let mut games = Vec::new();
    for line in input.lines() {
        let mut sets = Vec::new();
        for cap in pattern.captures_iter(line) {
            let mut set: HashMap<Color, u8> = HashMap::new();
            match (cap.get(1), cap.get(2)) {
                (Some(count), Some(color)) => {
                    set.insert(
                        color.as_str().parse().unwrap(),
                        count.as_str().parse().unwrap(),
                    );
                }
                (None, _) => (),
                (Some(_), None) => unreachable!(
                    "regex pattern makes it impossible for count to exist without the color name"
                ),
            }
            match (cap.get(4), cap.get(5)) {
                (Some(count1), Some(color1)) => {
                    set.insert(
                        color1.as_str().parse().unwrap(),
                        count1.as_str().parse().unwrap(),
                    );
                }
                (None, _) => (),
                (Some(_), None) => unreachable!(
                    "regex pattern makes it impossible for count to exist without the color name"
                ),
            }
            match (cap.get(7), cap.get(8)) {
                (Some(count1), Some(color1)) => {
                    set.insert(
                        color1.as_str().parse().unwrap(),
                        count1.as_str().parse().unwrap(),
                    );
                }
                (None, _) => (),
                (Some(_), None) => unreachable!(
                    "regex pattern makes it impossible for count to exist without the color name"
                ),
            }
            sets.push(set);
        }
        games.push(sets);
    }
    games
}

fn game_required_cubes(sets: &[HashMap<Color, u8>]) -> HashMap<Color, u8> {
    let mut required = HashMap::from([(Color::Red, 0u8), (Color::Green, 0), (Color::Blue, 0)]);
    for set in sets {
        for (color, count) in set {
            required
                .entry(*color)
                .and_modify(|current_count| *current_count = (*current_count).max(*count));
        }
    }
    required
}

fn part1(games: &[Vec<HashMap<Color, u8>>]) -> u32 {
    let available_cubes =
        HashMap::from([(Color::Red, 12u8), (Color::Green, 13), (Color::Blue, 14)]);
    let mut sum = 0;
    for (idx, game) in games.into_iter().enumerate() {
        let required = game_required_cubes(game);
        if required
            .iter()
            .all(|(color, count)| count <= &available_cubes[color])
        {
            sum += idx as u32 + 1;
        }
    }
    sum
}

fn part2(games: &[Vec<HashMap<Color, u8>>]) -> u32 {
    games
        .into_iter()
        .map(|game| game_required_cubes(game))
        .map(|required| required.into_values().map(u32::from).product::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{parse_input, part1, part2, Color};

    fn test_input01() -> &'static str {
        include_str!("test_input01")
    }

    #[test]
    fn parsing_works() {
        let input = test_input01();
        let expected = vec![
            vec![
                HashMap::from([(Color::Blue, 3), (Color::Red, 4)]),
                HashMap::from([(Color::Red, 1), (Color::Green, 2), (Color::Blue, 6)]),
                HashMap::from([(Color::Green, 2)]),
            ],
            vec![
                HashMap::from([(Color::Green, 2), (Color::Blue, 1)]),
                HashMap::from([(Color::Red, 1), (Color::Green, 3), (Color::Blue, 4)]),
                HashMap::from([(Color::Green, 1), (Color::Blue, 1)]),
            ],
            vec![
                HashMap::from([(Color::Red, 20), (Color::Green, 8), (Color::Blue, 6)]),
                HashMap::from([(Color::Red, 4), (Color::Green, 13), (Color::Blue, 5)]),
                HashMap::from([(Color::Red, 1), (Color::Green, 5)]),
            ],
            vec![
                HashMap::from([(Color::Red, 3), (Color::Green, 1), (Color::Blue, 6)]),
                HashMap::from([(Color::Red, 6), (Color::Green, 3)]),
                HashMap::from([(Color::Red, 14), (Color::Green, 3), (Color::Blue, 15)]),
            ],
            vec![
                HashMap::from([(Color::Red, 6), (Color::Green, 3), (Color::Blue, 1)]),
                HashMap::from([(Color::Red, 1), (Color::Green, 2), (Color::Blue, 2)]),
            ],
        ];
        let actual = parse_input(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_sums_possible_game_ids() {
        let input = test_input01();
        let games = parse_input(input);
        let expected = 8;
        let actual = part1(&games);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_sums_powers_of_games() {
        let input = test_input01();
        let games = parse_input(input);
        let expected = 2286;
        let actual = part2(&games);
        assert_eq!(expected, actual);
    }
}
