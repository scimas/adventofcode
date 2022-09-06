masses = open("data/Day01_input.txt", "r") do f
    parse.(Int, readlines(f))
end

# Part 1
fuel = div.(masses, 3) .- 2

println(sum(fuel))

# Part 2
for i in 1:length(fuel)
    extra_fuel = div(fuel[i], 3) - 2
    while extra_fuel > 0
        fuel[i] += extra_fuel
        extra_fuel = div(extra_fuel, 3) - 2
    end
end

println(sum(fuel))
