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
fn find_most_common_wires(
    num_nodes: usize,
    mapping: &HashMap<usize, Vec<usize>>,
) -> Vec<(usize, usize)> {
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
                    if nodes[i] > nodes[i + 1] {
                        let w = wires
                            .get_mut(&(nodes[i] as usize, nodes[i + 1] as usize))
                            .unwrap();
                        *w += 1;
                    } else {
                        let w = wires
                            .get_mut(&(nodes[i + 1] as usize, nodes[i] as usize))
                            .unwrap();
                        *w += 1;
                    }
                }
            }
        }
    }

    let mut sorted_vec: Vec<_> = wires.iter().collect();
    sorted_vec.sort_by(|a, b| b.1.cmp(a.1));

    vec![
        (sorted_vec[0].0 .0, sorted_vec[0].0 .1),
        (sorted_vec[1].0 .0, sorted_vec[1].0 .1),
        (sorted_vec[2].0 .0, sorted_vec[2].0 .1),
    ]
}

fn remove_wires(mapping: &mut HashMap<usize, Vec<usize>>, wires: Vec<(usize, usize)>) {
    for (comp1, comp2) in wires {
        if mapping[&comp1].contains(&comp2) {
            mapping.get_mut(&comp1).unwrap().retain(|&x| x != comp2);
        }
        if mapping[&comp2].contains(&comp1) {
            mapping.get_mut(&comp2).unwrap().retain(|&x| x != comp1);
        }
    }
}

fn solve_part_one(num_nodes: usize, mapping: &HashMap<usize, Vec<usize>>) -> u64 {
    let mut loops: Vec<HashSet<usize>> = vec![];
    let mut processed_name: HashSet<usize> = HashSet::new();
    let mut search_names: Vec<usize> = vec![0];
    while processed_name.len() < num_nodes {
        let mut from_node = usize::MAX;

        if search_names.is_empty() {
            for node in 0..num_nodes {
                if !processed_name.contains(&node) {
                    from_node = node;
                    break;
                }
            }
        } else {
            from_node = search_names.remove(0);
        };

        processed_name.insert(from_node);
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
                if !processed_name.contains(target) {
                    processed_name.insert(*target);
                    search_names.push(*target);
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
                    if !processed_name.contains(target) {
                        processed_name.insert(*target);
                        search_names.push(*target);
                    }
                }
                loops[found_loop_id].insert(from_node);
                if !processed_name.contains(&from_node) {
                    processed_name.insert(from_node);
                }
            } else {
                let mut new_loop: HashSet<usize> = HashSet::new();
                new_loop.insert(from_node);
                for target in &mapping[&from_node] {
                    new_loop.insert(*target);
                    if !processed_name.contains(target) {
                        processed_name.insert(*target);
                        search_names.push(*target);
                    }
                }
                loops.push(new_loop);
            }
        }
    }
    if loops.len() == 2 {
        println!("DONE: {}", loops[0].len() as u64 * loops[1].len() as u64);
        loops[0].len() as u64 * loops[1].len() as u64
    } else {
        panic!("ERROR");
    }
}

pub fn solve25(input: &[String]) -> (i128, i128) {
    // Replace node names with integers.
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
    let most_common_wires = find_most_common_wires(next_node_id, &mapping);
    remove_wires(&mut mapping, most_common_wires);

    (solve_part_one(next_node_id, &mapping) as i128, -1)
}
