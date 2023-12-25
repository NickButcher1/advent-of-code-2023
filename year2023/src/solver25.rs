use std::collections::{HashMap, HashSet};

fn remove_wires(
    comp_names: &HashSet<&str>,
    o_mapping: &HashMap<&str, Vec<&str>>,
    wires: Vec<(&str, &str)>,
) -> u64 {
    let mut mapping = o_mapping.clone();

    for (comp1, comp2) in wires {
        if mapping[comp1].contains(&comp2) {
            mapping.get_mut(comp1).unwrap().retain(|&x| x != comp2);
        }
        if mapping[comp2].contains(&comp1) {
            mapping.get_mut(comp2).unwrap().retain(|&x| x != comp1);
        }
    }

    count_loops(comp_names, &mapping)
}

fn count_loops(comp_names: &HashSet<&str>, mapping: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut loops: Vec<HashSet<&str>> = vec![];
    let mut processed_name: HashSet<&str> = HashSet::new();
    let mut search_names: Vec<&str> = vec!["bcf"]; // bcf for real input
    while processed_name.len() < comp_names.len() {
        let mut from_name = "ERROR";

        if search_names.len() == 0 {
            for name in comp_names {
                if !processed_name.contains(name) {
                    from_name = name;
                    break;
                }
            }
        } else {
            from_name = search_names.remove(0);
        };

        processed_name.insert(from_name);
        let mut found = false;
        let mut found_loop_id = 0;
        for loop_id in 0..loops.len() {
            if loops[loop_id].contains(from_name) {
                found = true;
                found_loop_id = loop_id;
                break;
            }
        }

        if found {
            // Already in a loop. Add all targets if not already.
            for target in &mapping[from_name] {
                loops[found_loop_id].insert(target);
                if !processed_name.contains(target) {
                    processed_name.insert(target);
                    search_names.push(target);
                }
            }
        } else {
            // Are any of the targets in a loop.
            let mut found = false;
            let mut found_loop_id = 0;
            for target in &mapping[from_name] {
                if !found {
                    for loop_id in 0..loops.len() {
                        if loops[loop_id].contains(target) {
                            found = true;
                            found_loop_id = loop_id;
                            break;
                        }
                    }
                }
            }
            if found {
                // Already in a loop. Add all targets if not already.
                for target in &mapping[from_name] {
                    loops[found_loop_id].insert(target);
                    if !processed_name.contains(target) {
                        processed_name.insert(target);
                        search_names.push(target);
                    }
                }
                loops[found_loop_id].insert(from_name);
                if !processed_name.contains(&from_name) {
                    processed_name.insert(from_name);
                }
            } else {
                let mut new_loop: HashSet<&str> = HashSet::new();
                new_loop.insert(from_name);
                for target in &mapping[from_name] {
                    new_loop.insert(target);
                    if !processed_name.contains(target) {
                        processed_name.insert(target);
                        search_names.push(target);
                    }
                }
                loops.push(new_loop);
            }
        }
    }
    if loops.len() == 2 {
        loops[0].len() as u64 * loops[1].len() as u64
    } else {
        0
    }
}

pub fn solve25(input: &[String]) -> (i128, i128) {
    let mut comp_names: HashSet<&str> = HashSet::new();
    let mut mapping: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input {
        let line_1: Vec<&str> = line.split(": ").collect();
        let name_1 = line_1[0];
        let names: Vec<&str> = line_1[1].split(' ').collect();

        comp_names.insert(name_1);
        for name in names {
            comp_names.insert(name);
            mapping.entry(name_1).or_default();
            mapping.entry(name).or_default();
            mapping.get_mut(name_1).unwrap().push(name);
            mapping.get_mut(name).unwrap().push(name_1);
        }
    }

    remove_wires(&comp_names, &mapping, vec![]);
    // These three wires found by visualising the graph (separate code) which makes it very obvious
    // that there are two dense networks connected by these three wires only.
    let part_1 = remove_wires(
        &comp_names,
        &mapping,
        vec![("fjn", "mzb"), ("mhb", "zqg"), ("jlt", "sjr")],
    );
    (part_1 as i128, -1)
}
