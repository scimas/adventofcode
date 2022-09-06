import os

class Body:
    def __init__(self, name):
        self.name = name
        self.parent = None
        self.children = []
    

    def add_parent(self, body):
        self.parent = body
    

    def add_child(self, body):
        self.children.append(body)


base_dir = os.getcwd()
data_dir = os.path.join(base_dir, "data")

bodies = {}
COM = None
with open(os.path.join(data_dir, "Day06_input.csv"), "r") as local_map:
    relation = local_map.readline().strip()
    while relation:
        parent_n, child_n = relation.split(")")
        if parent_n not in bodies.keys():
            parent = Body(parent_n)
            bodies[parent_n] = parent
        else:
            parent = bodies[parent_n]
        if child_n not in bodies.keys():
            child = Body(child_n)
            bodies[child_n] = child
        else:
            child = bodies[child_n]
        child.add_parent(parent)
        parent.add_child(child)
        relation = local_map.readline().strip()

# Part 1
orbits = 0
print(len(bodies.keys()))
for body_n, body in bodies.items():
    parent = body.parent
    while parent:
        orbits += 1
        parent = parent.parent

print(orbits)

# Part 2
you = bodies["YOU"]
san = bodies["SAN"]
you_parents = []
san_parents = []

parent = you.parent
while parent:
    you_parents.append(parent.name)
    parent = parent.parent

parent = san.parent
while parent:
    san_parents.append(parent.name)
    parent = parent.parent

common_ancestor = ""
found = False
for you_par in you_parents:
    for san_par in san_parents:
        if you_par == san_par:
            common_ancestor = you_par
            found = True
            break
    if found:
        break

jumps = you_parents.index(common_ancestor) + san_parents.index(common_ancestor)
print(jumps)
