include("../intcode.jl")
using .Intcode

function text_to_instructions(text::String)
    ins = split(strip(text), ",")
    return Dict([(k-1, parse(Int, v)) for (k, v) in enumerate(ins)])
end

path = "data/Day02_input.txt"
if !ispath(path)
    path = joinpath("..", path)
    if !ispath(path)
        exit(1)
    end
end

text = open(path, "r") do infile
    return readlines(infile)[1]
end

instructions = text_to_instructions(text)

my_computer = make_computer()

# Part 1
p1_instructions = deepcopy(instructions)
p1_instructions[1] = 12
p1_instructions[2] = 2
load_program(my_computer, p1_instructions)
initialize(my_computer)
while get_state(my_computer) != Intcode.HALT
    execute(my_computer)
end

println(read_memory(my_computer, 0))

# Part 2
found = false
for noun in 0:99
    for verb in 0:99
        p2_instructions = deepcopy(instructions)
        p2_instructions[1] = noun
        p2_instructions[2] = verb
        load_program(my_computer, p2_instructions)
        initialize(my_computer)
        while get_state(my_computer) != Intcode.HALT
            execute(my_computer)
        end
        if read_memory(my_computer, 0) == 19690720
            println(noun, verb)
            found = true
            break
        end
    end
    if found
        break
    end
end
