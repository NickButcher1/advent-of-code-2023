##############################################################################################################
# Simple brute force solution. I know I could have done something smarter, for example if checking range
# 123000 to 999999, for part one:
# - start at 123000, jump straight to 123123, check if that's in range
# - increment first half to 124000, jump straight to 124124, check if that's in range.
##############################################################################################################

def is_number_invalid_part_one(number: int) -> bool:
    number_to_test = str(number)
    if len(number_to_test) % 2 == 0:
        half_len = int(len(number_to_test) / 2)
        if number_to_test[half_len:] == number_to_test[:half_len]:
            return True
    return False

def is_number_invalid_part_two(number: int) -> bool:
    number_to_test = str(number)
    half_len = int(len(number_to_test) / 2)

    for len_to_test in range(1, half_len + 1):
        if len(number_to_test) % len_to_test == 0:
            parts = [
                number_to_test[i : i + len_to_test]
                for i in range(0, len(number_to_test), len_to_test)
            ]
            if all(x == parts[0] for x in parts):
                return True
    return False

def solve(input_file: str) -> tuple[int, int]:
    sum_numbers_invalid_part_one = 0
    sum_numbers_invalid_part_two = 0

    f = open(input_file)
    line = f.readline()
    ranges = line.split(",")
    
    for this_range in ranges:
        begin, end = this_range.split("-")
        begin_int = int(begin)
        end_int = int(end)

        for i in range(begin_int, end_int + 1):
            if is_number_invalid_part_one(i):
                sum_numbers_invalid_part_one += i
            if is_number_invalid_part_two(i):
                sum_numbers_invalid_part_two += i

    return sum_numbers_invalid_part_one, sum_numbers_invalid_part_two

print(f"Sample: {solve("../input/2025/input02-sample")}")
print(f"Answer: {solve("../input/2025/input02")}")
