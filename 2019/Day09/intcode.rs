use std::fs;
use std::collections::HashMap;

enum State {
    IDLE,
    RUN,
    WAIT,
    HALT,
}

struct Computer {
    ram: HashMap<usize, i64>,
    state: State,
    instruction_pointer: usize,
    relative_base: usize,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

impl Computer {
    fn initialize(&mut self) {
        self.state = State::RUN;
    }

    fn load_program(&mut self, program: &HashMap<usize, i64>) {
        self.ram = program.iter().map(|(k, v)| (*k, *v)).collect();
        self.instruction_pointer = 0;
        self.relative_base = 0;
    }

    fn run(&mut self) {
        let mut instruction: i64 = *self.ram.entry(self.instruction_pointer).or_insert(0);
        let opcode: i64 = instruction % 100;
        instruction /= 100;
        let in1_pos: usize = match instruction % 10 {
            0 => *self.ram.entry(self.instruction_pointer + 1).or_insert(0) as usize,
            1 => self.instruction_pointer + 1,
            2 => (*self.ram.entry(self.instruction_pointer + 1).or_insert(0) + self.relative_base as i64) as usize,
            _ => {
                println!("Invalid input 1 mode!");
                0
            }
        };
        instruction /= 10;
        let in2_pos: usize = match instruction % 10 {
            0 => *self.ram.entry(self.instruction_pointer + 2).or_insert(0) as usize,
            1 => self.instruction_pointer + 2,
            2 => (*self.ram.entry(self.instruction_pointer + 2).or_insert(0) + self.relative_base as i64) as usize,
            _ => {
                println!("Invalid input 2 mode!");
                0
            }
        };
        instruction /= 10;
        let out_pos: usize = match instruction % 10 {
            0 => *self.ram.entry(self.instruction_pointer + 3).or_insert(0) as usize,
            1 => self.instruction_pointer + 3,
            2 => (*self.ram.entry(self.instruction_pointer + 3).or_insert(0) + self.relative_base as i64) as usize,
            _ => {
                println!("Invalid output mode!");
                0
            }
        };
        match opcode {
            99 => self.halt(),
            1 => self.add(in1_pos, in2_pos, out_pos),
            2 => self.multiply(in1_pos, in2_pos, out_pos),
            3 => self.input(in1_pos),
            4 => self.output(in1_pos),
            5 => self.jump_if_true(in1_pos, in2_pos),
            6 => self.jump_if_false(in1_pos, in2_pos),
            7 => self.less_than(in1_pos, in2_pos, out_pos),
            8 => self.equals(in1_pos, in2_pos, out_pos),
            9 => self.offset_relative_base(in1_pos),
            _ => {
                println!("Invalid opcode!");
            }
        }
    }

    fn halt(&mut self) {
        self.instruction_pointer += 1;
        self.state = State::HALT;
    }

    fn add(&mut self, in1_pos: usize, in2_pos: usize, out_pos: usize) {
        self.ram.entry(in1_pos).or_insert(0);
        self.ram.entry(in2_pos).or_insert(0);
        self.ram.insert(out_pos, *self.ram.get(&in1_pos).expect("") + *self.ram.get(&in2_pos).expect(""));
        self.instruction_pointer += 4;
    }

    fn multiply(&mut self, in1_pos: usize, in2_pos: usize, out_pos: usize) {
        self.ram.entry(in1_pos).or_insert(0);
        self.ram.entry(in2_pos).or_insert(0);
        self.ram.insert(out_pos, *self.ram.get(&in1_pos).expect("") * *self.ram.get(&in2_pos).expect(""));
        self.instruction_pointer += 4;
    }

    fn input(&mut self, in_pos: usize) {
        self.ram.insert(in_pos, self.inputs.pop().expect("Couldn't find any input!"));
        self.instruction_pointer += 2;
    }

    fn output(&mut self, in_pos: usize) {
        self.outputs.push(*self.ram.entry(in_pos).or_insert(0));
        self.instruction_pointer += 2;
        self.state = State::WAIT;
    }

    fn jump_if_true(&mut self, in1_pos: usize, in2_pos: usize) {
        if *self.ram.entry(in1_pos).or_insert(0) != 0 {
            self.instruction_pointer = *self.ram.entry(in2_pos).or_insert(0) as usize;
        }
        else {
            self.instruction_pointer += 3;
        }
    }

    fn jump_if_false(&mut self, in1_pos: usize, in2_pos: usize) {
        if *self.ram.entry(in1_pos).or_insert(0) == 0 {
            self.instruction_pointer = *self.ram.entry(in2_pos).or_insert(0) as usize;
        }
        else {
            self.instruction_pointer += 3;
        }
    }

    fn less_than(&mut self, in1_pos: usize, in2_pos: usize, out_pos: usize) {
        if *self.ram.entry(in1_pos).or_insert(0) < *self.ram.entry(in2_pos).or_insert(0) {
            self.ram.insert(out_pos, 1);
        }
        else {
            self.ram.insert(out_pos, 0);
        }
        self.instruction_pointer += 4;
    }

    fn equals(&mut self, in1_pos: usize, in2_pos: usize, out_pos: usize) {
        if *self.ram.entry(in1_pos).or_insert(0) == *self.ram.entry(in2_pos).or_insert(0) {
            self.ram.insert(out_pos, 1);
        }
        else {
            self.ram.insert(out_pos, 0);
        }
        self.instruction_pointer += 4;
    }

    fn offset_relative_base(&mut self, in_pos: usize) {
        self.relative_base = (*self.ram.entry(in_pos).or_insert(0) + self.relative_base as i64) as usize;
        self.instruction_pointer += 2;
    }
}

fn main() {
    let program_text = fs::read_to_string("../data/Day09_input.csv").expect("Couldn't read the file!");
    // let program_text = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let program_text = program_text.trim();
    let program_vec: Vec<i64> = program_text.split(",").map(|x| x.parse::<i64>().expect("Couldn't parse instruction!")).collect();
    let program: HashMap<usize, i64> = (0..program_vec.len()).zip(program_vec.iter()).map(|(k, v)| (k, *v)).collect();
    
    let mut my_computer = Computer {
        ram: HashMap::new(),
        state: State::IDLE,
        instruction_pointer: 0,
        relative_base: 0,
        inputs: Vec::new(),
        outputs: Vec::new(),
    };

    // Part 1
    my_computer.load_program(&program);
    my_computer.inputs.push(1);
    my_computer.initialize();
    loop {
        match my_computer.state {
            State::HALT => break,
            _ => my_computer.run(),
        }
    }
    println!("{}", my_computer.outputs.pop().expect(""));

    // Part 2
    my_computer.load_program(&program);
    my_computer.inputs.push(2);
    my_computer.initialize();
    loop {
        match my_computer.state {
            State::HALT => break,
            _ => my_computer.run(),
        }
    }
    println!("{}", my_computer.outputs.pop().expect(""));
}
