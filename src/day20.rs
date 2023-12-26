use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

use crate::util::load;

fn set_bit(state: u64, bit_mask: u64, value: bool) -> u64 {
    match value {
        true => state | bit_mask,
        false => state & !bit_mask,
    }
}

#[derive(Debug)]
struct Pulse {
    src_mask: u64,
    dst_mask: u64,
    high: bool,
}

impl Pulse {
    fn new(src_mask: u64, dst_mask: u64, high: bool) -> Self {
        Pulse {
            src_mask,
            dst_mask,
            high,
        }
    }
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} --[{}]--> {}",
            self.src_mask,
            if self.high { 'H' } else { 'L' },
            self.dst_mask,
        )
    }
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(u64, Option<usize>),
    Broadcaster,
    Rx(usize),
}

impl Display for ModuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleType::FlipFlop(state) => write!(f, "FF({})", if *state { "on" } else { "off" }),
            ModuleType::Conjunction(state, _) => write!(f, "CJ({})", state),
            ModuleType::Broadcaster => write!(f, "BC"),
            ModuleType::Rx(cnt) => write!(f, "RX({})", cnt),
        }
    }
}

#[derive(Debug)]
struct Module {
    name: String,
    id_mask: u64,
    mt: ModuleType,
    src_mask: u64,
    dst: Vec<u64>,
}

type Modules = HashMap<u64, Module>;

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "* {} = {}", self.name, self.mt)
    }
}

impl Module {
    fn receive(&mut self, pulse: &Pulse, it: usize) -> Vec<Pulse> {
        // println!(
        //     "\t{:?}({}) (src={}, dst={})",
        //     self.name, self.id_mask, pulse.src_mask, pulse.dst_mask
        // );
        let high = match self.mt {
            ModuleType::FlipFlop(state) => {
                if pulse.high {
                    // println!("\t`-> with state {}", state);
                    None
                } else {
                    // println!("\t`-> with state {} -> {}", state, !state);
                    let new_state = !state;
                    self.mt = ModuleType::FlipFlop(new_state);
                    Some(new_state)
                }
            }
            ModuleType::Conjunction(state, i) => {
                let new_state = set_bit(state, pulse.src_mask, pulse.high);
                // println!("\t`-> with state {} -> {}", state, new_state);
                if new_state & self.src_mask == self.src_mask {
                    match i {
                        None => self.mt = ModuleType::Conjunction(new_state, Some(it)),
                        _ => self.mt = ModuleType::Conjunction(new_state, i),
                    }
                    Some(false)
                } else {
                    self.mt = ModuleType::Conjunction(new_state, i);
                    Some(true)
                }
            }
            ModuleType::Broadcaster => Some(pulse.high),
            ModuleType::Rx(cnt) => {
                let c = if pulse.high { cnt } else { cnt + 1 };
                self.mt = ModuleType::Rx(c);
                None
            }
        };
        high.map(|h| {
            self.dst
                .iter()
                .map(|d| Pulse::new(self.id_mask, *d, h))
                .collect_vec()
        })
        .unwrap_or(vec![])
    }
}

fn input(file: &str) -> (u64, u64, Modules) {
    let mods = load::<String>(file)
        .into_iter()
        .map(|l| l.split(" -> ").map(|s| s.to_string()).collect_vec())
        .map(|tokens| {
            let (name, mt) = match tokens[0].chars().next().unwrap() {
                '%' => (tokens[0][1..].to_string(), ModuleType::FlipFlop(false)),
                '&' => (tokens[0][1..].to_string(), ModuleType::Conjunction(0, None)),
                _ => (tokens[0].to_string(), ModuleType::Broadcaster),
            };
            (name, mt, tokens[1].to_string())
        })
        .collect_vec();
    // map names to bits
    let mut max_bit = 0;
    let mut name2bit = mods
        .iter()
        .enumerate()
        .map(|(i, (name, _, _))| {
            let b = 1u64 << i;
            max_bit = max_bit.max(b);
            (name.to_string(), b)
        })
        .collect::<HashMap<_, _>>();
    let mut dst2src: HashMap<u64, u64> = HashMap::new();
    // create modules
    let mut rx = None;
    let mut modules: HashMap<_, _> = mods
        .into_iter()
        .map(|(name, mt, dst_lst)| {
            let id_mask = *name2bit.get(&name).unwrap();
            let dst = dst_lst
                .split(", ")
                .map(|n| {
                    // let dst_mask = *name2bit.get(&n.to_string()).unwrap_or(&(max_bit << 1));
                    let dst_mask = name2bit.entry(n.to_string()).or_insert_with(|| {
                        max_bit <<= 1;
                        rx = Some(max_bit);
                        max_bit
                    });
                    dst2src
                        .entry(*dst_mask)
                        .and_modify(|d| *d |= id_mask)
                        .or_insert(id_mask);
                    *dst_mask
                })
                .collect_vec();
            (
                id_mask,
                Module {
                    name,
                    id_mask,
                    mt,
                    src_mask: 0,
                    dst,
                },
            )
        })
        .collect();
    // Add Rx
    rx.iter().for_each(|id| {
        modules.insert(
            *id,
            Module {
                name: "rx".to_string(),
                id_mask: *id,
                mt: ModuleType::Rx(0),
                src_mask: 0,
                dst: vec![],
            },
        );
    });
    // init sources
    for m in modules.values_mut() {
        dst2src
            .get(&m.id_mask)
            .iter()
            .for_each(|d| m.src_mask = **d);
    }
    (
        *name2bit.get("broadcaster").unwrap(),
        rx.unwrap_or(0),
        modules,
    )
}

fn push_button(mods: &mut Modules, broadcaster: u64, it: usize) -> (usize, usize) {
    let (mut lo, mut hi) = (0, 0);
    let initial = Pulse::new(9999, broadcaster, false);
    let mut pulses = vec![initial];
    loop {
        if pulses.is_empty() {
            break;
        }
        pulses
            .iter()
            .for_each(|p| if p.high { hi += 1 } else { lo += 1 });
        let mut new_pulses = pulses
            .iter()
            .flat_map(|p| {
                // println!("{}", p);
                match mods.get_mut(&p.dst_mask) {
                    Some(m) => m.receive(&p, it),
                    None => vec![],
                }
            })
            .collect_vec();
        // println!("\t{:?}", new_pulses);
        pulses.clear();
        pulses.append(&mut new_pulses);
    }
    (lo, hi)
}

fn repeat_until_all_low(broadcaster: u64, mut modules: Modules) -> (usize, usize, usize) {
    let (mut lo, mut hi) = (0, 0);
    let mut it = 0;
    loop {
        it += 1;
        let (nlo, nhi) = push_button(&mut modules, broadcaster, it);
        lo += nlo;
        hi += nhi;
        let all_low = modules.values().all(|m| match m.mt {
            ModuleType::Broadcaster => true,
            ModuleType::FlipFlop(state) => state == false,
            ModuleType::Conjunction(state, _) => state == 0,
            ModuleType::Rx(_) => true,
        });
        if all_low {
            break;
        }
        if it == 1000 {
            break;
        }
    }
    (it, lo, hi)
}

fn repeat_until_rx_low(broadcaster: u64, mut modules: Modules) -> usize {
    let nodes = modules
        .values()
        .filter(|m| m.name == "bl" || m.name == "mr" || m.name == "pv" || m.name == "vv")
        .map(|m| m.id_mask)
        .collect_vec();
    let mut it = 0;
    let its = loop {
        it += 1;
        let _ = push_button(&mut modules, broadcaster, it);
        let (all_low, its) = nodes.iter().map(|id| modules.get(id).unwrap()).fold(
            (true, vec![]),
            |(low, mut v), m| {
                if let ModuleType::Conjunction(_, Some(i)) = m.mt {
                    v.push(i);
                    (low & true, v)
                } else {
                    (false, v)
                }
            },
        );
        if all_low {
            break its;
        }
    };
    println!("Stopped after {} iterations {:?}...", it, its);
    its.into_iter().product()
}

pub fn part1() -> usize {
    let (bc, _, modules) = input("data/day20.txt");
    let (it, lo, hi) = repeat_until_all_low(bc, modules);
    println!(
        "Stopped after {} iterations with (lo,hi) = ({},{})",
        it, lo, hi
    );
    (1000 / it).pow(2) * (lo * hi)
}

pub fn part2() -> usize {
    let (bc, _, modules) = input("data/day20.txt");
    repeat_until_rx_low(bc, modules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 777666211);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 243081086866483);
    }
}
