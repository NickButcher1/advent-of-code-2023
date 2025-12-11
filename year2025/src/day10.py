import ast
import itertools
from collections.abc import Iterator

import z3


def visualize_bits(number: int, bit_width: int = 10, one_char: str = "#", zero_char: str = ".") -> str:
    binary_string = f"{number:0{bit_width}b}"
    translation_table = str.maketrans("10", f"{one_char}{zero_char}")
    return binary_string.translate(translation_table)[::-1]


def choose_buttons_to_press(num_buttons: int, how_many_to_choose: int) -> Iterator:
    """Generate all ways to choose N values from range 0 to M (with repeats)."""
    yield from itertools.combinations(range(num_buttons), how_many_to_choose)


def solve(input_file: str) -> tuple[int, int]:
    # [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    with open(input_file) as f:
        rows = [line.rstrip().split() for line in f]

    # From inspection of my input.
    max_bits = 10  # 6 for sample

    part_one_answer = 0
    part_two_answer = 0

    for row in rows:
        lights_str = row[0][1:-1].ljust(max_bits, ".")  # Pad to max bits with lights OFF.
        target_lights = int("".join("1" if bit == "#" else "0" for bit in reversed(lights_str)), 2)

        buttons_list = [ast.literal_eval(buttons) for buttons in (row[1:-1])]
        # eval converts tuple of length 1 to int. Fix that.
        buttons_list = [buttons if isinstance(buttons, tuple) else (buttons,) for buttons in buttons_list]
        buttons_masks = [sum(1 << bit for bit in buttons) for buttons in buttons_list]

        joltage_str = "[" + row[-1][1:-1] + "]"
        joltage: set[int] = ast.literal_eval(joltage_str)

        # PART ONE
        num_buttons_to_press = 0
        solved = False
        while not solved:
            num_buttons_to_press += 1
            for buttons_to_press in choose_buttons_to_press(len(buttons_masks), num_buttons_to_press):
                lights_after_presses = 0
                for button_index in buttons_to_press:
                    lights_after_presses = lights_after_presses ^ buttons_masks[button_index]
                if lights_after_presses == target_lights:
                    part_one_answer += num_buttons_to_press
                    solved = True
                    break

        # PART TWO
        C = len(joltage)  # Number of counters
        B = len(buttons_masks)  # Number of buttons

        equations: list[dict[str, int]] = []
        for c in range(C):
            equation: dict[str, int] = {"constant": joltage[c]}
            for b in range(B):
                if c in buttons_list[b]:
                    equation[f"x{b}"] = 1
            equations.append(equation)

        var_names = set()
        for equation in equations:
            var_names.update(k for k in equation if k != "constant")
        variables = {name: z3.Int(name) for name in var_names}

        solver = z3.Optimize()

        sum_str = [z3.Int(f"x{i}") for i in range(B)]
        solver.minimize(sum(sum_str))
        solver.add([var >= 0 for var in sum_str])

        for eq in equations:
            expr = sum(eq[var] * variables[var] for var in var_names if var in eq)
            solver.add(expr == eq["constant"])

        if solver.check() == z3.sat:
            model = solver.model()
            num_presses = 0
            for var_name in sorted(var_names):
                num_presses += model[variables[var_name]].as_long()
            part_two_answer += num_presses
        else:
            print("No solution found")

    return part_one_answer, part_two_answer


print(f"Sample: {solve('../input/2025/input10-sample')}")
print(f"Answer: {solve('../input/2025/input10')}")
