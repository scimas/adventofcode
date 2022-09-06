function get_points(line::String)
    points = Dict()
    movex = Dict([('L', -1), ('R', 1), ('D', 0), ('U', 0)])
    movey = Dict([('L', 0), ('R', 0), ('D', -1), ('U', 1)])

    x, y = 0, 0
    distance = 0
    for instr in split(line, ",")
        n_moves = parse(Int, instr[2:end])
        deltax = movex[instr[1]]
        deltay = movey[instr[1]]
        for _ in 1:n_moves
            x += deltax
            y += deltay
            distance += 1
            if !haskey(points, (x, y))
                points[(x, y)] = distance
            end
        end
    end
    points
end

path = "data/Day03_input.txt"
if !ispath(path)
    path = joinpath("..", path)
    if !ispath(path)
        exit(1)
    end
end

lines = open(path, "r") do infile
    return readlines(infile)
end

lineA = get_points(lines[1])
lineB = get_points(lines[2])
common_points = intersect(collect(keys(lineA)), collect(keys(lineB)))

# Part 1
println(minimum(map(x -> abs(x[1]) + abs(x[2]), common_points)))

# Part 2
println(map(x -> lineA[x] + lineB[x], common_points) |> minimum)