path = "data/Day06_input.txt"
if !ispath(path)
    path = joinpath("..", path)
    if !ispath(path)
        exit(1)
    end
end

orbits = open(path, "r") do infile
    return readlines(infile)
end

mutable struct Body
    name::String
    parent::String
    children::Array{String, 1}
end

function make_body(name::AbstractString)
    Body(name, "", [])
end

solar_system = Dict{String, Body}()
for orbit in orbits
    bodies = split(orbit, ")")
    for body in bodies
        if !haskey(solar_system, body)
            solar_system[body] = make_body(body)
        end
    end
    push!(solar_system[bodies[1]].children, bodies[2])
    solar_system[bodies[2]].parent = bodies[1]
end

# Part 1
num_orbits = 0
for (name, body) in pairs(solar_system)
    parent = get(solar_system, body.parent, nothing)
    while parent !== nothing
        global num_orbits += 1
        parent = get(solar_system, parent.parent, nothing)
    end
end
println(num_orbits)

# Part 2
you = solar_system["YOU"]
you_parents = Body[]
let
parent = get(solar_system, you.parent, nothing)
while parent !== nothing
    push!(you_parents, parent)
    parent = get(solar_system, parent.parent, nothing)
end
end

santa = solar_system["SAN"]
parent = get(solar_system, santa.parent, nothing)
santa_parents = [parent]
jumps = 1
while parent ∉ you_parents
    global jumps += 1
    global parent = get(solar_system, parent.parent, nothing)
    push!(santa_parents, parent)
end

for body in you_parents
    global jumps += 1
    if isequal(body, santa_parents[end])
        break
    end
end

println(jumps - 2)