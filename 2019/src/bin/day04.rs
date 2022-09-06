fn main() {
    let pass_list = part1();
    part2(&pass_list);
}

fn part1() -> Vec<u32> {
    let lower: u32 = 284639;
    let upper: u32 = 748759;
    let mut possible_pass: Vec<u32> = Vec::new();
    for i in lower..(upper + 1) {
        let mut problem = false;
        let mut pair = false;
        let mut num: u32 = i;
        for j in (1..6).rev() {
            let d1: u32 = num / 10u32.pow(j);
            let d2: u32 = (num % 10u32.pow(j)) / 10u32.pow(j - 1);
            if d2 < d1 {
                problem = true;
                break;
            }
            if d1 == d2 {
                pair = true;
            }
            num %= 10u32.pow(j);
        }
        if !problem && pair {
            possible_pass.push(i);
        }
    }
    println!("{}", possible_pass.len());
    possible_pass
}

fn part2(pass_list: &[u32]) {
    let mut possible_pass: Vec<u32> = Vec::new();
    for i in pass_list.iter() {
        let num: Vec<char> = i.to_string().chars().collect();
        let mut repeated_d: char = 'a';
        let mut length: u32 = 0;
        let mut done = false;
        for j in 0..(num.len() - 1) {
            if num[j] == repeated_d {
                length += 1;
                continue;
            }
            if length == 2 {
                possible_pass.push(*i);
                done = true;
                break;
            }
            if num[j] == num[j + 1] {
                repeated_d = num[j];
                length = 1;
            } else {
                length = 0;
                repeated_d = 'a';
            }
        }
        if !done {
            if length == 1 {
                possible_pass.push(*i);
            }
            if length == 2 && num[4] != num[5] {
                possible_pass.push(*i);
            }
        }
    }
    println!("{}", possible_pass.len());
}
