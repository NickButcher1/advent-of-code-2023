use num_integer::lcm;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
    Output,
    Rx,
}
#[derive(Debug)]
struct Module<'a> {
    module_type: ModuleType,
    targets: Vec<&'a str>,
    last_incoming: HashMap<&'a str, bool>,
    state: bool,
}

#[derive(Debug)]
struct Pulse<'a> {
    from: &'a str,
    target: &'a str,
    state: bool,
}

const BROADCASTER: &str = "broadcaster";
const PUSH_BUTTON: &str = "pushbutton";
const OUTPUT: &str = "output";

pub fn solve20(input: &[String]) -> (i128, i128) {
    let mut modules: HashMap<&str, Module> = HashMap::new();

    // TODO: I really need to learn Rust regex.
    for line_num in 0..input.len() {
        let line = &input[line_num];
        let line_1: Vec<&str> = line.split(' ').collect();
        let targets_untrimmed = &line_1[2..];

        let mut targets: Vec<&str> = vec![];
        for t in targets_untrimmed {
            let t2 = t.strip_suffix(',');
            if let Some(t3) = t2 {
                targets.push(t3);
            } else {
                targets.push(t);
            }
        }
        let (module_type, module_name) = if line_1[0] == BROADCASTER {
            (ModuleType::Broadcast, BROADCASTER)
        } else {
            let type_and_name: Vec<char> = line_1[0].chars().collect();
            let module_name = &input[line_num][1..type_and_name.len()];
            if type_and_name[0] == '%' {
                (ModuleType::FlipFlop, module_name)
            } else {
                (ModuleType::Conjunction, module_name)
            }
        };

        let module = Module {
            module_type,
            targets,
            last_incoming: HashMap::new(),
            state: false,
        };
        modules.insert(module_name, module);
        // Only required for one of the the samples.
        modules.insert(
            OUTPUT,
            Module {
                module_type: ModuleType::Output,
                targets: vec![],
                last_incoming: HashMap::new(),
                state: false,
            },
        );
        // The input has a target "rx" that isn't defined in the input.
        modules.insert(
            "rx",
            Module {
                module_type: ModuleType::Rx,
                targets: vec![],
                last_incoming: HashMap::new(),
                state: false,
            },
        );
    }

    let mut incoming_mapping: Vec<(&str, &str)> = vec![];
    for (module_name, module) in &modules {
        for target in &module.targets {
            incoming_mapping.push((module_name, target));
        }
    }

    for (from_module_name, to_module_name) in incoming_mapping {
        let to_module = modules.get_mut(to_module_name).unwrap();
        if to_module.module_type == ModuleType::Conjunction {
            to_module.last_incoming.insert(from_module_name, false);
        }
    }

    // For part 1.
    let mut part_1: u64 = 0;
    let mut low_sent: u64 = 0;
    let mut high_sent: u64 = 0;

    // For part 2.
    let mut part_2: u64 = 1;
    let mut first_dl_found = false;
    let mut first_vk_found = false;
    let mut first_ks_found = false;
    let mut first_pm_found = false;

    // Keep pushing the button, then waiting for the queue to clear, until both parts are solved.
    let mut pulse_queue: Vec<Pulse> = vec![];
    for loop_id in 1..100_000_000_000 {
        pulse_queue.push(Pulse {
            from: PUSH_BUTTON,
            target: BROADCASTER,
            state: false,
        });

        while !pulse_queue.is_empty() {
            let pulse = pulse_queue.remove(0);
            if pulse.state {
                high_sent += 1;
            } else {
                low_sent += 1;
            }
            let module = modules.get_mut(&pulse.target).unwrap();

            match module.module_type {
                ModuleType::Broadcast => {
                    for target in &module.targets {
                        pulse_queue.push(Pulse {
                            from: pulse.target,
                            target,
                            state: pulse.state,
                        });
                    }
                }
                ModuleType::FlipFlop => {
                    if !pulse.state {
                        module.state = !module.state;

                        for target in &module.targets {
                            pulse_queue.push(Pulse {
                                from: pulse.target,
                                target,
                                state: module.state,
                            });
                        }
                    }
                }
                ModuleType::Conjunction => {
                    // Super hacky - this relied on inspection of the input, noticing there was an
                    // "rx" terminating output, and then noticing that it was fed from a conjunction
                    // of four other inputs. I guessed that all four would be on a cycle and it was
                    // LCM for when they all coincided in turning on. Fortunately their cycles all
                    // start at offset zero. This obviously won't work with a different input.
                    let last_incoming_state = module.last_incoming.get_mut(pulse.from).unwrap();
                    if pulse.target == "dt" && *last_incoming_state != pulse.state && pulse.state {
                        match pulse.from {
                            "dl" => {
                                if !first_dl_found {
                                    first_dl_found = true;
                                    part_2 = lcm(part_2, loop_id);
                                }
                            }
                            "vk" => {
                                if !first_vk_found {
                                    first_vk_found = true;
                                    part_2 = lcm(part_2, loop_id);
                                }
                            }
                            "ks" => {
                                if !first_ks_found {
                                    first_ks_found = true;
                                    part_2 = lcm(part_2, loop_id);
                                }
                            }
                            "pm" => {
                                if !first_pm_found {
                                    first_pm_found = true;
                                    part_2 = lcm(part_2, loop_id);
                                }
                            }
                            _ => {}
                        }
                        if first_dl_found && first_vk_found && first_ks_found && first_pm_found {
                            return (i128::from(part_1), i128::from(part_2));
                        }
                    }
                    *last_incoming_state = pulse.state;
                    let mut send_state = false;
                    for last_state in module.last_incoming.values() {
                        if !last_state {
                            send_state = true;
                            break;
                        }
                    }
                    for target in &module.targets {
                        pulse_queue.push(Pulse {
                            from: pulse.target,
                            target,
                            state: send_state,
                        });
                    }
                }
                ModuleType::Output | ModuleType::Rx => {}
            }
        }
        if loop_id == 1000 {
            part_1 = low_sent * high_sent;
        }
    }

    unreachable!();
}
