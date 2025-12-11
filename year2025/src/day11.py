def read_input(input_file: str) -> dict[str, list[str]]:
    with open(input_file) as f:
        rows = [line.rstrip().split(":") for line in f]

    next_nodes: dict[str, list[str]] = {}
    for row in rows:
        next_nodes[row[0]] = row[1].strip().split()

    # Build the reverse lookup (which nodes can a node be reached from).
    prev_nodes: dict[str, list[str]] = {}
    for prev_node, this_nodes in next_nodes.items():
        for this_node in this_nodes:
            if this_node not in prev_nodes:
                prev_nodes[this_node] = [prev_node]
            else:
                prev_nodes[this_node].append(prev_node)

    return prev_nodes


def solve(prev_nodes: dict[str, list[str]], source: str) -> int:
    # Start at "out" and count every path to "you". At each step, track only the tip of each path and the
    # number of ways to reach that tip so far.
    # Include whether "dac" and "fft" have been satisfied in the tip node name. For part one, just pretend
    # they have been visited already.
    current_paths: dict[str, int] = {}
    if source == "svr":
        current_paths["outnn"] = 1
    else:
        current_paths["outyy"] = 1

    satisfied_source = f"{source}yy"

    while satisfied_source not in current_paths or len(current_paths) > 1:
        new_current_paths: dict[str, int] = {}
        for tip_node, num_ways in current_paths.items():
            plain_tip_node = tip_node[0:3]
            next_tip_nodes = prev_nodes.get(plain_tip_node, [])

            if tip_node == satisfied_source:
                new_current_paths[satisfied_source] = new_current_paths.get(satisfied_source, 0) + num_ways
            elif not next_tip_nodes:
                # Dead end. Drop this path.
                pass
            else:
                for next_tip_node in next_tip_nodes:
                    if next_tip_node == "dac":
                        next_tip_node_nn = f"{next_tip_node}y{tip_node[4]}"
                    elif next_tip_node == "fft":
                        next_tip_node_nn = f"{next_tip_node}{tip_node[3]}y"
                    else:
                        next_tip_node_nn = f"{next_tip_node}{tip_node[3]}{tip_node[4]}"
                    new_current_paths[next_tip_node_nn] = (
                        new_current_paths.get(next_tip_node_nn, 0) + num_ways
                    )
        current_paths = new_current_paths

    return current_paths.get(satisfied_source, 0)


print(
    f"Sample: ({solve(read_input('../input/2025/input11-sample'), 'you')}, "
    f"{solve(read_input('../input/2025/input11-sample-2'), 'svr')})"
)
print(
    f"Answer: ({solve(read_input('../input/2025/input11'), 'you')}, "
    f"{solve(read_input('../input/2025/input11'), 'svr')})"
)
