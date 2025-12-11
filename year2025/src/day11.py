# aaa: you hhh
# you: bbb ccc
# bbb: ddd eee
# ccc: ddd eee fff
# ddd: ggg
# eee: out
# fff: out
# ggg: out
# hhh: ccc fff iii
# iii: out
from pprint import pprint


def solve(input_file: str) -> tuple[int, int]:
    with open(input_file) as f:
        rows = [line.rstrip().split(":") for line in f]

    next_nodes: dict[str, list[str]] = {}
    for row in rows:
        next_nodes[row[0]] = row[1].strip().split()
    print("FORWARD LOOKUP:")
    pprint(next_nodes)

    # Build the reverse lookup (which nodes can a node be reached from).
    prev_nodes: dict[str, list[str]] = {}
    for prev_node, this_nodes in next_nodes.items():
        for this_node in this_nodes:
            if this_node not in prev_nodes:
                prev_nodes[this_node] = [prev_node]
            else:
                prev_nodes[this_node].append(prev_node)

    print("REVERSE LOOKUP:")
    pprint(prev_nodes)

    # Start at 'out' and count every path to 'you'. At each step, track only the tip of each path and the
    # number of ways to reach that tip so far.
    current_paths: dict[str, int] = {"out": 1}
    while "you" not in current_paths or len(current_paths) > 1:
        new_current_paths: dict[str, int] = {}
        for tip_node, num_ways in current_paths.items():
            next_tip_nodes = prev_nodes.get(tip_node, [])
            if tip_node == "you":
                new_current_paths["you"] = new_current_paths.get("you", 0) + num_ways
            elif not next_tip_nodes:
                # Dead end. Drop this path.
                pass
            else:
                for next_tip_node in next_tip_nodes:
                    new_current_paths[next_tip_node] = new_current_paths.get(next_tip_node, 0) + num_ways
        current_paths = new_current_paths
        print("Current paths:")
        pprint(current_paths)

    part_one_answer = current_paths.get("you", 0)

    return part_one_answer, 0


print(f"Sample: {solve('../input/2025/input11-sample')}")
print(f"Answer: {solve('../input/2025/input11')}")
