def solve(input_file: str) -> tuple[int, int]:
    with open(input_file) as f:
        raw_lines = f.read().splitlines()
        empty_index = raw_lines.index("")
        good_ranges = [tuple(map(int, s.split("-"))) for s in raw_lines[:empty_index]]
        ids = list(map(int, raw_lines[empty_index + 1 :]))
        good_ranges_asc_1 = sorted(good_ranges, key=lambda x: x[0], reverse=False)
        good_ranges_desc_2 = sorted(good_ranges, key=lambda x: x[1], reverse=True)

        print(good_ranges_asc_1)
        print(good_ranges_desc_2)
        print(ids)

        fresh_ids = 0

        for id in ids:
            for i in range(len(good_ranges_asc_1)):
                if good_ranges_asc_1[i][0] <= id <= good_ranges_asc_1[i][1]:
                    fresh_ids += 1
                    break
                if good_ranges_desc_2[i][0] <= id <= good_ranges_desc_2[i][1]:
                    fresh_ids += 1
                    break
    return fresh_ids, 0


print(f"Sample: {solve('../input/2025/input05-sample')}")
print(f"Answer: {solve('../input/2025/input05')}")
