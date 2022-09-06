use std::fs;

enum State {
    IDLE,
    RUN,
    WAIT,
    HALT,
}

struct Computer {
    ram: Vec<i32>,
    state: State,
    instruction_pointer: usize,
    inputs: Vec<i32>,
    outputs: Vec<i32>,
}

impl Computer {
    fn initialize(&mut self) {
        self.state = State::RUN;
    }

    fn load_program(&mut self, program: &Vec<i32>) {
        self.ram = Vec::new();
        self.ram.extend_from_slice(program);
        self.instruction_pointer = 0;
    }

    fn run(&mut self) {
        let mut instruction: i32 = self.ram[self.instruction_pointer];
        let opcode: i32 = instruction % 100;
        instruction /= 100;
        let in1_pos: usize = match instruction % 10 {
            0 => self.ram[self.instruction_pointer + 1] as usize,
            1 => self.instruction_pointer + 1,
            _ => {
                println!("Invalid input 1 mode!");
                0
            }
        };
        instruction /= 10;
        let in2_pos: usize = match instruction % 10 {
            0 => self.ram[self.instruction_pointer + 2] as usize,
            1 => self.instruction_pointer + 2,
            _ => {
                println!("Invalid input 2 mode!");
                0
            }
        };
        instruction /= 10;
        let out_pos: usize = match instruction % 10 {
            0 => self.ram[self.instruction_pointer + 3] as usize,
            1 => self.instruction_pointer + 3,
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
        self.ram[out_pos] = self.ram[in1_pos] + self.ram[in2_pos];
        self.instruction_pointer += 4;
    }

    fn multiply(&mut self, in1_pos: usize, in2_pos: usize, out_pos: usize) {
        self.ram[out_pos] = self.ram[in1_pos] * self.ram[in2_pos];
        self.instruction_pointer += 4;
    }

    fn input(&mut self, in_pos: usize) {
        self.ram[in_pos] = self.inputs.pop().expect("Couldn't find any input!");
        self.instruction_pointer += 2;
    }

    fn output(&mut self, in_pos: usize) {
        self.outputs.push(self.ram[in_pos]);
        self.instruction_pointer += 2;
        self.state = State::WAIT;
    }

    fn jump_if_true(&mut self, in1_pos: usize, in2_pos: usize) {
        if self.ram[in1_pos] != 0 {
            self.instruction_pointer = self.ram[in2_pos] as usize;
        }
        else {
            self.instruction_pointer += 3;
        }
    }

    fn jump_if_false(&mut self, in1_pos: usize, in2_pos: usize) {
        if self.ram[in1_pos] == 0 {
            self.instruction_pointer = self.ram[in2_pos] as usize;
        }
        else {
            self.instruction_pointer += 3;
        }
    }

    fn less_than(&mut self, in1_pos: usize, in2_pos: usize, out_pos: usize) {
        if self.ram[in1_pos] < self.ram[in2_pos] {
            self.ram[out_pos] = 1;
        }
        else {
            self.ram[out_pos] = 0;
        }
        self.instruction_pointer += 4;
    }

    fn equals(&mut self, in1_pos: usize, in2_pos: usize, out_pos: usize) {
        if self.ram[in1_pos] == self.ram[in2_pos] {
            self.ram[out_pos] = 1;
        }
        else {
            self.ram[out_pos] = 0;
        }
        self.instruction_pointer += 4;
    }
}

fn main() {
    let program_text = fs::read_to_string("../data/Day02_input.csv").expect("Couldn't read the file!");
    let program_text = program_text.trim();
    let program: Vec<i32> = program_text.split(",").map(|x| x.parse::<i32>().expect("Couldn't parse instruction!")).collect();
    
    let mut my_computer = Computer {
        ram: Vec::new(),
        state: State::IDLE,
        instruction_pointer: 0,
        inputs: Vec::new(),
        outputs: Vec::new(),
    };

    // Part 1
    my_computer.load_program(&program);
    my_computer.ram[1] = 12;
    my_computer.ram[2] = 2;
    my_computer.initialize();
    loop {
        match my_computer.state {
            State::RUN => my_computer.run(),
            State::HALT => break,
            _ => (),
        }
    }
    println!("{}", my_computer.ram[0]);

    // Part 2
    let look_for: i32 = 19690720;
    let mut found: bool = false;
    for noun in 0..99 {
        for verb in 0..99 {
            my_computer.load_program(&program);
            my_computer.ram[1] = noun;
            my_computer.ram[2] = verb;
            my_computer.initialize();
            let answer: i32 = loop {
                match my_computer.state {
                    State::RUN => my_computer.run(),
                    State::HALT => break my_computer.ram[0],
                    _ => (),
                }
            };
            if answer == look_for {
                println!("{}, {}", noun, verb);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
}
