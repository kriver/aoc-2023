use std::{collections::HashMap, str::FromStr};

use regex::Regex;

use crate::util::load;

type Part = [u32; 4];

#[derive(Debug)]
enum Destination {
    Accepted,
    Rejected,
    WorkFlow(String),
}

impl FromStr for Destination {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Destination::Accepted),
            "R" => Ok(Destination::Rejected),
            _ => Ok(Destination::WorkFlow(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct Condition {
    category: usize,
    operator: char,
    value: u32,
}

impl Condition {
    fn matches(&self, value: u32) -> bool {
        match self.operator {
            '<' if value < self.value => true,
            '>' if value > self.value => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    dst: Destination,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^((.)([<>])(\d+):)?([ARa-z]+)$").unwrap();
        let capt = re.captures(s).unwrap();
        let condition = match capt.get(1) {
            None => None,
            Some(_) => {
                let category = match capt.get(2).unwrap().as_str() {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    s => unreachable!("unexpected category {}", s),
                };
                let operator = capt.get(3).unwrap().as_str().chars().next().unwrap();
                let value = capt.get(4).unwrap().as_str().parse().unwrap();
                Some(Condition {
                    category,
                    operator,
                    value,
                })
            }
        };
        let dst = capt.get(5).unwrap().as_str().parse().unwrap();
        Ok(Rule { condition, dst })
    }
}

impl Rule {
    fn matches(&self, part: &Part) -> Option<&Destination> {
        match &self.condition {
            None => Some(&self.dst),
            Some(cond) => {
                let value = part[cond.category];
                if cond.matches(value) {
                    Some(&self.dst)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
struct WorkFlow {
    name: String,
    rules: Vec<Rule>,
}

impl FromStr for WorkFlow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([a-z]+)\{(.+)\}").unwrap();
        let capt = re.captures(s).unwrap();
        let name = capt.get(1).unwrap().as_str().to_string();
        let rules = capt
            .get(2)
            .unwrap()
            .as_str()
            .split(',')
            .into_iter()
            .map(|r| r.parse().unwrap())
            .collect();
        Ok(WorkFlow { name, rules })
    }
}

impl WorkFlow {
    fn process(&self, part: &Part) -> &Destination {
        for rul in &self.rules {
            if let Some(dst) = rul.matches(part) {
                return dst;
            }
        }
        unreachable!("No rule matches part {:?} in workflow {}", part, self.name);
    }
}

#[derive(Debug)]
struct Data {
    workflows: HashMap<String, WorkFlow>,
    parts: Vec<Part>,
}

impl Data {
    fn is_accepted(&self, part: &Part) -> bool {
        let mut name = "in".to_string();
        loop {
            let wf = self.workflows.get(&name).unwrap();
            match wf.process(part) {
                Destination::Accepted => return true,
                Destination::Rejected => return false,
                Destination::WorkFlow(dst) => name = dst.to_string(),
            }
        }
    }

    fn sort(&self) -> Vec<&Part> {
        self.parts.iter().filter(|p| self.is_accepted(p)).collect()
    }
}

fn input(file: &str) -> Data {
    let mut lines = load::<String>(file).into_iter();
    let mut workflows = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let wf: WorkFlow = line.parse().unwrap();
        workflows.insert(wf.name.clone(), wf);
    }
    let mut parts = vec![];
    let re = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
    for line in lines {
        let capt = re.captures(line.as_str()).unwrap();
        let part = [
            capt.get(1).unwrap().as_str().parse().unwrap(),
            capt.get(2).unwrap().as_str().parse().unwrap(),
            capt.get(3).unwrap().as_str().parse().unwrap(),
            capt.get(4).unwrap().as_str().parse().unwrap(),
        ];
        parts.push(part);
    }
    Data { workflows, parts }
}

fn rating(part: &Part) -> u32 {
    part.iter().sum()
}

pub fn part1() -> u32 {
    let data = input("data/day19.txt");
    let accepted = data.sort();
    accepted.iter().map(|p| rating(p)).sum()
}

pub fn part2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 397643);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
