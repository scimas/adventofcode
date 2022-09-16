use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug, Clone, Copy)]
enum Stage {
    FindOpenBracket,
    FindCharCount,
    FindMultiplier,
    CollectMultiChars,
}

fn main() {
    let fl = File::open("res/day09/input").expect("couldn't open the file");
    let mut reader = BufReader::new(fl);
    let mut data = String::new();
    reader
        .read_to_string(&mut data)
        .expect("couldn't read the file");
    data = data.trim().to_string();
    println!("Day 09");
    let decompressed_str = part1(&data);
    println!("Part 1: {}", decompressed_str.len());
    println!("Part 2: {}", part2(&data));
}

fn part1(data: &str) -> String {
    let mut decompressed = Vec::with_capacity(data.len());
    let mut stage = Stage::FindOpenBracket;
    let mut char_count_str = String::new();
    let mut char_count = 0;
    let mut multiplier_str = String::new();
    let mut multiplier = 0;
    let mut multi_chars = vec![];
    for ch in data.chars() {
        match stage {
            Stage::FindOpenBracket => {
                if ch == '(' {
                    stage = Stage::FindCharCount;
                } else {
                    decompressed.push(ch);
                }
            }
            Stage::FindCharCount => {
                if ch == 'x' {
                    char_count = char_count_str
                        .parse()
                        .expect("couldn't parse character count");
                    char_count_str.clear();
                    stage = Stage::FindMultiplier;
                } else {
                    char_count_str.push(ch);
                }
            }
            Stage::FindMultiplier => {
                if ch == ')' {
                    multiplier = multiplier_str.parse().expect("couldn't parse multiplier");
                    multiplier_str.clear();
                    stage = Stage::CollectMultiChars;
                } else {
                    multiplier_str.push(ch);
                }
            }
            Stage::CollectMultiChars => {
                multi_chars.push(ch);
                char_count -= 1;
                if char_count == 0 {
                    for _ in 0..multiplier {
                        decompressed.extend(multi_chars.iter());
                    }
                    multi_chars.clear();
                    stage = Stage::FindOpenBracket;
                }
            }
        }
    }
    decompressed.iter().collect()
}

fn decompress_length(data: &[char]) -> usize {
    let mut length = 0;
    let mut stage = Stage::FindOpenBracket;
    let mut char_count_str = String::new();
    let mut char_count = 0;
    let mut multiplier_str = String::new();
    let mut multiplier: usize;
    let mut i = 0;
    while i < data.len() {
        match stage {
            Stage::FindOpenBracket => {
                if data[i] == '(' {
                    stage = Stage::FindCharCount;
                } else {
                    length += 1;
                }
            }
            Stage::FindCharCount => {
                if data[i] == 'x' {
                    char_count = char_count_str
                        .parse()
                        .expect("couldn't parse character count");
                    char_count_str.clear();
                    stage = Stage::FindMultiplier;
                } else {
                    char_count_str.push(data[i]);
                }
            }
            Stage::FindMultiplier => {
                if data[i] == ')' {
                    multiplier = multiplier_str.parse().expect("couldn't parse multiplier");
                    multiplier_str.clear();
                    length += multiplier * decompress_length(&data[(i + 1)..(i + 1 + char_count)]);
                    i += char_count;
                    stage = Stage::FindOpenBracket;
                } else {
                    multiplier_str.push(data[i]);
                }
            }
            Stage::CollectMultiChars => unreachable!(),
        }
        i += 1;
    }
    length
}

fn part2(data: &str) -> usize {
    let chars: Vec<char> = data.chars().collect();
    decompress_length(&chars)
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_case_1() {
        let data = "ADVENT";
        assert_eq!(data, part1(data));
        assert_eq!(6, part2(data));
    }

    #[test]
    fn test_case_2() {
        let data = "A(1x5)BC";
        let expected = "ABBBBBC";
        assert_eq!(expected, part1(data));
        assert_eq!(7, part2(data));
    }

    #[test]
    fn test_case_3() {
        let data = "(3x3)XYZ";
        assert_eq!(9, part2(data));
    }

    #[test]
    fn test_case_4() {
        let data = "X(8x2)(3x3)ABCY";
        assert_eq!(20, part2(data));
    }

    #[test]
    fn test_case_5() {
        let data = "(27x12)(20x12)(13x14)(7x10)(1x12)A";
        assert_eq!(241920, part2(data));
    }
}
