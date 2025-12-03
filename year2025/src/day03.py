def find_best_digit(input_str: str) -> tuple[int, int]:
    for digit in range(9, 0, -1):
        position = input_str.find(str(digit))
        if position != -1:
            return digit, position
    return 0, -1


def solve_for_batteries(input_file: str, num_batteries: int) -> int:
    total_joltage = 0

    with open(input_file) as f:
        for line in f:
            row = line.strip()

            joltage = 0
            start_pos = 0
            end_pos = 1 - num_batteries

            for _ in range(num_batteries):
                digit, pos = find_best_digit(row[start_pos : (end_pos or None)])

                start_pos += pos + 1
                end_pos += 1
                joltage = joltage * 10 + digit

            total_joltage += joltage

    return total_joltage


def solve(input_file: str) -> tuple[int, int]:
    return solve_for_batteries(input_file, 2), solve_for_batteries(input_file, 12)


print(f"Sample: {solve('../input/2025/input03-sample')}")
print(f"Answer: {solve('../input/2025/input03')}")
