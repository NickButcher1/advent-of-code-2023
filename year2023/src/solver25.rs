use fast_paths::InputGraph;
use std::collections::{HashMap, HashSet};

// Find the shortest path between every pair of components, then count every wire on that path. From the puzzle
// description, I guessed that there would be two dense networks of components, bridged by just three wires (the three
// that need to be removed to solve the puzzle.
//
// Prior to this code to automatically find the most common wires, I also solved it by visualising the graph, which
// makes it very obvious that there are two dense networks connected by these three wires only. These are:
// - "fjn", "mzb"
// - "mhb", "zqg
// - "jlt", "sjr"
fn find_most_common_wires(mapping: &HashMap<usize, Vec<usize>>) -> Vec<(usize, usize)> {
    let num_nodes = mapping.len();
    let mut input_graph = InputGraph::new();

    for from_node in 0..num_nodes {
        for target in &mapping[&from_node] {
            input_graph.add_edge(from_node, *target, 1);
        }
    }
    input_graph.freeze();
    let fast_graph = fast_paths::prepare(&input_graph);
    let mut path_calculator = fast_paths::create_calculator(&fast_graph);

    let mut wires: HashMap<(usize, usize), usize> = HashMap::new();
    for from_node in 0..num_nodes {
        for to_node in 0..num_nodes {
            wires.insert((from_node, to_node), 0);
        }
    }
    for from_node in 0..num_nodes {
        for to_node in 0..num_nodes {
            if from_node > to_node {
                let shortest_path = path_calculator
                    .calc_path(&fast_graph, from_node, to_node)
                    .unwrap();
                let nodes = &shortest_path.get_nodes();
                let nodes_len = shortest_path.get_nodes().len();
                for i in 0..nodes_len - 1 {
                    let (idx1, idx2) = if nodes[i] > nodes[i + 1] {
                        (0, 1)
                    } else {
                        (1, 0)
                    };
                    let w = wires
                        .get_mut(&(nodes[i + idx1] as usize, nodes[i + idx2] as usize))
                        .unwrap();
                    *w += 1;
                }
            }
        }
    }

    let mut sorted_wires: Vec<_> = wires.iter().collect();
    sorted_wires.sort_by(|a, b| b.1.cmp(a.1));

    vec![*sorted_wires[0].0, *sorted_wires[1].0, *sorted_wires[2].0]
}

fn remove_wires(mapping: &mut HashMap<usize, Vec<usize>>, wires: Vec<(usize, usize)>) {
    for (comp1, comp2) in wires {
        mapping.get_mut(&comp1).unwrap().retain(|&x| x != comp2);
        mapping.get_mut(&comp2).unwrap().retain(|&x| x != comp1);
    }
}

// Split all the components into loops. We only expect two loops.
fn solve_part_one(mapping: &HashMap<usize, Vec<usize>>) -> u64 {
    let num_nodes = mapping.len();
    let mut loops: Vec<HashSet<usize>> = vec![];
    let mut processed_nodes: HashSet<usize> = HashSet::new();
    let mut search_nodes: Vec<usize> = vec![0];

    while processed_nodes.len() < num_nodes {
        let mut from_node = usize::MAX;

        if search_nodes.is_empty() {
            for node in 0..num_nodes {
                if !processed_nodes.contains(&node) {
                    from_node = node;
                    break;
                }
            }
        } else {
            from_node = search_nodes.remove(0);
        };

        processed_nodes.insert(from_node);
        let mut found = false;
        let mut found_loop_id = 0;
        for (loop_id, my_loop) in loops.iter().enumerate() {
            if my_loop.contains(&from_node) {
                found = true;
                found_loop_id = loop_id;
                break;
            }
        }

        if found {
            // Already in a loop. Add all targets if not already.
            for target in &mapping[&from_node] {
                loops[found_loop_id].insert(*target);
                if !processed_nodes.contains(target) {
                    processed_nodes.insert(*target);
                    search_nodes.push(*target);
                }
            }
        } else {
            // Are any of the targets in a loop.
            let mut found = false;
            let mut found_loop_id = 0;
            for target in &mapping[&from_node] {
                if !found {
                    for (loop_id, my_loop) in loops.iter().enumerate() {
                        if my_loop.contains(target) {
                            found = true;
                            found_loop_id = loop_id;
                            break;
                        }
                    }
                }
            }
            if found {
                // Already in a loop. Add all targets if not already.
                for target in &mapping[&from_node] {
                    loops[found_loop_id].insert(*target);
                    if !processed_nodes.contains(target) {
                        processed_nodes.insert(*target);
                        search_nodes.push(*target);
                    }
                }
                loops[found_loop_id].insert(from_node);
                if !processed_nodes.contains(&from_node) {
                    processed_nodes.insert(from_node);
                }
            } else {
                let mut new_loop: HashSet<usize> = HashSet::new();
                new_loop.insert(from_node);
                for target in &mapping[&from_node] {
                    new_loop.insert(*target);
                    if !processed_nodes.contains(target) {
                        processed_nodes.insert(*target);
                        search_nodes.push(*target);
                    }
                }
                loops.push(new_loop);
            }
        }
    }
    assert_eq!(loops.len(), 2);
    loops[0].len() as u64 * loops[1].len() as u64
}

// Translate node names to integers in order to work with fast_path.
// Return a map of node -> all nodes it connects to.
fn read_nodes_from_input(input: &[String]) -> HashMap<usize, Vec<usize>> {
    let mut next_node_id = 0;
    let mut name_to_id: HashMap<&str, usize> = HashMap::new();
    let mut mapping: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in input {
        let line_1: Vec<&str> = line.split(": ").collect();
        let name_1 = line_1[0];
        let names: Vec<&str> = line_1[1].split(' ').collect();

        if !name_to_id.contains_key(name_1) {
            name_to_id.insert(name_1, next_node_id);
            next_node_id += 1;
        }

        for name in names {
            if !name_to_id.contains_key(name) {
                name_to_id.insert(name, next_node_id);
                next_node_id += 1;
            }
            mapping.entry(name_to_id[name_1]).or_default();
            mapping.entry(name_to_id[name]).or_default();
            mapping
                .get_mut(&name_to_id[name_1])
                .unwrap()
                .push(name_to_id[name]);
            mapping
                .get_mut(&name_to_id[name])
                .unwrap()
                .push(name_to_id[name_1]);
        }
    }
    mapping
}
pub fn solve25(input: &[String]) -> (i128, i128) {
    let mut mapping = read_nodes_from_input(input);
    let most_common_wires = find_most_common_wires(&mapping);
    remove_wires(&mut mapping, most_common_wires);

    (i128::from(solve_part_one(&mapping)), -1)
}
