// Pulses are always processed in the order they are sent. So, if a pulse is sent to modules a, b, and c, and then module a
// processes its pulse and sends more pulses, the pulses sent to modules b and c would have to be handled first.

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
    module_name: &'a str,
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

// impl Module<'_> {
//     fn receive_pulse<'a>(&'a self, pulse_queue: &mut Vec<Pulse>, pulse: &Pulse) {
//         match self.module_type {
//             ModuleType::Broadcast => {
//                 // When it receives a pulse, it sends the same pulse to all of its destination modules.
//                 for target in &self.targets {
//                     pulse_queue.push(Pulse {target, state: pulse.state});
//                 }
//             },
//             ModuleType::FlipFlop => {
//                 // TODO
//             },
//             ModuleType::Conjunction => {
//                 // TODO
//             },
//         }
//         // TODO
//     }
// }

pub fn solve20(input: &[String]) -> (i128, i128) {
    let mut modules: HashMap<&str, Module> = HashMap::new();

    for line_num in 0..input.len() {
        let line = &input[line_num];
        let line_1: Vec<&str> = line.split(' ').collect();
        let targets_untrimmed = &line_1[2..];

        let mut targets: Vec<&str> = vec![];
        for t in targets_untrimmed {
            let t2 = t.strip_suffix(',');
            if t2 == None {
                targets.push(t);
            } else {
                targets.push(t2.unwrap());
            }
        }
        let (module_type, module_name) = if line_1[0] == "broadcaster" {
            (ModuleType::Broadcast, "broadcaster")
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
            module_name,
            module_type,
            targets,
            last_incoming: HashMap::new(),
            state: false,
        };
        // println!("INSERT {module_name}    {module:?}");
        modules.insert(module_name, module);
        modules.insert(
            "output",
            Module {
                module_name: "output",
                module_type: ModuleType::Output,
                targets: vec![],
                last_incoming: HashMap::new(),
                state: false,
            },
        );
        modules.insert(
            "rx",
            Module {
                module_name: "rx",
                module_type: ModuleType::Rx,
                targets: vec![],
                last_incoming: HashMap::new(),
                state: false,
            },
        );
    }

    // Now fill in last_incoming.
    println!("MODULES");
    let mut incoming_mapping: Vec<(&str, &str)> = vec![];
    for (module_name, module) in &modules {
        println!("    {module:?}");

        for target in &module.targets {
            incoming_mapping.push((module_name, target));
        }
    }

    for (from_module_name, to_module_name) in incoming_mapping {
        println!("DBG: {to_module_name}");
        let to_module = modules.get_mut(to_module_name).unwrap();
        if to_module.module_type == ModuleType::Conjunction {
            to_module.last_incoming.insert(from_module_name, false);
        }
    }

    println!("MODULES WITH LAST INCOMING");
    let mut incoming_mapping: Vec<(&str, &str)> = vec![];
    for (module_name, module) in &modules {
        println!("    {module:?}");
    }
    // When you push the button, a single low pulse is sent directly to the broadcaster module.
    let mut low_sent: u64 = 0;
    let mut high_sent: u64 = 0;
    let mut pulse_queue: Vec<Pulse> = vec![];
    let mut rx_count: u64 = 0;
    let mut part_2: u64 = 0;
    let mut first_dl: u64 = 0;
    let mut first_vk: u64 = 0;
    let mut first_ks: u64 = 0;
    let mut first_pm: u64 = 0;

    for loop_id in 1..=100_000_000_000 {
        pulse_queue.push(Pulse {
            from: "pushbutton",
            target: "broadcaster",
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
            // println!("\n{pulse:?} for {module:?}");

            // Receive the pulse.
            match module.module_type {
                ModuleType::Broadcast => {
                    // When it receives a pulse, it sends the same pulse to all of its destination modules.
                    for target in &module.targets {
                        // println!("    BROA-{} push {target} {}", module.state, pulse.state);
                        pulse_queue.push(Pulse {
                            from: pulse.target,
                            target,
                            state: pulse.state,
                        });
                    }
                }
                ModuleType::FlipFlop => {
                    // Flip-flop modules (prefix %) are either on or off; they are initially off.
                    // If a flip-flop module receives a high pulse, it is ignored and nothing happens. However, if a flip-flop module
                    // receives a low pulse, it flips between on and off. If it was off, it turns on and sends a high pulse.
                    // If it was on, it turns off and sends a low pulse.
                    if !pulse.state {
                        module.state = !module.state;

                        for target in &module.targets {
                            // println!(
                            //     "    FLFL-{}-{} push {target} {}",
                            //     module.module_name, module.state, module.state
                            // );
                            pulse_queue.push(Pulse {
                                from: pulse.target,
                                target,
                                state: module.state,
                            });
                        }
                    }
                }
                ModuleType::Conjunction => {
                    // Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules;
                    // they initially default to remembering a low pulse for each input. When a pulse is received, the conjunction module
                    // first updates its memory for that input. Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
                    let x = module.last_incoming.get_mut(pulse.from);
                    let x2 = x.unwrap();
                    let mut trigger = false;
                    if module.module_name == "dt" && *x2 != pulse.state && pulse.state {
                        // println!("    {loop_id}    dt from {} flipped to {}", pulse.from, pulse.state);
                        trigger = true;
                        match pulse.from {
                            "dl" => {
                                if first_dl == 0 {
                                    first_dl = loop_id;
                                    println!(
                                        "DL {first_dl} {first_vk} {first_ks} {first_pm}    {}",
                                        lcm(lcm(lcm(first_dl, first_vk), first_ks), first_pm)
                                    );
                                }
                            }
                            "vk" => {
                                if first_vk == 0 {
                                    first_vk = loop_id;
                                    println!(
                                        "VK {first_dl} {first_vk} {first_ks} {first_pm}    {}",
                                        lcm(lcm(lcm(first_dl, first_vk), first_ks), first_pm)
                                    );
                                }
                            }
                            "ks" => {
                                if first_ks == 0 {
                                    first_ks = loop_id;
                                    println!(
                                        "KS {first_dl} {first_vk} {first_ks} {first_pm}    {}",
                                        lcm(lcm(lcm(first_dl, first_vk), first_ks), first_pm)
                                    );
                                }
                            }
                            "pm" => {
                                if first_pm == 0 {
                                    first_pm = loop_id;
                                    println!(
                                        "PM {first_dl} {first_vk} {first_ks} {first_pm}    {}",
                                        lcm(lcm(lcm(first_dl, first_vk), first_ks), first_pm)
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                    *x2 = pulse.state;
                    let mut send_state = false;
                    // println!("    CONJ TEST last_incoming: {:?}", &module.last_incoming);
                    for (_, last_state) in &module.last_incoming {
                        if !last_state {
                            send_state = true;
                            break;
                        }
                    }
                    // if trigger {
                    //     println!(
                    //         "    CONJ-{}-{} rx {}    last_incoming {:?}",
                    //         module.module_name, module.state, send_state, module.last_incoming
                    //     );
                    // }
                    for target in &module.targets {
                        // println!(
                        //     "    CONJ-{}-{} push {target} {}",
                        //     module.module_name, module.state, send_state
                        // );
                        pulse_queue.push(Pulse {
                            from: pulse.target,
                            target,
                            state: send_state,
                        });
                    }
                }
                ModuleType::Output => {}
                ModuleType::Rx => {
                    if !pulse.state {
                        rx_count += 1;
                        if rx_count == 1 {
                            part_2 = loop_id;
                            println!("PART 2: {loop_id}");
                        }
                    }
                }
            }
        }
        // println!("LOOP {loop_id:04}  HIGH {high_sent}  LOW {low_sent}  MULT {}", low_sent * high_sent);
    }

    (i128::from(low_sent * high_sent), i128::from(part_2))
}
