import re

lower = 284639
upper = 748759

repeat_pattern = re.compile(r"(([0-9])\2+)")
possible_passwords = []
for i in range(lower, upper + 1):
    rep = str(i)
    problem = False
    for j in range(5):
        if rep[j] > rep[j+1]:
            problem = True
            break
    if problem:
        continue
    elif repeat_pattern.findall(rep):
        possible_passwords.append(i)

# Part 1
print(len(possible_passwords))

# Part 2
test_pass = possible_passwords.copy()
possible_passwords = []
for i in test_pass:
    rep = str(i)
    for match in repeat_pattern.findall(rep):
        if len(match[0]) == 2:
            possible_passwords.append(i)
            break

print(len(possible_passwords))
