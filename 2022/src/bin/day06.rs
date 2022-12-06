use std::{collections::HashSet, fs::File, io::Read};

fn main() -> Result<(), anyhow::Error> {
    let mut fl = File::open("resources/input06")?;
    let mut input = String::new();
    fl.read_to_string(&mut input)?;
    println!("Day 6");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn marker_position(buf: &str, size: usize) -> usize {
    match buf
        .as_bytes()
        .windows(size)
        .enumerate()
        .find(|(_, window)| HashSet::<_>::from_iter(window.iter()).len() == size)
    {
        Some((idx, _)) => idx + size,
        None => panic!("buffer did not have a non-repeating {size} byte sequence"),
    }
}

fn part1(buf: &str) -> usize {
    marker_position(buf, 4)
}

fn part2(buf: &str) -> usize {
    marker_position(buf, 14)
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    fn test_inputs() -> Vec<String> {
        vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(),
            "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(),
            "nppdvjthqldpwncqszvftbrmjlhg".to_string(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(),
        ]
    }

    #[test]
    fn part1_test() {
        let input = test_inputs();
        let expected = vec![7, 5, 6, 10, 11];
        for (inp, exp) in input.iter().zip(expected) {
            assert_eq!(part1(inp), exp, "failure for {inp}");
        }
    }

    #[test]
    fn part2_test() {
        let input = test_inputs();
        let expected = vec![19, 23, 23, 29, 26];
        for (inp, exp) in input.iter().zip(expected) {
            assert_eq!(part2(inp), exp, "failure for {inp}");
        }
    }
}
