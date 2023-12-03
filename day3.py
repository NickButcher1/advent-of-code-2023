f = open("input/input03")
# f = open("input/input03-sample")

# Load input a 2D array.
lines = []
for line in f.readlines():
    lines.append(line.rstrip())

num_rows = len(lines)
num_cols = len(lines[0])
print("R " + str(num_rows))
print("C " + str(num_cols))

is_symbol = [ [False]*(num_rows+2) for i in range(num_cols+2)]
digit = [ [" "]*(num_rows+2) for i in range(num_cols+2)]
star_number = [ [0]*(num_rows+2) for i in range(num_cols+2)]
next_star = 1
for r in range(num_rows):
    for c in range(num_cols):
        char = lines[r][c]
        is_symbol[r+1][c+1] = char != "." and not char.isdigit()
        if char.isdigit():
            digit[r+1][c+1] = char
        else:
            digit[r+1][c+1] = " "
        if char == "*":
            star_number[r+1][c+1] = next_star
            next_star += 1

for row in star_number:
    print(row)
# print(is_symbol)
# print(digit)

is_adjacent_symbol = [ [False]*(num_rows+2) for i in range(num_cols+2)]
is_adjacent_star_numbers = [ [None]*(num_rows+2) for i in range(num_cols+2)]

for r in range(num_rows+2):
    for c in range(num_cols+2):
        is_adjacent_star_numbers[r][c] = []

for r in range(1, num_rows+1):
    for c in range(1, num_cols+1):
        if is_symbol[r][c]:
            is_adjacent_symbol[r][c] = True
        if is_symbol[r][c-1]:
            is_adjacent_symbol[r][c] = True
        if is_symbol[r][c+1]:
            is_adjacent_symbol[r][c] = True
        if is_symbol[r-1][c]:
            is_adjacent_symbol[r][c] = True
        if is_symbol[r-1][c-1]:
            is_adjacent_symbol[r][c] = True
        if is_symbol[r-1][c+1]:
            is_adjacent_symbol[r][c] = True
        if is_symbol[r+1][c]:
            is_adjacent_symbol[r][c] = True
        if is_symbol[r+1][c-1]:
            is_adjacent_symbol[r][c] = True
        if is_symbol[r+1][c+1]:
            is_adjacent_symbol[r][c] = True

        if star_number[r][c-1] != 0:
            is_adjacent_star_numbers[r][c].append(star_number[r][c-1])
        if star_number[r][c+1] != 0:
            is_adjacent_star_numbers[r][c].append(star_number[r][c+1])
        if star_number[r][c] != 0:
            is_adjacent_star_numbers[r][c].append(star_number[r][c])
        if star_number[r-1][c-1] != 0:
            is_adjacent_star_numbers[r][c].append(star_number[r-1][c-1])
        if star_number[r-1][c+1] != 0:
            is_adjacent_star_numbers[r][c].append(star_number[r-1][c+1])
        if star_number[r-1][c] != 0:
            is_adjacent_star_numbers[r][c].append(star_number[r-1][c])
        if star_number[r+1][c-1] != 0:
            is_adjacent_star_numbers[r][c].append(star_number[r+1][c-1])
        if star_number[r+1][c+1] != 0:
            is_adjacent_star_numbers[r][c].append(star_number[r+1][c+1])
        if star_number[r+1][c] != 0:
            is_adjacent_star_numbers[r][c].append(star_number[r+1][c])
print("ADJACENT")
for row in is_adjacent_star_numbers:
    print(row)

star_list = []
for i in range(next_star + 1):
    star_list.append([])
total_part_1 = 0
for r in range(1, num_rows+1):
    for c in range(1, num_cols+1):
        # print("TEST: " + digit[r][c] + digit[r][c-1])
        matching_star_numbers = []
        is_allowed = False
        if digit[r][c] != " " and digit[r][c-1] == " ":
            digit_str = ""
            cx = c
            while digit[r][cx] != " ":
                digit_str += digit[r][cx]
                is_allowed = is_allowed or is_adjacent_symbol[r][cx]
                matching_star_numbers += is_adjacent_star_numbers[r][cx]
                # print("DIGIT: " + digit_str + "  " + str(is_adjacent_star_numbers[r][cx]))
                cx = cx + 1
            this_number = int(digit_str)
            matching_star_numbers = list(set(matching_star_numbers))
            print(digit_str + "  " + str(is_allowed) + "  " + str(matching_star_numbers))
            for star_number in matching_star_numbers:
                star_list[star_number].append(this_number)
            if is_allowed:
                total_part_1 += this_number
print("total_part_1: " + str(total_part_1))

total_part_2 = 0
for star_line in star_list:
    print(star_line)
    if len(star_line) == 2:
        this_number = star_line[0] * star_line[1]
        total_part_2 += this_number
print("total_part_2: " + str(total_part_2))