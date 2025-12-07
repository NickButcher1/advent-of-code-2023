def display_grid(grid: list[str], num_ways: list[list[int]]) -> None:
    print("\n\n")
    for i in range(len(grid)):
        print(f"{grid[i]}    {num_ways[i]}")


def solve(input_file: str) -> tuple[int, int]:
    with open(input_file) as f:
        rows = [line.rstrip() for line in f]

    num_ways = [[0] * len(rows[0]) for _ in range(len(rows))]
    num_ways[0][rows[0].index("S")] = 1
    rows[0] = rows[0].replace("S", "|")
    # display_grid(rows, num_ways)

    num_splits = 0

    for i in range(len(rows) - 1):  # No action on last row.
        this_row = rows[i]
        next_row = rows[i + 1]
        new_next_row = list(next_row)  # List of char.

        for j in range(len(this_row)):
            if this_row[j] == "|":
                if next_row[j] == ".":
                    new_next_row[j] = "|"
                    num_ways[i + 1][j] += num_ways[i][j]
                elif next_row[j] == "^":
                    new_next_row[j - 1] = "|"
                    new_next_row[j + 1] = "|"
                    num_splits += 1
                    num_ways[i + 1][j - 1] += num_ways[i][j]
                    num_ways[i + 1][j + 1] += num_ways[i][j]

        rows[i + 1] = "".join(new_next_row)

        # display_grid(rows, num_ways)

    return num_splits, sum(num_ways[len(rows) - 1])


print(f"Sample: {solve('../input/2025/input07-sample')}")
print(f"Answer: {solve('../input/2025/input07')}")
