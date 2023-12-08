f = open("input/input08")
# f = open("input/input08-sample")

lines = []
for line in f.readlines():
    lines.append(line.rstrip())

directions = []
from_x_to_left = {}
from_x_to_right = {}
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

print(directions)
print(from_x_to_left)
print(from_x_to_right)

location = "AAA"
steps = 0

while location != "ZZZ":
    for direction in directions:
        steps += 1
        old_location = location
        if direction == "L":
            location = from_x_to_left[location]
            print("DBG-2L " + str(direction) + "  " + old_location + " -> " + location)
        elif direction == "R":
            location = from_x_to_right[location]
            print("DBG-2R " + str(direction) + "  " + old_location + " -> " + location)
        else:
            print("ERROR!")
        if location == "ZZZ":
            print("PART 1: " + str(steps))
            exit()

print("PART 1: " + str(steps))
