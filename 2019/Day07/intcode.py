import re


class Computer:


    def __init__(self, start_position=0, opcodes=[]):
        self.instruction_pointer = start_position
        self.ram = dict(enumerate(opcodes))
        self.parametric_opcodes = ("01", "02", "05", "06", "07", "08")
        self.register = 0
        self.instructions = {
            "1": self.add, "2": self.multiply, "3": self.receive_input, "4": self.output, "99": self.halt,
            "5": self.jump_if_true, "6": self.jump_if_false, "7": self.less_than, "8": self.equals,
            "01": self.add, "02": self.multiply, "03": self.receive_input, "04": self.output,
            "05": self.jump_if_true, "06": self.jump_if_false, "07": self.less_than, "08": self.equals,
        }
    
    
    def load_program(self, opcodes):
        self.ram = dict(enumerate(opcodes))
        self.instruction_pointer = 0
    
    
    def view_memory(self, pos):
        return self.ram[pos]
    

    def parse_instruction(self):
        ret_value = 0
        instruction = self.ram[self.instruction_pointer]
        in1_mode = in2_mode = out_mode = "0"
        if instruction == "4":
            in1_mode = in2_mode = out_mode = "0"
            self.instructions[instruction](in1_mode, in2_mode, out_mode)
            ret_value = 4
        elif len(instruction) == 1:
            in1_mode = in2_mode = out_mode = "0"
            if instruction in self.instructions.keys():
                self.instructions[instruction](in1_mode, in2_mode, out_mode)
            else:
                ret_value = 1
        elif instruction[-2:] in self.parametric_opcodes:
            in1_mode = in2_mode = out_mode = "0"
            if len(instruction) > 2:
                in1_mode = instruction[-3]
                if len(instruction) > 3:
                    in2_mode = instruction[-4]
            self.instructions[instruction[-2:]](in1_mode, in2_mode, out_mode)
        elif instruction[-2:] == "03":
            self.receive_input(in1_mode, in2_mode, out_mode)
        elif instruction[-2:] == "04":
            out_mode = "0"
            if len(instruction) == 3:
                out_mode = instruction[-3]
            self.output(in1_mode, in2_mode, out_mode)
            ret_value = 4
        elif instruction[-2:] == "99":
            self.halt(in1_mode, in2_mode, out_mode)
            ret_value = 99
        else:
            ret_value = 1
        return ret_value


    def run(self):
        out = self.parse_instruction()
        while out == 0:
            out = self.parse_instruction()
        if out == 1:
            print("Invalid opcode!")
            return 1
        elif out == 4:
            return 4
        elif out == 99:
            # print("Execution completed!")
            return 99
    

    def output(self, in1_mode, in2_mode, out_mode):
        if out_mode == "0":
            out_pos = int(self.ram[self.instruction_pointer + 1])
        else:
            out_pos = self.instruction_pointer + 1
        self.register = self.ram[out_pos]
        # print(self.register)
        self.instruction_pointer += 2


    def add(self, in1_mode, in2_mode, out_mode):
        if in1_mode == "0":
            in1_pos = int(self.ram[self.instruction_pointer + 1])
        else:
            in1_pos = self.instruction_pointer + 1
        if in2_mode == "0":
            in2_pos = int(self.ram[self.instruction_pointer + 2])
        else:
            in2_pos = self.instruction_pointer + 2
        in1 = int(self.ram[in1_pos])
        in2 = int(self.ram[in2_pos])
        out_pos = int(self.ram[self.instruction_pointer + 3])
        self.ram[out_pos] = str(in1 + in2)
        self.instruction_pointer += 4


    def multiply(self, in1_mode, in2_mode, out_mode):
        if in1_mode == "0":
            in1_pos = int(self.ram[self.instruction_pointer + 1])
        else:
            in1_pos = self.instruction_pointer + 1
        if in2_mode == "0":
            in2_pos = int(self.ram[self.instruction_pointer + 2])
        else:
            in2_pos = self.instruction_pointer + 2
        in1 = int(self.ram[in1_pos])
        in2 = int(self.ram[in2_pos])
        out_pos = int(self.ram[self.instruction_pointer + 3])
        self.ram[out_pos] = str(in1 * in2)
        self.instruction_pointer += 4


    def receive_input(self, in1_mode, in2_mode, out_mode):
        if self.inputs:
            val = self.inputs.pop()
        else:
            val = input("Input an integer: ")
        out_pos = int(self.ram[self.instruction_pointer + 1])
        self.ram[out_pos] = val
        self.instruction_pointer += 2
    

    def jump_if_true(self, in1_mode, in2_mode, out_mode):
        if in1_mode == "0":
            in1_pos = int(self.ram[self.instruction_pointer + 1])
        else:
            in1_pos = self.instruction_pointer + 1
        if in2_mode == "0":
            in2_pos = int(self.ram[self.instruction_pointer + 2])
        else:
            in2_pos = self.instruction_pointer + 2
        in1 = int(self.ram[in1_pos])
        in2 = int(self.ram[in2_pos])
        if in1 != 0:
            self.instruction_pointer = in2
        else:
            self.instruction_pointer += 3
    

    def jump_if_false(self, in1_mode, in2_mode, out_mode):
        if in1_mode == "0":
            in1_pos = int(self.ram[self.instruction_pointer + 1])
        else:
            in1_pos = self.instruction_pointer + 1
        if in2_mode == "0":
            in2_pos = int(self.ram[self.instruction_pointer + 2])
        else:
            in2_pos = self.instruction_pointer + 2
        in1 = int(self.ram[in1_pos])
        in2 = int(self.ram[in2_pos])
        if in1 == 0:
            self.instruction_pointer = in2
        else:
            self.instruction_pointer += 3


    def less_than(self, in1_mode, in2_mode, out_mode):
        if in1_mode == "0":
            in1_pos = int(self.ram[self.instruction_pointer + 1])
        else:
            in1_pos = self.instruction_pointer + 1
        if in2_mode == "0":
            in2_pos = int(self.ram[self.instruction_pointer + 2])
        else:
            in2_pos = self.instruction_pointer + 2
        in1 = int(self.ram[in1_pos])
        in2 = int(self.ram[in2_pos])
        out_pos = int(self.ram[self.instruction_pointer + 3])
        if in1 < in2:
            self.ram[out_pos] = "1"
        else:
            self.ram[out_pos] = "0"
        self.instruction_pointer += 4


    def equals(self, in1_mode, in2_mode, out_mode):
        if in1_mode == "0":
            in1_pos = int(self.ram[self.instruction_pointer + 1])
        else:
            in1_pos = self.instruction_pointer + 1
        if in2_mode == "0":
            in2_pos = int(self.ram[self.instruction_pointer + 2])
        else:
            in2_pos = self.instruction_pointer + 2
        in1 = int(self.ram[in1_pos])
        in2 = int(self.ram[in2_pos])
        out_pos = int(self.ram[self.instruction_pointer + 3])
        if in1 == in2:
            self.ram[out_pos] = "1"
        else:
            self.ram[out_pos] = "0"
        self.instruction_pointer += 4


    def halt(self, in1_mode, in2_mode, out_mode):
        pass


def opcodes_from_string(text):
    opcodes = [m.strip() for m in text.split(",")]
    return opcodes


if __name__ == "__main__":
    opcode_str = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"
    my_computer = Computer()
    my_computer.load_program(opcodes_from_string(opcode_str))
    my_computer.run()
