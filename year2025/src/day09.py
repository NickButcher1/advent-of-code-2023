from shapely.geometry import Polygon, box


def solve(input_file: str) -> tuple[int, int]:
    with open(input_file) as f:
        rows = [tuple(int(item.strip()) for item in line.rstrip().split(",")) for line in f]

        # Close the polygon loop.
        rows.append(rows[0])

        area_max_part_one = 0
        area_max_part_two = 0
        for i in range(len(rows)):
            for j in range(i + 1, len(rows)):
                area = (abs(rows[i][0] - rows[j][0]) + 1) * (abs(rows[i][1] - rows[j][1]) + 1)
                is_rectangle_in_polygon = box(
                    min(rows[i][0], rows[j][0]),
                    min(rows[i][1], rows[j][1]),
                    max(rows[i][0], rows[j][0]),
                    max(rows[i][1], rows[j][1]),
                    rows,
                ).covered_by(Polygon(rows))
                area_max_part_one = max(area_max_part_one, area)
                if area > area_max_part_two and is_rectangle_in_polygon:
                    area_max_part_two = area
    return int(area_max_part_one), int(area_max_part_two)


print(f"Sample: {solve('../input/2025/input09-sample')}")
print(f"Answer: {solve('../input/2025/input09')}")
