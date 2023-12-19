use std::{collections::HashMap, str::FromStr};

use regex::Regex;

use crate::util::load;

type Part = [u64; 4];
type Range = (u64, u64);
type PartRange = [Range; 4];

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
    value: u64,
}

impl Condition {
    fn matches(&self, value: u64) -> bool {
        match self.operator {
            '<' if value < self.value => true,
            '>' if value > self.value => true,
            _ => false,
        }
    }

    // returns (mapped, unmapped)
    fn matches_range(&self, min: u64, max: u64) -> (Option<Range>, Option<Range>) {
        match self.operator {
            '<' if max < self.value => (Some((min, max)), None),
            '<' if min < self.value => (Some((min, self.value - 1)), Some((self.value, max))),
            '>' if min > self.value => (Some((min, max)), None),
            '>' if max > self.value => (Some((self.value + 1, max)), Some((min, self.value))),
            _ => (None, Some((min, max))),
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

    // returns (dst, mapped, unmapped)
    fn matches_range(
        &self,
        range: &PartRange,
    ) -> (Option<(&Destination, PartRange)>, Option<PartRange>) {
        fn update_category(pr: &PartRange, category: usize, r: Range) -> PartRange {
            let mut new_pr = pr.clone();
            new_pr[category] = r;
            new_pr
        }
        match &self.condition {
            None => (Some((&self.dst, range.clone())), None),
            Some(cond) => {
                let (min, max) = range[cond.category];
                match cond.matches_range(min, max) {
                    (None, Some(u)) => (None, Some(update_category(range, cond.category, u))),
                    (Some(m), None) => (
                        Some((&self.dst, update_category(range, cond.category, m))),
                        None,
                    ),
                    (Some(m), Some(u)) => (
                        Some((&self.dst, update_category(range, cond.category, m))),
                        Some(update_category(range, cond.category, u)),
                    ),
                    _ => unreachable!("unexpected mapping"),
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
        for rule in &self.rules {
            if let Some(dst) = rule.matches(part) {
                return dst;
            }
        }
        unreachable!("No rule matches part {:?} in workflow {}", part, self.name);
    }

    fn process_range(&self, range: PartRange) -> Vec<(&Destination, PartRange)> {
        self.rules
            .iter()
            .fold((Some(range), vec![]), |(r, mut result), rule| match r {
                None => (None, result),
                Some(r) => match rule.matches_range(&r) {
                    (None, Some(u)) => (Some(u), result),
                    (Some(m), None) => {
                        result.push(m);
                        (None, result)
                    }
                    (Some(m), Some(u)) => {
                        result.push(m);
                        (Some(u), result)
                    }
                    _ => unreachable!("unexpected mapping"),
                },
            })
            .1
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

    fn sort_parts(&self) -> Vec<&Part> {
        self.parts.iter().filter(|p| self.is_accepted(p)).collect()
    }

    fn determine_accepted(&self, range: PartRange) -> Vec<PartRange> {
        let mut accepted = vec![];
        let mut q = vec![];
        q.push(("in".to_string(), range));
        loop {
            match q.pop() {
                None => break,
                Some((name, r)) => {
                    let wf = self.workflows.get(&name).unwrap();
                    let new_ranges = wf.process_range(r);
                    new_ranges.into_iter().for_each(|(dst, r)| match dst {
                        Destination::Accepted => accepted.push(r),
                        Destination::Rejected => (),
                        Destination::WorkFlow(dst) => q.push((dst.to_string(), r)),
                    });
                }
            }
        }
        accepted
    }

    fn accepted_ranges(&self, range: PartRange) -> Vec<PartRange> {
        self.determine_accepted(range)
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

fn rating(part: &Part) -> u64 {
    part.iter().sum()
}

pub fn part1() -> u64 {
    let data = input("data/day19.txt");
    let accepted = data.sort_parts();
    accepted.iter().map(|p| rating(p)).sum()
}

pub fn part2() -> u64 {
    let data = input("data/day19.txt");
    let range: PartRange = [(1, 4000), (1, 4000), (1, 4000), (1, 4000)];
    let accepted = data.accepted_ranges(range);
    accepted
        .into_iter()
        .map(|r| {
            (r[0].1 - r[0].0 + 1)
                * (r[1].1 - r[1].0 + 1)
                * (r[2].1 - r[2].0 + 1)
                * (r[3].1 - r[3].0 + 1)
        })
        .sum::<u64>()
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
        assert_eq!(part2(), 132392981697081);
    }
}
