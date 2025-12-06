import math


def solve_part_one(input_file: str) -> int:
    rows = []
    with open(input_file) as f:
        for line in f:
            rows.append(line.strip().split())
    operators = rows.pop()

    total_answer = 0
    for col in range(len(rows[0])):
        vals = [int(rows[row][col]) for row in range(len(rows))]
        if operators[col] == "+":
            col_answer = sum(vals)
        else:
            col_answer = math.prod(vals)
        total_answer += col_answer

    return total_answer


def solve_part_two(input_file: str) -> int:
    rows = []
    with open(input_file) as f:
        for line in f:
            # Reverse each line. Terminate with a space to make parsing easier later (the numbers for each
            # operator will terminate with a column of all spaces.
            rows.append(line.strip("\n")[::-1] + " ")
    operators = rows.pop().strip().split()[::-1]  # Unreverse operators.

    total_answer = 0
    i = 0
    vals = []

    while i < len(rows[0]):
        if all(s[i] == " " for s in rows):
            # End of input for this operator.
            operator = operators.pop()
            if operator == "+":
                col_answer = sum(vals)
            else:
                col_answer = math.prod(vals)
            total_answer += col_answer
            vals = []
        else:
            # Parse the next column.
            num_str = ""
            for r in range(len(rows)):
                if rows[r][i] != " ":
                    num_str += rows[r][i]
            vals.append(int(num_str))
        i += 1

    return total_answer


def solve(input_file: str) -> tuple[int, int]:
    return solve_part_one(input_file), solve_part_two(input_file)


print(f"Sample: {solve('../input/2025/input06-sample')}")
print(f"Answer: {solve('../input/2025/input06')}")
