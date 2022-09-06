use std::fs;

fn main() {
    part1();
    part2();
}

fn load_data(fname: &str) -> Vec<Vec<Vec<u32>>> {
    let image_text = fs::read_to_string(fname).expect("Couldn't read the file!");
    let image_text = image_text.trim();
    let mut image: Vec<Vec<Vec<u32>>> = Vec::new();
    let nrow = 6;
    let ncol = 25;
    let nlyr = image_text.len() / (nrow * ncol);
    let mut image_it = image_text.chars();
    for l in 0..nlyr {
        image.push(Vec::new());
        for i in 0..nrow {
            image[l].push(Vec::new());
            for _ in 0..ncol {
                image[l][i].push(image_it.next().unwrap().to_digit(10).unwrap());
            }
        }
    }
    return image;
}

fn part1() {
    let image = load_data("data/Day08_input.txt");
    let mut few_zero_layer = 0;
    let nlyr = image.len();
    let nrow = image[0].len();
    let ncol = image[0][0].len();

    let mut fewest_zeros = nrow * ncol;
    for l in 0..nlyr {
        let mut num_zeros = 0;
        for i in 0..nrow {
            for j in 0..ncol {
                if image[l][i][j] == 0 {
                    num_zeros += 1;
                }
            }
        }
        if num_zeros < fewest_zeros {
            few_zero_layer = l;
            fewest_zeros = num_zeros;
        }
    }

    let mut ones = 0;
    let mut twos = 0;
    for i in 0..nrow {
        for j in 0..ncol {
            if image[few_zero_layer][i][j] == 1 {
                ones += 1;
            } else if image[few_zero_layer][i][j] == 2 {
                twos += 1;
            }
        }
    }
    println!("{}", ones * twos);
}

fn part2() {
    let image = load_data("data/Day08_input.txt");
    let mut filtered_image: Vec<Vec<u32>> = Vec::new();
    let nlyr = image.len();
    let nrow = image[0].len();
    let ncol = image[0][0].len();

    for i in 0..nrow {
        filtered_image.push(Vec::new());
        for j in 0..ncol {
            for l in 0..nlyr {
                if image[l][i][j] != 2 {
                    filtered_image[i].push(image[l][i][j]);
                    break;
                }
            }
        }
    }
    for i in 0..nrow {
        println!("{:?}", filtered_image[i]);
    }
}
