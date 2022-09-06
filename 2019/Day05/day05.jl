include("../intcode.jl")
using .Intcode

function text_to_instructions(text::String)
    ins = split(strip(text), ",")
    return Dict([(k-1, parse(Int, v)) for (k, v) in enumerate(ins)])
end

path = "data/Day05_input.txt"
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
load_program(my_computer, deepcopy(instructions))
add_input(my_computer, 1)
initialize(my_computer)
while get_state(my_computer) != Intcode.HALT
    execute(my_computer)
end

println(get_output(my_computer))

# Part 2
load_program(my_computer, deepcopy(instructions))
add_input(my_computer, 5)
initialize(my_computer)
while get_state(my_computer) != Intcode.HALT
    execute(my_computer)
end

println(get_output(my_computer))