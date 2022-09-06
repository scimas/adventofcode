import os
import numpy as np
from matplotlib import pyplot as plt

base_dir = os.getcwd()
data_dir = os.path.join(base_dir, "data")
with open(os.path.join(data_dir, "Day08_input.csv"), "r") as image_file:
    image_str = image_file.readline().strip()

cols = 25
rows = 6
layers = len(image_str) // (rows * cols)

# Part 1
image = []
count = 0
for l in range(layers):
    image.append([])
    for r in range(rows):
        image[l].append([])
        for c in range(cols):
            image[l][r].append(int(image_str[count]))
            count += 1

image = np.array(image)
num_zeros = np.sum(np.sum(image == 0, axis=1), axis=1)
fewest_zero_layer = np.argmin(num_zeros)

print(np.sum(image[fewest_zero_layer] == 1) * np.sum(image[fewest_zero_layer] == 2))

# Part 2
mask = np.argmax(image != 2, axis=0)
final_image = []
for r in range(rows):
    final_image.append([])
    for c in range(cols):
        final_image[r].append(image[mask[r,c], r, c])

final_image = np.array(final_image)
plt.imshow(final_image, cmap="gray")
plt.show()
