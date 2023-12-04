use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("res/input01")?;
    let depths = parse(BufReader::new(file))?;

    println!("Part 1: {}", count_depth_increases(&depths));
    Ok(())
}

fn parse<T: std::io::BufRead>(input: T) -> std::io::Result<Vec<u64>> {
    let mut depths = Vec::new();
    for maybe_line in input.lines() {
        let depth = maybe_line?.parse().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "unable to parse as u64")
        })?;
        depths.push(depth);
    }
    Ok(depths)
}

fn count_depth_increases(depths: &[u64]) -> usize {
    let mut count = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::{count_depth_increases, parse};

    const TEST_INPUT_1: &str = "\
199
200
208
210
200
207
240
269
260
263
";

    #[test]
    fn test_depth_increase_count() {
        let depths = parse(TEST_INPUT_1.as_bytes()).expect("test input 1 did not parse correctly");
        assert_eq!(count_depth_increases(&depths), 7);
    }
}
