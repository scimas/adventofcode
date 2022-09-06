import os

from itertools import permutations
from intcode import Computer, opcodes_from_string

base_dir = os.getcwd()
data_dir = os.path.join(base_dir, "data")
with open(os.path.join(data_dir, "Day07_input.csv"), "r") as program_file:
    opcode_str = program_file.readline().strip()
opcodes = opcodes_from_string(opcode_str)

amplifiers = [Computer() for i in range(5)]

# Part 1
phase_outs = []
for phases in permutations(range(5)):
    last_out = 0
    for amp, phase in zip(amplifiers, phases):
        amp.inputs = [last_out, phase]
        amp.load_program(opcodes)
        while amp.run() != 99:
            pass
        last_out = amp.register
    phase_outs.append([phases, last_out])

phase_outs.sort(key=lambda x: x[1], reverse=True)
print(phase_outs[0])

# Part 2
phase_outs = []
for phases in permutations(range(5, 10)):
    for amp, phase in zip(amplifiers, phases):
        amp.inputs = [phase]
        amp.load_program(opcodes)
    amplifiers[0].inputs.insert(0, 0)
    outs = [0, 0, 0, 0, 0]
    i = 0
    while outs[-1] != 99:
        outs[i] = amplifiers[i].run()
        last_out = amplifiers[i].register
        i = (i + 1) % 5
        amplifiers[i].inputs.insert(0, last_out)
    phase_outs.append([
        phases,
        amplifiers[-1].register
    ])

phase_outs.sort(key=lambda x: x[1], reverse=True)
print(phase_outs[0])
