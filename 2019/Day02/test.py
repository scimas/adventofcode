import os
from intcode import Computer, opcodes_from_string

base_dir = os.getcwd()
data_dir = os.path.join(base_dir, "data")
with open(os.path.join(data_dir, "Day02_input.csv"), "r") as program_file:
    opcode_str = program_file.readline().strip()

# Part 1
my_computer = Computer()
opcodes = opcodes_from_string(opcode_str)
opcodes[1] = 12
opcodes[2] = 2
my_computer.load_program(opcodes)
if my_computer.run() == 0:
    print(my_computer.output(0))

# Part 2
found = False
for noun in range(100):
    for verb in range(100):
        opcodes = opcodes_from_string(opcode_str)
        opcodes[1] = noun
        opcodes[2] = verb
        my_computer.load_program(opcodes)
        if my_computer.run() == 0:
            if my_computer.output(0) == 19690720:
                print(noun, verb)
                found = True
                break
    if found:
        break

if not found:
    print("Couldn't find the noun - verb \U0001f622")
