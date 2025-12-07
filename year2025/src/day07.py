def display_grid(grid: list[str]) -> None:
    print("\n\n")
    for row in grid:
        print(row)

def solve_part_one(input_file: str) -> int:
    with open(input_file) as f:
        rows = [line.rstrip() for line in f]

    display_grid(rows)
    rows[0] = rows[0].replace("S", "|")

    num_splits = 0

    for i in range(len(rows) - 1): # No action on last row.
        this_row = rows[i]
        next_row = rows[i+1]
        new_next_row = list(next_row) # List of char.

        for j in range(len(this_row)):
            if this_row[j] == "|":
                if next_row[j] == ".":
                    new_next_row[j] = '|'
                elif next_row[j] == "^":
                    new_next_row[j-1] = '|'
                    new_next_row[j+1] = '|'
                    num_splits += 1


        rows[i+1] = "".join(new_next_row)

        display_grid(rows)

    return num_splits


def solve_part_two(input_file: str) -> int:
    return 0


def solve(input_file: str) -> tuple[int, int]:
    return solve_part_one(input_file), solve_part_two(input_file)


print(f"Sample: {solve('../input/2025/input07-sample')}")
print(f"Answer: {solve('../input/2025/input07')}")
