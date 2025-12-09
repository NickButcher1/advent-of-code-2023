def solve(input_file: str) -> tuple[int, int]:
    with open(input_file) as f:
        rows = [tuple(int(item.strip()) for item in line.rstrip().split(",")) for line in f]
        # print(rows)

        area_max = 0
        for i in range(len(rows)):
            for j in range(i + 1, len(rows)):
                area = abs(rows[i][0] - rows[j][0] + 1) * abs(rows[i][1] - rows[j][1] + 1)
                print(f"Comparing point {i} {rows[i]} to point {j} {rows[j]}: area={area}")
                if area > area_max:
                    area_max = area
    return area_max, 0


print(f"Sample: {solve('../input/2025/input09-sample')}")
print(f"Answer: {solve('../input/2025/input09')}")
