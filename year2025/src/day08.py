import heapq
import math
from pprint import pprint

def in_same_set(id1: int, id2: int, circuits: list[set[int]]) -> bool:
    for circuit in circuits:
        if id1 in circuit and id2 in circuit:
            return True
    return False

def solve(input_file: str, is_sample) -> tuple[int, int]:
    # Points are zero indexed. Map from distance between them to a pair of point indices.
    distance_between_points: dict[float, (int, int)] = {}

    with open(input_file) as f:
        rows = [line.rstrip().split(",") for line in f]

    for i in range(len(rows)):
        for j in range(len(rows)):
            if j > i:
                x1, y1, z1 = map(int, rows[i])
                x2, y2, z2 = map(int, rows[j])
                distance = ((x1 - x2) ** 2 + (y1 - y2) ** 2 + (z1 - z2) ** 2) ** 0.5
                distance_between_points[distance] = (i, j)

    num_required_for_part_one = 10 if is_sample else 1000
    # shortest_distances = heapq.nsmallest(num_required_for_part_one, distance_between_points.items(), key=lambda x: x[0])
    shortest_distances = sorted(distance_between_points.items())

    circuits: list[set[int]] = []  # Each circuit is a list of junction box indices.
    jb_used = [False] * len(rows)

    part_one_answer = -1
    part_two_answer = -1
    num_processed = 0

    for _, (id1, id2) in shortest_distances:
        print(f"PROCESSING PAIR: {id1} {id2}:   {rows[id1]} and {rows[id2]}")
        num_processed += 1 # Rename to num_processed
        if in_same_set(id1, id2, circuits):
            # Skip if both already in the same set.
            continue
        elif not jb_used[id1] and not jb_used[id2]:
            # Create a new circuit with these two junction boxes.
            circuits.append({id1, id2})
        elif not jb_used[id1] and jb_used[id2]:
            # Add junction box 1 to the circuit containing junction box 2.
            for circuit in circuits:
                if id2 in circuit:
                    circuit.add(id1)
                    break
        elif jb_used[id1] and not jb_used[id2]:
            # Add junction box 2 to the circuit containing junction box 1.
            for circuit in circuits:
                if id1 in circuit:
                    circuit.add(id2)
                    break
        else:
            # Both junction boxes are already used in different circuits. Join the two circuits.
            circuit_id_1 = -1
            circuit_id_2 = -1
            for circuit_id in range(len(circuits)):
                if id1 in circuits[circuit_id]:
                    circuit_id_1 = circuit_id
                if id2 in circuits[circuit_id]:
                    circuit_id_2 = circuit_id
            circuit_id_1, circuit_id_2 = sorted([circuit_id_1, circuit_id_2])
            circuits[circuit_id_1] |= circuits.pop(circuit_id_2)
        jb_used[id1] = True
        jb_used[id2] = True
        print(circuits)

        if num_processed == num_required_for_part_one:
            sorted_circuits = sorted(circuits, key=len, reverse=True)
            print(sorted_circuits)
            part_one_answer = math.prod(len(circuit) for circuit in sorted_circuits[:3])

        if len(circuits) == 1 and jb_used.count(False) == 0:
            print("All junction boxes connected.")
            print(rows)
            part_two_answer = int(rows[id1][0]) * int(rows[id2][0])
            break

    return part_one_answer, part_two_answer

# 17152350 too low
print(f"Sample: {solve('../input/2025/input08-sample', True)}")
print(f"Answer: {solve('../input/2025/input08', False)}")
