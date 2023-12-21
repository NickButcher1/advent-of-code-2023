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

fn read_modules_from_input(input: &[String]) -> HashMap<&str, Module> {
    let mut modules: HashMap<&str, Module> = HashMap::new();

    for line in input {
        let (module_type_and_name, targets_str) = line.split_once(" -> ").unwrap();

        let (module_type, module_name) = match &module_type_and_name[..1] {
            "b" => (ModuleType::Broadcast, BROADCASTER),
            "%" => (ModuleType::FlipFlop, &module_type_and_name[1..]),
            "&" => (ModuleType::Conjunction, &module_type_and_name[1..]),
            _ => unreachable!(),
        };

        modules.insert(
            module_name,
            Module {
                module_type,
                targets: targets_str.split(", ").collect(),
                last_incoming: HashMap::new(),
                state: false,
            },
        );
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

    modules
}
pub fn solve20(input: &[String]) -> (i128, i128) {
    let mut modules = read_modules_from_input(input);

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
