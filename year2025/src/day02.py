def count_invalid_numbers_in_range(
    begin: str, end: str, num_sequences: int, invalid_numbers: set[int]
) -> None:
    begin_int = int(begin)
    end_int = int(end)

    if len(begin) % num_sequences != 0:
        begin = "1" + "0" * len(begin)

    test_len = int(len(begin) / num_sequences)
    test_int = int(str(begin)[:test_len])

    while True:
        full_test_int = int(str(test_int) * num_sequences)
        if full_test_int < begin_int:
            test_int += 1
            continue
        if full_test_int <= end_int:
            invalid_numbers.add(full_test_int)
            test_int += 1
        else:
            break


def solve(input_file: str) -> tuple[int, int]:
    invalid_numbers_part_one: set[int] = set()
    invalid_numbers_part_two: set[int] = set()

    f = open(input_file)
    line = f.readline()
    ranges = line.split(",")

    for this_range in ranges:
        begin, end = this_range.split("-")

        count_invalid_numbers_in_range(begin, end, 2, invalid_numbers_part_one)

        for num_sequences in range(2, int(len(end) + 1)):
            count_invalid_numbers_in_range(begin, end, num_sequences, invalid_numbers_part_two)

    return sum(invalid_numbers_part_one), sum(invalid_numbers_part_two)


print(f"Sample: {solve('../input/2025/input02-sample')}")
print(f"Answer: {solve('../input/2025/input02')}")
