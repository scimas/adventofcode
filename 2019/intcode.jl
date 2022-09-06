module Intcode

export State, make_computer
export initialize, load_program, add_input, execute, get_output
export get_state, read_memory, get_instruction_pointer

@enum State IDLE RUN WAIT HALT

mutable struct Computer
    ram::Dict{Int, Int}
    state::State
    instruction_pointer::Int
    relative_base::Int
    inputs::Array{Int, 1}
    outputs::Array{Int, 1}
end

function make_computer()
    Computer(Dict(), IDLE, 0, 0, Int[], Int[])
end

function initialize(computer::Computer)
    computer.state = RUN
end

function load_program(computer::Computer, program::Dict{Int, Int})
    computer.ram = program
    computer.instruction_pointer = 0
    computer.relative_base = 0
    computer.inputs = Int[]
    computer.outputs = Int[]
end

function add_input(computer::Computer, in_val::Int)
    push!(computer.inputs, in_val)
end

function get_instruction_pointer(computer::Computer)::Int
    computer.instruction_pointer
end

function get_state(computer::Computer)::State
    computer.state
end

function get_output(computer::Computer)::Int
    pop!(computer.outputs)
end

function read_memory(computer::Computer, address::Int)::Int
    computer.ram[address]
end

function execute(computer::Computer)
    instruction = get!(computer.ram, computer.instruction_pointer, 0)
    opcode = instruction % 100
    instruction ÷= 100

    in1_mode = instruction % 10
    in1_pos = Int(0)
    if in1_mode == 0
        in1_pos = get!(computer.ram, computer.instruction_pointer + 1, 0)
    elseif in1_mode == 1
        in1_pos = computer.instruction_pointer + 1
    elseif in1_mode == 2
        in1_pos = get!(computer.ram, computer.instruction_pointer + 1, 0) + computer.relative_base
    else
        println("Invalid input 1 mode!")
    end
    instruction ÷= 10

    in2_mode = instruction % 10
    in2_pos = 0
    if in2_mode == 0
        in2_pos = get!(computer.ram, computer.instruction_pointer + 2, 0)
    elseif in2_mode == 1
        in2_pos = computer.instruction_pointer + 2
    elseif in2_mode == 2
        in2_pos = get!(computer.ram, computer.instruction_pointer + 2, 0) + computer.relative_base
    else
        println("Invalid input 2 mode!")
    end
    instruction ÷= 10

    out_mode = instruction % 10
    out_pos = 0
    if out_mode == 0
        out_pos = get!(computer.ram, computer.instruction_pointer + 3, 0)
    elseif out_mode == 1
        out_pos = computer.instruction_pointer + 3
    elseif out_mode == 2
        out_pos = get!(computer.ram, computer.instruction_pointer + 3, 0) + computer.relative_base
    else
        println("Invalid output mode!")
    end

    if opcode == 99
        halt(computer)
    elseif opcode == 1
        add(computer, in1_pos, in2_pos, out_pos)
    elseif opcode == 2
        multiply(computer, in1_pos, in2_pos, out_pos)
    elseif opcode == 3
        inp(computer, in1_pos)
    elseif opcode == 4
        output(computer, in1_pos)
    elseif opcode == 5
        jump_if_true(computer, in1_pos, in2_pos)
    elseif opcode == 6
        jump_if_false(computer, in1_pos, in2_pos)
    elseif opcode == 7
        less_than(computer, in1_pos, in2_pos, out_pos)
    elseif opcode == 8
        equals(computer, in1_pos, in2_pos, out_pos)
    elseif opcode == 9
        offset_relative_base(computer, in1_pos)
    else
        println("Invalid opcode!")
    end
end

function halt(computer::Computer)
    computer.instruction_pointer += 1
    computer.state = HALT
end

function add(computer::Computer, in1_pos::Int, in2_pos::Int, out_pos::Int)
    in1 = get!(computer.ram, in1_pos, 0)
    in2 = get!(computer.ram, in2_pos, 0)
    computer.ram[out_pos] = in1 + in2
    computer.instruction_pointer += 4
end

function multiply(computer::Computer, in1_pos::Int, in2_pos::Int, out_pos::Int)
    in1 = get!(computer.ram, in1_pos, 0)
    in2 = get!(computer.ram, in2_pos, 0)
    computer.ram[out_pos] = in1 * in2
    computer.instruction_pointer += 4
end

function inp(computer::Computer, in_pos::Int)
    computer.ram[in_pos] = popfirst!(computer.inputs)
    computer.instruction_pointer += 2
end

function output(computer::Computer, in_pos::Int)
    push!(computer.outputs, get!(computer.ram, in_pos, 0))
    computer.instruction_pointer += 2
    computer.state = WAIT
end

function jump_if_true(computer::Computer, in1_pos::Int, in2_pos::Int)
    if get!(computer.ram, in1_pos, 0) != 0
        computer.instruction_pointer = get!(computer.ram, in2_pos, 0)
    else
        computer.instruction_pointer += 3
    end
end

function jump_if_false(computer::Computer, in1_pos::Int, in2_pos::Int)
    if get!(computer.ram, in1_pos, 0) == 0
        computer.instruction_pointer = get!(computer.ram, in2_pos, 0)
    else
        computer.instruction_pointer += 3
    end
end

function less_than(computer::Computer, in1_pos::Int, in2_pos::Int, out_pos::Int)
    if get!(computer.ram, in1_pos, 0) < get!(computer.ram, in2_pos, 0)
        computer.ram[out_pos] = 1
    else
        computer.ram[out_pos] = 0
    end
    computer.instruction_pointer += 4
end

function equals(computer::Computer, in1_pos::Int, in2_pos::Int, out_pos::Int)
    if get!(computer.ram, in1_pos, 0) == get!(computer.ram, in2_pos, 0)
        computer.ram[out_pos] = 1
    else
        computer.ram[out_pos] = 0
    end
    computer.instruction_pointer += 4
end

function offset_relative_base(computer::Computer, in_pos::Int)
    computer.relative_base = get!(computer.ram, in_pos, 0) + computer.relative_base
    computer.instruction_pointer += 2
end
end
