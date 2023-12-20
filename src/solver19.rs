use aoc::common::split_string_to_u64;
use std::collections::HashMap;

#[derive(Debug)]
struct Part {
    // Index with 0 for x, 1 for m, 2 for a, 3 for s.
    xmas: Vec<u64>,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

#[derive(Debug)]
enum Rule {
    Accept,
    Reject,
    ToWorkflow(String),
    ComparisonLessThanToWorkflow(usize, u64, String),
    ComparisonGreaterThanToWorkflow(usize, u64, String),
}

const MIN_XMAS: u64 = 1;
const MAX_XMAS: u64 = 4000;

fn xmas_to_index(xmas: char) -> usize {
    match xmas {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!("ERROR!"),
    }
}

pub fn solve19(input: &[String]) -> (i128, i128) {
    let mut reached_blank_line = false;
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = vec![];

    for line in input {
        if line.is_empty() {
            reached_blank_line = true;
        } else if reached_blank_line {
            let line_1: Vec<&str> = line[1..line.len() - 1].split(',').collect();
            let part = Part {
                xmas: vec![
                    split_string_to_u64(line_1[0], '=', 1),
                    split_string_to_u64(line_1[1], '=', 1),
                    split_string_to_u64(line_1[2], '=', 1),
                    split_string_to_u64(line_1[3], '=', 1),
                ],
            };
            parts.push(part);
        } else {
            // TODO: Use regex.
            let line_1: Vec<&str> = line[0..line.len() - 1].split('{').collect();
            let name = line_1[0];
            let line_2: Vec<&str> = line_1[1].split(',').collect();
            let mut rules: Vec<Rule> = vec![];
            for r in line_2 {
                if r == "A" {
                    rules.push(Rule::Accept);
                } else if r == "R" {
                    rules.push(Rule::Reject);
                } else {
                    let r2: Vec<&str> = r.split(':').collect();
                    if r2.len() == 1 {
                        rules.push(Rule::ToWorkflow(r2[0].to_string()));
                    } else {
                        let r3: Vec<&str> = r2[0].split('<').collect();
                        if r3.len() == 2 {
                            let num = r3[1].parse::<u64>().unwrap();
                            let rule = Rule::ComparisonLessThanToWorkflow(
                                xmas_to_index(r3[0].chars().next().unwrap()),
                                num,
                                r2[1].to_string(),
                            );
                            rules.push(rule);
                        } else {
                            let r4: Vec<&str> = r2[0].split('>').collect();
                            if r4.len() == 2 {
                                let num = r4[1].parse::<u64>().unwrap();
                                let rule = Rule::ComparisonGreaterThanToWorkflow(
                                    xmas_to_index(r4[0].chars().next().unwrap()),
                                    num,
                                    r2[1].to_string(),
                                );
                                rules.push(rule);
                            } else {
                                panic!("Didn't decode this rule: {r}");
                            }
                        }
                    }
                }
            }
            workflows.insert(name.to_string(), Workflow { rules });
        }

        // Simplifies processing.
        workflows.insert(
            "A".to_string(),
            Workflow {
                rules: vec![Rule::Accept],
            },
        );
        workflows.insert(
            "R".to_string(),
            Workflow {
                rules: vec![Rule::Reject],
            },
        );
    }

    (
        solve_part_1(&workflows, &parts) as i128,
        solve_part_2(&workflows) as i128,
    )
}

// For each part, start at the workflow called "in". Follow that workflow until it terminates on
// an Accept or Reject instruction.
// Sum the x, m, a and s values for all accepted parts.
fn solve_part_1(workflows: &HashMap<String, Workflow>, parts: &Vec<Part>) -> u64 {
    let mut sum: u64 = 0;
    for part in parts {
        let mut done = false;
        let mut workflow = &workflows["in"];

        while !done {
            for rule in &workflow.rules {
                match rule {
                    Rule::Accept => {
                        sum += part.xmas.iter().sum::<u64>();
                        done = true;
                        break;
                    }
                    Rule::Reject => {
                        done = true;
                        break;
                    }
                    Rule::ToWorkflow(new_workflow_name) => {
                        workflow = &workflows[new_workflow_name];
                        break;
                    }
                    Rule::ComparisonLessThanToWorkflow(comp, amount, new_workflow_name) => {
                        if part.xmas[*comp] < *amount {
                            workflow = &workflows[new_workflow_name];
                            break;
                        }
                    }
                    Rule::ComparisonGreaterThanToWorkflow(comp, amount, new_workflow_name) => {
                        if part.xmas[*comp] > *amount {
                            workflow = &workflows[new_workflow_name];
                            break;
                        }
                    }
                }
            }
        }
    }

    sum
}

#[derive(Clone, Copy, Debug)]
struct Range<'a> {
    next_workflow: &'a str,
    // Index with 0 for x, 1 for m, 2 for a, 3 for s.
    min_xmas: [u64; 4],
    max_xmas: [u64; 4],
}

fn solve_part_2(workflows: &HashMap<String, Workflow>) -> u64 {
    let mut ranges: Vec<Range> = vec![];
    let mut accept_ranges: Vec<Range> = vec![];
    let range = Range {
        next_workflow: "in",
        min_xmas: [MIN_XMAS; 4],
        max_xmas: [MAX_XMAS; 4],
    };
    ranges.push(range);

    while let Some(mut range) = ranges.pop() {
        let workflow = &workflows[range.next_workflow];
        let rules = &workflow.rules;
        for rule in rules {
            match rule {
                Rule::Accept => {
                    accept_ranges.push(range);
                }
                Rule::Reject => {
                    // Discard this range.
                }
                Rule::ToWorkflow(new_workflow_name) => {
                    let mut new_range = range;
                    new_range.next_workflow = new_workflow_name;
                    ranges.push(new_range);
                }
                Rule::ComparisonLessThanToWorkflow(comp, amount, new_workflow_name) => {
                    let mut new_range = range;
                    let i = *comp;

                    if new_range.min_xmas[i] < *amount && new_workflow_name != "R" {
                        new_range.min_xmas[i] = std::cmp::max(MIN_XMAS, new_range.min_xmas[i]);
                        new_range.max_xmas[i] = std::cmp::min(*amount - 1, new_range.max_xmas[i]);
                        if new_workflow_name == "A" {
                            accept_ranges.push(new_range);
                        } else {
                            new_range.next_workflow = new_workflow_name;
                            ranges.push(new_range);
                        }
                    }
                    // Also need to update the range that we'll process the next rule for.
                    range.min_xmas[i] = std::cmp::max(*amount, range.min_xmas[i]);
                }
                Rule::ComparisonGreaterThanToWorkflow(comp, amount, new_workflow_name) => {
                    let mut new_range = range;
                    let i = *comp;

                    if new_range.max_xmas[i] > *amount && new_workflow_name != "R" {
                        new_range.max_xmas[i] = std::cmp::min(MAX_XMAS, new_range.max_xmas[i]);
                        new_range.min_xmas[i] = std::cmp::max(*amount + 1, new_range.min_xmas[i]);
                        if new_workflow_name == "A" {
                            accept_ranges.push(new_range);
                        } else {
                            new_range.next_workflow = new_workflow_name;
                            ranges.push(new_range);
                        }
                    }
                    range.max_xmas[i] = std::cmp::min(*amount, range.max_xmas[i]);
                }
            }
        }
    }

    let mut start_of: u64 = MIN_XMAS;
    let mut prev_ranges: Vec<usize> = vec![];
    let mut x_ranges: Vec<(u64, Vec<usize>)> = vec![];

    for i in MIN_XMAS..=MAX_XMAS {
        let mut ranges: Vec<usize> = vec![];
        for (range_id, range) in accept_ranges.iter().enumerate() {
            if i >= range.min_xmas[0] && i <= range.max_xmas[0] {
                ranges.push(range_id);
            }
        }
        if i != 1 && ranges != prev_ranges || i == MAX_XMAS && !prev_ranges.is_empty() {
            let r_new = if i == MAX_XMAS {
                (1 + i - start_of, prev_ranges.clone())
            } else {
                (i - start_of, prev_ranges.clone())
            };
            x_ranges.push(r_new);
            start_of = i;
        }
        if ranges.is_empty() {
            start_of += 1;
        }
        prev_ranges = ranges;
    }

    let mut m_ranges: Vec<(u64, Vec<usize>)> = vec![];
    for (ways, range_ids) in x_ranges {
        let mut start_of: u64 = MIN_XMAS;
        let mut prev_ranges: Vec<usize> = vec![];
        for i in MIN_XMAS..=MAX_XMAS {
            let mut ranges: Vec<usize> = vec![];
            for range_id in &range_ids {
                let range = &accept_ranges[*range_id];
                if i >= range.min_xmas[1] && i <= range.max_xmas[1] {
                    ranges.push(*range_id);
                }
            }
            if i != MIN_XMAS && (ranges != prev_ranges) || i == MAX_XMAS && !prev_ranges.is_empty()
            {
                let r_new = if i == MAX_XMAS {
                    (ways * (1 + i - start_of), prev_ranges.clone())
                } else {
                    (ways * (i - start_of), prev_ranges.clone())
                };
                m_ranges.push(r_new);
                start_of = i;
            }
            if ranges.is_empty() {
                start_of += 1;
            }
            prev_ranges = ranges;
        }
    }

    let mut a_ranges: Vec<(u64, Vec<usize>)> = vec![];
    for (ways, range_ids) in m_ranges {
        let mut start_of: u64 = MIN_XMAS;
        let mut prev_ranges: Vec<usize> = vec![];
        for i in MIN_XMAS..=MAX_XMAS {
            let mut ranges: Vec<usize> = vec![];
            for range_id in &range_ids {
                let range = &accept_ranges[*range_id];
                if i >= range.min_xmas[2] && i <= range.max_xmas[2] {
                    ranges.push(*range_id);
                }
            }
            if i != MIN_XMAS && (ranges != prev_ranges) || i == MAX_XMAS && !prev_ranges.is_empty()
            {
                let r_new = if i == MAX_XMAS {
                    (ways * (1 + i - start_of), prev_ranges.clone())
                } else {
                    (ways * (i - start_of), prev_ranges.clone())
                };
                a_ranges.push(r_new);
                start_of = i;
            }
            if ranges.is_empty() {
                start_of += 1;
            }
            prev_ranges = ranges;
        }
    }

    let mut s_ranges: Vec<u64> = vec![];
    for (ways, range_ids) in a_ranges {
        let mut start_of: u64 = MIN_XMAS;
        let mut prev_ranges: Vec<usize> = vec![];
        for i in MIN_XMAS..=MAX_XMAS {
            let mut ranges: Vec<usize> = vec![];
            for range_id in &range_ids {
                let range = &accept_ranges[*range_id];
                if i >= range.min_xmas[3] && i <= range.max_xmas[3] {
                    ranges.push(*range_id);
                }
            }
            if i != MIN_XMAS && (ranges != prev_ranges) || i == MAX_XMAS && !prev_ranges.is_empty()
            {
                let r_new = if i == MAX_XMAS {
                    ways * (1 + i - start_of)
                } else {
                    ways * (i - start_of)
                };
                s_ranges.push(r_new);
                start_of = i;
            }
            if ranges.is_empty() {
                start_of += 1;
            }
            prev_ranges = ranges;
        }
    }

    let mut total = 0;
    for ways in s_ranges {
        total += ways;
    }

    total
}
