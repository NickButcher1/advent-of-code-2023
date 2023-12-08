import math

f = open("input/input08")

lines = []
for line in f.readlines():
    lines.append(line.rstrip())

directions = []
from_x_to_left = {}
from_x_to_right = {}
start_nodes = []
line_num = 0
for line in lines:
    line_num += 1
    if line_num == 1:
        directions = line
    elif line_num >= 3:
        line = line.split()
        src = line[0]
        left = line[2].replace("(", "").replace(",", "")
        right = line[3].replace(")", "")
        from_x_to_left[src] = left
        from_x_to_right[src] = right
        if src[2] == 'A':
            start_nodes.append(src)

def all_at_zzz(locations):
    num_at_z = 0
    for location in locations:
        if location[2] == 'Z':
            num_at_z += 1

    return num_at_z == len(locations)

num_starts = len(start_nodes)
locations = []
for start_node in start_nodes:
    locations.append(start_node)
steps = 0
first_z = [0] * num_starts

while not all_at_zzz(locations):
    for direction in directions:
        steps += 1
        for id in range(len(locations)):
            old_location = locations[id]
            if direction == "L":
                locations[id] = from_x_to_left[old_location]
                # print("DBG-2L " + str(direction) + "  " + old_location + " -> " + locations[id])
            elif direction == "R":
                locations[id] = from_x_to_right[old_location]
                # print("DBG-2R " + str(direction) + "  " + old_location + " -> " + locations[id])
            else:
                print("ERROR!")
        if all_at_zzz(locations):
            print("PART 2: " + str(steps))
            exit()

        for id in range(num_starts):
            if first_z[id] == 0 and locations[id][2] == 'Z':
                first_z[id] = steps

        if first_z.count(0) == 0:
            answer = first_z[0]
            for id in range(num_starts):
                answer = math.lcm(answer, first_z[id])
            print("STOP " + str(answer))
            exit()

# 0 always DVZ
# 1 always XKZ
# 2 always HSZ
# 3 always GGZ
# 4 always ZZZ
# 5 always HLZ
