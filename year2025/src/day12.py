from dataclasses import dataclass

EMPTY = ord(".")
FILLED = ord("#")

NUM_PRESENT_TYPES = 6


@dataclass
class Present:
    rows: list[list[int]]

    def print(self) -> None:
        print("\n")
        for r in self.rows:
            print("".join(chr(c) for c in r))

    def rotate(self) -> "Present":
        # Rotate 90 degrees clockwise.
        new_rows = [
            [self.rows[2][0], self.rows[1][0], self.rows[0][0]],
            [self.rows[2][1], self.rows[1][1], self.rows[0][1]],
            [self.rows[2][2], self.rows[1][2], self.rows[0][2]],
        ]
        return Present(rows=new_rows)

    def flip(self) -> "Present":
        # Flip horizontally.
        new_rows = [
            [self.rows[0][2], self.rows[0][1], self.rows[0][0]],
            [self.rows[1][2], self.rows[1][1], self.rows[1][0]],
            [self.rows[2][2], self.rows[2][1], self.rows[2][0]],
        ]
        return Present(rows=new_rows)

    def hash_count(self) -> int:
        count = 0
        for r in self.rows:
            for c in r:
                if c == FILLED:
                    count += 1
        return count


class Grid:
    def __init__(self, width: int, length: int) -> None:
        self.width = width
        self.length = length
        self.num_cells = width * length
        self.cells: list[list[int]] = [[EMPTY for _ in range(width)] for _ in range(length)]

    def print(self) -> None:
        print(f"\n{self.width}x{self.length}, cells={self.num_cells}:")
        for r in self.cells:
            print("".join(chr(c) for c in r))


def solve(input_file: str) -> tuple[int, int]:
    with open(input_file) as f:
        lines = [line.rstrip() for line in f]

    # Read the unrotated/flipped presents.
    presents: list[list[Present]] = []
    for i in range(1, 27, 5):
        present = Present(rows=[[ord(c) for c in lines[i + r]] for r in range(3)])
        presents.append([present])

    # Now add all rotations/flips that aren't duplicates.
    for i in range(NUM_PRESENT_TYPES):
        for flip in [False, True]:
            try_present = presents[i][0]
            if flip:
                try_present = try_present.flip()
            for rotation in range(4):
                if try_present not in presents[i]:
                    presents[i].append(try_present)
                try_present = try_present.rotate()

    # for i in range(NUM_PRESENT_TYPES):
    #     print(f"\nPRESENT: {i} has {len(presents[i])} variations")
    #     for present in presents[i]:
    #         present.print()

    num_possible_part_one = 0

    for i in range(30, len(lines)):
        # print(f"\n{lines[i]}")

        required_present_counts = [int(x) for x in lines[i].split(":")[1].strip().split()]
        grid = Grid(
            width=int(lines[i].split(":")[0].split("x")[0]), length=int(lines[i].split(":")[0].split("x")[1])
        )
        # grid.print()
        hash_to_place = 0
        for j in range(NUM_PRESENT_TYPES):
            hash_to_place += presents[j][0].hash_count() * required_present_counts[j]
        # print(f"Need to place {hash_to_place} # symbols")

        # This eliminates 474/1,000 cases immediately for my input.
        if hash_to_place > grid.num_cells:
            # print("Impossible to fit all presents!")
            continue

        # The remaining cases all appear to be possible given they have about 30% slack.
        slack_cells = grid.num_cells - hash_to_place
        slack_percent = int(100 * slack_cells / grid.num_cells)
        print(f"{slack_cells:>6} out of {grid.num_cells:>6}, slack percent={slack_percent:>3}%")

        num_possible_part_one += 1

    return num_possible_part_one, 0


print(f"Sample: {solve('../input/2025/input12-sample')}")
print(f"Answer: {solve('../input/2025/input12')}")
