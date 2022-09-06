use std::collections::HashMap;
use std::fs;

fn main() {
    let p1_result = part1();
    part2(p1_result);
}

struct Planet {
    name: String,
    parent_n: String,
    children: Vec<String>,
}

impl Planet {
    fn new(name: String) -> Self {
        Self {
            name,
            parent_n: "Dummy".to_string(),
            children: Vec::new(),
        }
    }

    fn add_parent(&mut self, parent_n: String) {
        self.parent_n = parent_n;
    }

    fn add_child(&mut self, child_n: String) {
        self.children.push(child_n);
    }
}

fn load_data(fname: &str) -> HashMap<String, Planet> {
    let relations = fs::read_to_string(fname).expect("Couldn't read the file!");
    let mut planetary_system: HashMap<String, Planet> = HashMap::new();
    for r in relations.lines() {
        let mut obs = r.split(')');
        let parent_n = obs.next().unwrap();
        let child_n = obs.next().unwrap();
        {
            let parent = planetary_system
                .entry(parent_n.to_string())
                .or_insert_with(|| Planet::new(parent_n.to_string()));
            parent.add_child(child_n.to_string());
        }
        let child = planetary_system
            .entry(child_n.to_string())
            .or_insert_with(|| Planet::new(child_n.to_string()));
        child.add_parent(parent_n.to_string());
    }

    planetary_system
}

fn part1() -> HashMap<String, Planet> {
    let planetary_system = load_data("data/Day06_input.txt");
    let mut orbits: u32 = 0;
    for (_body_n, body) in planetary_system.iter() {
        let mut parent = planetary_system.get(&body.parent_n);
        loop {
            match parent {
                None => break,
                Some(t) => {
                    orbits += 1;
                    parent = planetary_system.get(&t.parent_n);
                }
            }
        }
    }
    println!("{}", orbits);
    planetary_system
}

fn part2(planetary_system: HashMap<String, Planet>) {
    let you = planetary_system.get("YOU").unwrap();
    let san = planetary_system.get("SAN").unwrap();
    let mut you_parents: Vec<&String> = Vec::new();
    let mut san_parents: Vec<&String> = Vec::new();

    let mut parent = planetary_system.get(&you.parent_n);
    loop {
        match parent {
            None => break,
            Some(t) => {
                you_parents.push(&t.name);
                parent = planetary_system.get(&t.parent_n);
            }
        }
    }

    let mut parent = planetary_system.get(&san.parent_n);
    loop {
        match parent {
            None => break,
            Some(t) => {
                san_parents.push(&t.name);
                parent = planetary_system.get(&t.parent_n);
            }
        }
    }

    let mut found = false;
    let mut j1: u32 = 0;
    let mut j2: u32 = 0;
    for you_par in you_parents.iter() {
        j1 += 1;
        j2 = 0;
        for san_par in san_parents.iter() {
            j2 += 1;
            if you_par == san_par {
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
    println!("{}", j1 + j2 - 2);
}
