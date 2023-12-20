use std::collections::{HashMap, HashSet};

use crate::util::load;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseType {
    LOW,
    HIGH,
}

#[derive(Debug)]
struct Pulse<'a> {
    pt: PulseType,
    src: &'a str,
    dst: &'a str,
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[derive(Debug)]
struct Module {
    name: String,
    outputs: Vec<String>,
    mt: ModuleType,
}

impl Module {
    fn pulse(&self, state: &mut State, pulse: Pulse) -> Vec<Pulse> {
        let out = match self.mt {
            ModuleType::FlipFlop => match pulse.pt {
                PulseType::LOW => {
                    if state.flip_state(&self.name) {
                        Some(PulseType::HIGH)
                    } else {
                        Some(PulseType::LOW)
                    }
                }
                PulseType::HIGH => None,
            },
            ModuleType::Conjunction => {
                state.set_state(pulse.src, pulse.pt == PulseType::HIGH);
                let all_high = state.are_inputs_all_high(&self.name);
                if all_high {
                    Some(PulseType::LOW)
                } else {
                    Some(PulseType::HIGH)
                }
            }
            ModuleType::Broadcaster => Some(pulse.pt),
        };
        match out {
            Some(pt) => self
                .outputs
                .iter()
                .map(|o| Pulse {
                    pt,
                    src: self.name.as_str(),
                    dst: o,
                })
                .collect(),
            None => vec![],
        }
    }
}

#[derive(Debug)]
struct State {
    s: HashMap<String, u64>,
}

impl State {
    fn bitmask(&self, name: &str) -> u64 {
        let id = self.ids.get(name).unwrap();
        1 << id
    }

    fn get_state(&self, name: &str) -> bool {
        self.state & self.bitmask(name) != 0
    }

    fn set_state(&mut self, name: &str, high: bool) {
        if high {
            self.state |= self.bitmask(name);
        } else {
            self.state &= !self.bitmask(name);
        }
    }

    fn flip_state(&mut self, name: &str) -> bool {
        let bitmask = self.bitmask(name);
        self.state ^= bitmask;
        self.state & bitmask != 0
    }

    fn are_inputs_all_high(&self, name: &str) -> bool {
        let inputs = self.inputs.get(name).unwrap();
        inputs.iter().all(|name| self.get_state(name))
    }
}

#[derive(Debug)]
struct Configuration {
    modules: HashMap<String, Module>,
    ids: HashMap<String, usize>,
    inputs: HashMap<String, HashSet<String>>,
}

impl Configuration {
    fn load(file: &str) -> Self {
        fn name(token: &str) -> (String, ModuleType) {
            match token.chars().next().unwrap() {
                '%' => (token[1..].to_string(), ModuleType::FlipFlop),
                '&' => (token[1..].to_string(), ModuleType::Conjunction),
                _ => (token[0..].to_string(), ModuleType::Broadcaster),
            }
        }
        fn outputs(token: &str) -> Vec<String> {
            token.split(", ").map(|s| s.to_string()).collect()
        }
        let mut state_id: usize = 0;
        let mut config = Configuration {
            modules: HashMap::new(),
            ids: HashMap::new(),
            inputs: HashMap::new(),
        };
        for line in load::<String>(file) {
            let tokens = line.split(" -> ").collect::<Vec<_>>();
            let (name, mt) = name(tokens[0]);
            let outputs = outputs(tokens[1]);
            // set states ID
            config.add_state_id(&mut state_id, &name);
            // build reverse mapping
            outputs.iter().for_each(|o| {
                config
                    .inputs
                    .entry(o.to_string())
                    .or_insert_with(HashSet::new)
                    .insert(name.to_string());
            });
            // create module
            config
                .modules
                .insert(name.to_string(), Module { name, outputs, mt });
        }
        config
    }

    fn init_state(&self) -> State {
        let mut s = HashMap::new();
        self.modules.iter().for_each(|(name, _)| {
            s.insert(name.to_string(), 0);
        });
        State { s }
    }

    fn add_state_id(&mut self, id: &mut usize, name: &str) {
        if !self.ids.contains_key(name) {
            self.ids.insert(name.to_string(), *id);
            *id += 1;
        }
    }

    fn pulse(&self, state: &mut State, pulse: Pulse) -> Vec<Pulse> {
        match self.modules.get(pulse.dst) {
            None => vec![],
            Some(m) => m.pulse( state, pulse),
        }
    }

    fn pulse_until_steady(&self, pulse: Pulse) {
        let mut state = self.init_state();
        let mut pulses = vec![pulse];
        loop {
            println!("{:?} /// {:?}", state, pulses);
            let mut out = vec![];
            loop {
                match pulses.pop() {
                    None => break,
                    Some(p) => out.append(&mut self.pulse(&mut state, p)),
                }
            }
            if out.is_empty() {
                break;
            }
            pulses.append(&mut out);
        }
    }
}

pub fn part1() -> usize {
    let config = Configuration::load("data/test.txt");
    config.pulse_until_steady(Pulse {
        pt: PulseType::LOW,
        src: "button",
        dst: "broadcaster",
    });
    // FIXME call this 1000 times
    0
}

pub fn part2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
