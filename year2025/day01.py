def solve(input_file: str) -> tuple[int, int]:
    position = 50
    part_one_answer = 0
    part_two_answer = 0
    f = open(input_file)
    for line in f.readlines():
        is_positive = 1 if line[0] == "R" else -1
        offset = int(line[1:])

        hundreds, offset = divmod(offset, 100)
        part_two_answer += hundreds

        offset = offset * is_positive

        if position != 0:
            if offset > 0 and (offset + position) >= 100:
                part_two_answer += 1
            if offset < 0 and abs(offset) >= position:
                part_two_answer += 1

        position = (position + offset) % 100
        if position == 0:
            part_one_answer += 1
    
    return part_one_answer, part_two_answer

print(f"Sample: {solve("../input/2025/input01-sample")}")
print(f"Answer: {solve("../input/2025/input01")}")
