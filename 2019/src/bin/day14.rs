use std::collections::HashMap;
use std::fs;

fn main() {
    let p1_result = part1();
    part2(p1_result);
}

fn parse_line(line: &str) -> ((String, i64), Vec<(String, i64)>) {
    let sides: Vec<&str> = line.split(" => ").collect();
    let output = parse_reactant(sides[1]);
    let mut inputs: Vec<(String, i64)> = Vec::new();
    for reactant in sides[0].split(", ") {
        inputs.push(parse_reactant(reactant));
    }
    (output, inputs)
}

fn parse_reactant(reactant: &str) -> (String, i64) {
    let mut iter = reactant.rsplit(" ");
    (
        iter.next().unwrap().to_string(),
        iter.next()
            .unwrap()
            .parse::<i64>()
            .expect("Couldn't parse reactant amount"),
    )
}

fn load_data(fname: &str) -> String {
    fs::read_to_string(fname).expect("Couldn't read the file.")
}

fn reactions_from_text(text: &str) -> HashMap<String, ((String, i64), Vec<(String, i64)>)> {
    let text = text.trim();
    let mut reactions: HashMap<String, ((String, i64), Vec<(String, i64)>)> = HashMap::new();

    for line in text.lines() {
        let reaction = parse_line(line);
        reactions.insert((reaction.0).0.clone(), (reaction.0, reaction.1));
    }
    reactions
}

fn make(
    product: &str,
    amount: i64,
    reactions: &HashMap<String, ((String, i64), Vec<(String, i64)>)>,
    extra: &mut HashMap<String, i64>,
) -> i64 {
    let reaction = reactions.get(product).unwrap();
    let min_amount = (reaction.0).1;
    let multiplier = amount / min_amount + if amount % min_amount != 0 { 1 } else { 0 };
    let mut ore = 0;
    for input in reaction.1.iter() {
        let required = input.1 * multiplier;
        if input.0 == "ORE" {
            ore += required;
        } else {
            extra.entry(input.0.to_string()).or_insert(0);
            let &have_extra = extra.get(&input.0).unwrap();
            if have_extra < required {
                ore += make(&input.0, required - have_extra, reactions, extra);
            }
            let &have_extra = extra.get(&input.0).unwrap();
            extra.insert(input.0.to_string(), have_extra - required);
        }
    }
    let product_extra = extra.entry(product.to_string()).or_insert(0);
    *product_extra += min_amount * multiplier;
    ore
}

fn part1() -> i64 {
    let reaction_text = load_data("data/Day14_input.txt");
    // let reaction_text = "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";
    let reactions = reactions_from_text(&reaction_text);
    let mut extra: HashMap<String, i64> = HashMap::new();
    let ore = make("FUEL", 1, &reactions, &mut extra);
    println!("{}", ore);
    ore
}

fn part2(min_ore: i64) {
    let reaction_text = load_data("data/Day14_input.txt");
    // let reaction_text = "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";
    let reactions = reactions_from_text(&reaction_text);
    const AVAILABLE_ORE: i64 = 1_000_000_000_000;
    let mut make_fuel = AVAILABLE_ORE / min_ore;
    loop {
        let mut extra: HashMap<String, i64> = HashMap::new();
        let ore = make("FUEL", make_fuel + 1, &reactions, &mut extra);
        if AVAILABLE_ORE > ore {
            make_fuel = (make_fuel + 1).max(((make_fuel + 1) * AVAILABLE_ORE) / ore);
        } else {
            break;
        }
    }
    println!("{}", make_fuel);
}
