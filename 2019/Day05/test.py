import os
from intcode import Computer, opcodes_from_string

base_dir = os.getcwd()
data_dir = os.path.join(base_dir, "data")
with open(os.path.join(data_dir, "Day05_input.csv"), "r") as program_file:
    opcode_str = program_file.readline().strip()

my_computer = Computer()
opcodes = opcodes_from_string(opcode_str)
my_computer.load_program(opcodes)
my_computer.run()
