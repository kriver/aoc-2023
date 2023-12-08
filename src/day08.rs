use std::collections::HashMap;

use num::Integer;
use regex::Regex;

use crate::util::load;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Node {
    name: String,
    is_start: bool,
    is_end: bool,
}

impl Node {
    pub fn new(name: &str) -> Self {
        let c = name.chars().skip(2).next().unwrap();
        Self {
            name: name.to_string(),
            is_start: c == 'A',
            is_end: c == 'Z',
        }
    }
}

type Instructions = HashMap<Node, (Node, Node)>;

fn input() -> (Vec<Direction>, Instructions) {
    let re = Regex::new(r"([0-9A-Z]+) = \(([0-9A-Z]+), ([0-9A-Z]+)\)").unwrap();
    let lines = load::<String>("data/day08.txt");
    let dirs = lines[0]
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!("Invalid direction"),
        })
        .collect();
    let instr = lines
        .into_iter()
        .skip(2)
        .fold(HashMap::new(), |mut acc, l| {
            let caps = re.captures(&l).unwrap();
            acc.insert(
                Node::new(caps.get(1).unwrap().as_str()),
                (
                    Node::new(caps.get(2).unwrap().as_str()),
                    Node::new(caps.get(3).unwrap().as_str()),
                ),
            );
            acc
        });
    (dirs, instr)
}

fn go_to_end(dirs: &[Direction], instr: &Instructions, pos: &Node) -> usize {
    let mut p = pos;
    let mut i = 0;
    loop {
        if p.is_end {
            break;
        }
        let dir = &dirs[i % dirs.len()];
        p = match dir {
            Direction::Left => &instr.get(&p).unwrap().0,
            Direction::Right => &instr.get(&p).unwrap().1,
        };
        i += 1;
    }
    i
}

pub fn part1() -> usize {
    let (dirs, instr) = input();
    let pos = &Node::new("AAA");
    go_to_end(&dirs, &instr, &pos)
}

pub fn part2() -> usize {
    let (dirs, instr) = input();
    let starts: Vec<&Node> = instr.keys().filter(|n| n.is_start).collect();
    let ends: Vec<usize> = starts
        .into_iter()
        .map(|p| go_to_end(&dirs, &instr, p))
        .collect();
    ends.into_iter().reduce(|acc, end| acc.lcm(&end)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 16043);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 15726453850399);
    }
}
