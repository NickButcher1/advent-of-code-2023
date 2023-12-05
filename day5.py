# f = open("input/input05-sample")
f = open("input/input05")

# Read input into memory.
seeds = []
maps = [[], [], [], [], [], [], []]
current_map_id = -1
for line in f.readlines():
    line = line.rstrip()
    if line.startswith("seeds:"):
        seeds = [int(s) for s in line.split("seeds: ")[1].split(' ')]
    elif line.count(" map:") == 1:
        current_map_id += 1
    elif line == "":
        pass
    else:
        maps[current_map_id].append([int(s) for s in line.split(' ')])

def map_a_to_b(value, map, map_depth, all_values):
    for block in map:
        # print("    TRY BLOCK: " + str(block))
        dest = int(block[0])
        source = int(block[1])
        block_len = int(block[2])
        if value >= source and value <= (source + block_len - 1):
            value = value + dest - source
            all_values.append(value)
            return(value)

    all_values.append(value)
    return value

def seed_to_location(seed, maps):
    # print("SEED: " + str(seed))
    current_value = seed
    map_depth = 0
    all_values = [seed]
    for map in maps:
        current_value = map_a_to_b(current_value, map, map_depth, all_values)

        map_depth += 1

    return current_value

# Part 1.
min_location = 999999999999999
for seed in seeds:
    min_location = min(seed_to_location(seed, maps), min_location)

print("Part 1: " + str(min_location))


def test_valid_seed(value, location):
    valid_seed = False
   
    for i in range(0, len(seeds), 2):
        first_seed = seeds[i]
        block_len = seeds[i+1]

        if value >= first_seed and value <= (first_seed + block_len - 1):
            print("VALID SEED " + str(value) + " INDEX " + str(i) + " BLOCK " + str(seeds[i]) + " LEN " + str(seeds[i+1]))
            valid_seed = True
            break
    
    if valid_seed:
        print("Part 2: " + str(location))
        exit()

def map_b_to_a(location, value, map, map_depth):
    for block in map:
        dest = int(block[0])
        source = int(block[1])
        block_len = int(block[2])
        if value >= dest and value <= (dest + block_len - 1):
            value = value - dest + source
            
            return value

    return value

def try_location(location):
    current_value = location
    map_depth = 0
    for map in maps:
        current_value = map_b_to_a(location, current_value, map, map_depth)
        map_depth += 1

    test_valid_seed(current_value, location)
    
    return False

# Part 2.
location = 0
found = False
# Start with location.
maps.reverse()

while not found:
    found = try_location(location)
    location += 1


print("Part 2: " + str(location))