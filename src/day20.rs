use std::collections::{HashMap, VecDeque};
use std::ops::Not;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleType {
    FlipFlop(bool),
    Conjunction,
    Brodcaster,
}

impl FromStr for ModuleType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "%" => ModuleType::FlipFlop(false),
            "&" => ModuleType::Conjunction,
            "b" => ModuleType::Brodcaster,
            _ => Err(())?,
        })
    }
}

pub fn count_low_and_high_pulse_combinations(logic_flow: &str) -> u64 {
    let mut conjunction_modules: Vec<(&str, Vec<&str>, Vec<bool>)> = vec![];
    let mut logic_map: HashMap<_, _> = logic_flow
        .lines()
        .filter_map(|flow| flow.split("->").map(str::trim).collect_tuple())
        .filter_map(|(module, output_list)| {
            let res = (
                if module == "broadcaster" {
                    module
                } else {
                    &module[1..]
                },
                (
                    module[..1].parse::<ModuleType>().ok()?,
                    output_list.split(',').map(str::trim).collect_vec(),
                ),
            );
            if res.1 .0 == ModuleType::Conjunction {
                conjunction_modules.push((res.0, vec![], vec![]));
            }
            Some(res)
        })
        .collect();

    for (&k, (_, recievers)) in logic_map.iter() {
        for reciever in recievers.iter().copied() {
            let Some((_, list, in_state)) = conjunction_modules
                .iter_mut()
                .find(|(x, _, _)| &reciever == x)
            else {
                continue;
            };

            list.push(k);
            in_state.push(false);
        }
    }

    let mut low_pulse = 0;
    let mut high_pulse = 0;

    for _ in 0..1000 {
        let mut frontier = VecDeque::from([(false, "broadcaster", "button")]);

        while let Some((signal, module, sender)) = frontier.pop_front() {
            if signal {
                high_pulse += 1;
            } else {
                low_pulse += 1;
            }
            let Some((module_type, recievers)) = logic_map.get_mut(module) else {
                continue;
            };
            match module_type {
                ModuleType::FlipFlop(x) if !signal => {
                    *x ^= true;
                    frontier.extend(recievers.iter().copied().map(|y| (*x, y, module)))
                }
                ModuleType::Conjunction => {
                    let (_, senders, mem) = conjunction_modules
                        .iter_mut()
                        .find(|(k, _, _)| *k == module)
                        .unwrap();

                    let id = senders.iter().position(|&k| k == sender).unwrap();
                    mem[id] = signal;
                    let next_signal = mem.iter().copied().any(bool::not);
                    frontier.extend(recievers.iter().copied().map(|x| (next_signal, x, module)));
                }
                ModuleType::Brodcaster => {
                    frontier.extend(recievers.iter().copied().map(|k| (false, k, "broadcaster")));
                }
                _ => (),
            }
        }
    }
    low_pulse * high_pulse
}

pub fn get_to_rx(logic_flow: &str) -> u64 {
    let mut conjunction_modules: Vec<(&str, Vec<&str>, Vec<bool>)> = vec![];
    let mut logic_map: HashMap<_, _> = logic_flow
        .lines()
        .filter_map(|flow| flow.split("->").map(str::trim).collect_tuple())
        .filter_map(|(module, output_list)| {
            let res = (
                if module == "broadcaster" {
                    module
                } else {
                    &module[1..]
                },
                (
                    module[..1].parse::<ModuleType>().ok()?,
                    output_list.split(',').map(str::trim).collect_vec(),
                ),
            );
            if res.1 .0 == ModuleType::Conjunction {
                conjunction_modules.push((res.0, vec![], vec![]));
            }
            Some(res)
        })
        .collect();

    for (&k, (_, recievers)) in logic_map.iter() {
        for reciever in recievers.iter().copied() {
            let Some((_, list, in_state)) = conjunction_modules
                .iter_mut()
                .find(|(x, _, _)| &reciever == x)
            else {
                continue;
            };

            list.push(k);
            in_state.push(false);
        }
    }

    let mut count = 0;
    loop {
        let mut frontier = VecDeque::from([(false, "broadcaster", "button")]);
        count += 1;

        while let Some((signal, module, sender)) = frontier.pop_front() {
            if module == "rx" && !signal {
                return count;
            }
            let Some((module_type, recievers)) = logic_map.get_mut(module) else {
                continue;
            };
            match module_type {
                ModuleType::FlipFlop(x) if !signal => {
                    *x ^= true;
                    frontier.extend(recievers.iter().copied().map(|y| (*x, y, module)))
                }
                ModuleType::Conjunction => {
                    let (_, senders, mem) = conjunction_modules
                        .iter_mut()
                        .find(|(k, _, _)| *k == module)
                        .unwrap();

                    let id = senders.iter().position(|&k| k == sender).unwrap();
                    mem[id] = signal;
                    let next_signal = mem.iter().copied().any(bool::not);
                    frontier.extend(recievers.iter().copied().map(|x| (next_signal, x, module)));
                }
                ModuleType::Brodcaster => {
                    frontier.extend(recievers.iter().copied().map(|k| (false, k, "broadcaster")));
                }
                _ => (),
            }
        }
    }
}
