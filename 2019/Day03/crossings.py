import os
import re

import numpy as np

def generate_coords(sequence):
    move_pattern = re.compile(r"([A-Z])([0-9]+)")
    coords = [(0, 0)]
    for move in move_pattern.findall(sequence):
        if move[0] == "U":
            next_coord = (coords[-1][0], coords[-1][1] + int(move[1]))
        elif move[0] == "D":
            next_coord = (coords[-1][0], coords[-1][1] - int(move[1]))
        elif move[0] == "L":
            next_coord = (coords[-1][0] - int(move[1]), coords[-1][1])
        elif move[0] == "R":
            next_coord = (coords[-1][0] + int(move[1]), coords[-1][1])
        coords.append(next_coord)
    return coords


def get_crossings(wire1, wire2):
    possible_crossings = []
    crossing_indices = []
    for i in range(len(wire1) - 1):
        for j in range(len(wire2) - 1):
            x_diff = (wire1[i][0] - wire2[j][0]) * (wire1[i+1][0] - wire2[j+1][0])
            y_diff = (wire1[i][1] - wire2[j][1]) * (wire1[i+1][1] - wire2[j+1][1])
            if (x_diff > 0) or (y_diff > 0):
                continue
            crossing_indices.append((i, j))
            if x_diff == 0:
                ys = sorted([wire1[i][1], wire1[i+1][1], wire2[j][1], wire2[j+1][1]])
                possible_crossings.append((wire1[i][0], ys[1]))
                possible_crossings.append((wire1[i][0], ys[2]))
            elif y_diff == 0:
                xs = sorted([wire1[i][0], wire1[i+1][0], wire2[j][0], wire2[j+1][0]])
                possible_crossings.append((xs[1], wire1[i][1]))
                possible_crossings.append((xs[2], wire1[i][1]))
            else:
                xs = sorted([wire1[i][0], wire1[i+1][0], wire2[j][0], wire2[j+1][0]])
                ys = sorted([wire1[i][1], wire1[i+1][1], wire2[j][1], wire2[j+1][1]])
                possible_crossings.append((xs[1], ys[1]))
    
    return possible_crossings, crossing_indices


def manhattan_distance(x0, y0, x1, y1):
    return abs(x1 - x0) + abs(y1 - y0)


base_dir = os.getcwd()
data_dir = os.path.join(base_dir, "data")
with open(os.path.join(data_dir, "Day03_input.csv"), "r") as wires:
    wire1 = wires.readline().strip()
    wire2 = wires.readline().strip()

wire1 = generate_coords(wire1)
wire2 = generate_coords(wire2)
possible_crossings, crossing_indices = get_crossings(wire1, wire2)
distances = [manhattan_distance(*p, *(0, 0)) for p in possible_crossings]
distances = np.array(distances)
mask = np.argsort(distances)

# Part 1
min_index = np.argmax(distances[mask] != 0)
print(distances[mask][min_index])

# Part 2
crossing_steps = []
for crossing in crossing_indices:
    if crossing[0] == 0 and crossing[1] == 0:
        continue
    steps1 = 0
    for i in range(crossing[0]):
        steps1 += manhattan_distance(*wire1[i], *wire1[i+1])
    steps2 = 0
    for j in range(crossing[1]):
        steps2 += manhattan_distance(*wire2[j], *wire2[j+1])
    xs = sorted([wire1[crossing[0]][0], wire1[crossing[0]+1][0], wire2[crossing[1]][0], wire2[crossing[1]+1][0]])
    ys = sorted([wire1[crossing[0]][1], wire1[crossing[0]+1][1], wire2[crossing[1]][1], wire2[crossing[1]+1][1]])
    steps1 += manhattan_distance(*wire1[crossing[0]], xs[2], ys[2])
    steps2 += manhattan_distance(*wire2[crossing[1]], xs[2], ys[2])
    crossing_steps.append(steps1 + steps2)

print(min(crossing_steps))
