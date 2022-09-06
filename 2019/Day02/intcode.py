import re


class Computer:


    def __init__(self, start_position=0, opcodes=[]):
        self.program_counter = start_position
        self.ram = dict(enumerate(opcodes))
    
    
    def load_program(self, opcodes):
        self.ram = dict(enumerate(opcodes))
        self.program_counter = 0
    

    def run(self):
        valid_opcodes = (1, 2, 99)
        opcode = self.ram[self.program_counter]
        while opcode in valid_opcodes:
            if opcode == 99:
                self.halt()
                break
            elif opcode == 1:
                self.add()
            elif opcode == 2:
                self.multiply()
            else:
                print("Invalid opcode")
                return 1
            opcode = self.ram[self.program_counter]
        return 0
    

    def output(self, counter):
        return self.ram[counter]
    

    def add(self):
        in1_pos = self.ram[self.program_counter + 1]
        in2_pos = self.ram[self.program_counter + 2]
        out_pos = self.ram[self.program_counter + 3]
        self.ram[out_pos] = self.ram[in1_pos] + self.ram[in2_pos]
        self.program_counter += 4
    

    def multiply(self):
        in1_pos = self.ram[self.program_counter + 1]
        in2_pos = self.ram[self.program_counter + 2]
        out_pos = self.ram[self.program_counter + 3]
        self.ram[out_pos] = self.ram[in1_pos] * self.ram[in2_pos]
        self.program_counter += 4


    def halt(self):
        self.program_counter += 1


def opcodes_from_string(text):
    num_patters = re.compile(r"[0-9]+")
    opcodes = [int(m) for m in num_patters.findall(text)]
    return opcodes


if __name__ == "__main__":
    opcode_str = "1,9,10,3,2,3,11,0,99,30,40,50"
    my_computer = Computer()
    my_computer.load_program(opcodes_from_string(opcode_str))
    if my_computer.run() == 0:
        print(my_computer.output(0))
