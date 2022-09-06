lower = 284639
upper = 758759

part1_possibilities = 0
part2_possibilities = 0

for i in lower:upper
    rep = [string(i)...]
    if rep != sort(rep)
        continue
    else
        unique_digits = unique(rep)
        counts = [count(x -> x == i, rep) for i in unique_digits]
        if maximum(counts) >= 2
            global part1_possibilities += 1
        end
        if 2 in counts
            global part2_possibilities += 1
        end 
    end
end

println(possibilities)