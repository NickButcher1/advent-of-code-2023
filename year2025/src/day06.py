import math

def solve(input_file: str) -> tuple[int, int]:
    rows = []
    with open(input_file) as f:
        for line in f:
            rows.append(line.strip().split())
    operators = rows.pop()

    part_one_answer = 0
    for col in range(len(rows[0])):
        vals = [int(rows[row][col]) for row in range(len(rows))]
        if operators[col] == "+":
            col_answer = sum(vals)
        else:
            col_answer = math.prod(vals)
        part_one_answer += col_answer

    return part_one_answer, 0


print(f"Sample: {solve('../input/2025/input06-sample')}")
print(f"Answer: {solve('../input/2025/input06')}")
