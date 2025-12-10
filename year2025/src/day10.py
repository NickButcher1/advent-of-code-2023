import ast
import itertools


def choose_buttons_to_press(num_buttons: int, how_many_to_choose: int):
    """Generate all ways to choose N values from range 0 to M (with repeats)."""
    yield from itertools.combinations(range(num_buttons), how_many_to_choose)

def solve(input_file: str) -> tuple[int, int]:
    # [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    with open(input_file) as f:
        rows = [line.rstrip().split() for line in f]

    # From inspection of my input.
    max_bits = 10 # 6 for sample

    part_one_answer = 0

    for row in rows:
        lights_str = row[0][1:-1].ljust(max_bits, ".") # Pad to max bits with lights OFF.
        target_lights = int("".join("1" if bit == "#" else "0" for bit in reversed(lights_str)), 2)

        buttons_list = [ast.literal_eval(buttons) for buttons in (row[1:-1])]
        # eval converts tuple of length 1 to int. Fix that.
        buttons_list = [buttons if isinstance(buttons, tuple) else (buttons,) for buttons in buttons_list]
        buttons_masks = [sum(1 << bit for bit in buttons) for buttons in buttons_list]

        joltage: set[int] = ast.literal_eval(row[-1])

        print(f"{lights_str}  {str(target_lights):>6}    {buttons_masks}    {joltage}")

        # Find minimum presses for this row.
        num_buttons_to_press = 0
        solved = False
        print(f"SOLVE ROW: {row}")
        while not solved:
            num_buttons_to_press += 1
            for buttons_to_press in choose_buttons_to_press(len(buttons_masks), num_buttons_to_press):
                lights_after_presses = 0
                for button_index in buttons_to_press:
                    lights_after_presses = lights_after_presses ^ buttons_masks[button_index]
                print(f"    Trying buttons {buttons_to_press} gives {lights_after_presses}  ")

                if lights_after_presses == target_lights:
                    part_one_answer += num_buttons_to_press
                    solved = True
                    break

    return part_one_answer, 0


print(f"Sample: {solve('../input/2025/input10-sample')}")
print(f"Answer: {solve('../input/2025/input10')}")
