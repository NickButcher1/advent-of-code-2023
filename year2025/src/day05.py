from itertools import product


# This can surely be made more efficient. It stops after finding the first pair of ranges it can combine.
def combine_ranges(ranges: list[tuple[int, int]]) -> list[tuple[int, int]]:
    ranges = sorted(ranges, key=lambda x: x[0], reverse=False)
    for i, j in product(range(len(ranges)), range(len(ranges))):
        if i < j and ranges[j][0] <= ranges[i][1]:
            new_range = (ranges[i][0], max(ranges[i][1], ranges[j][1]))
            ranges = ranges[:j] + ranges[j + 1 :]
            ranges = ranges[:i] + ranges[i + 1 :]
            ranges.append(new_range)
            return ranges

    return ranges


def solve(input_file: str) -> tuple[int, int]:
    with open(input_file) as f:
        raw_lines = f.read().splitlines()
        empty_index = raw_lines.index("")
        good_ranges = [tuple(map(int, s.split("-"))) for s in raw_lines[:empty_index]]
        ids = list(map(int, raw_lines[empty_index + 1 :]))

        # Part one.
        fresh_ids = 0
        for id in ids:
            for i in range(len(good_ranges)):
                if good_ranges[i][0] <= id <= good_ranges[i][1]:
                    fresh_ids += 1
                    break

        # Part two.
        last_len = -1
        while last_len != len(good_ranges):
            last_len = len(good_ranges)
            good_ranges = combine_ranges(good_ranges)

        total_fresh_ids = 0
        for i in range(len(good_ranges)):
            total_fresh_ids += good_ranges[i][1] - good_ranges[i][0] + 1

    return fresh_ids, total_fresh_ids


print(f"Sample: {solve('../input/2025/input05-sample')}")
print(f"Answer: {solve('../input/2025/input05')}")
