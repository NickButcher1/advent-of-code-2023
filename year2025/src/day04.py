def count_and_remove_rolls(lines: list[str]) -> int:
    coords_to_remove: list[tuple[int, int]] = []

    for r in range(1, len(lines) - 1):
        for c in range(1, len(lines[0]) - 1):
            if lines[r][c] == "@":
                num_neighbours = [
                    lines[r - 1][c - 1],
                    lines[r - 1][c],
                    lines[r - 1][c + 1],
                    lines[r][c - 1],
                    lines[r][c + 1],
                    lines[r + 1][c - 1],
                    lines[r + 1][c],
                    lines[r + 1][c + 1],
                ].count("@")

                if num_neighbours < 4:
                    coords_to_remove.append((r, c))

    for r, c in coords_to_remove:
        lines[r] = lines[r][:c] + "." + lines[r][c + 1 :]

    return len(coords_to_remove)


def solve(input_file: str) -> tuple[int, int]:
    with open(input_file) as f:
        raw_lines = f.read().splitlines()

    border = "." * (len(raw_lines[0]) + 2)
    lines = [border] + [f".{line}." for line in raw_lines] + [border]

    part_one_answer = count_and_remove_rolls(lines)
    part_two_answer = part_one_answer
    last_answer = -1

    while last_answer != part_two_answer:
        last_answer = part_two_answer
        part_two_answer += count_and_remove_rolls(lines)

    return part_one_answer, part_two_answer


print(f"Sample: {solve('../input/2025/input04-sample')}")
print(f"Answer: {solve('../input/2025/input04')}")
