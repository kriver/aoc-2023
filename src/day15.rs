use std::collections::HashMap;

use crate::util::load;

struct HolidayAsciiStringHelper {
    steps: Vec<String>,
    boxes: Vec<Vec<String>>,
    lenses: HashMap<String, u32>,
}

fn hash(s: &str) -> u8 {
    s.as_bytes()
        .iter()
        .fold(0u32, |v, c| (v + (*c as u32)) * 17 % 256) as u8
}

impl HolidayAsciiStringHelper {
    fn load(file: &str) -> Self {
        HolidayAsciiStringHelper {
            steps: load::<String>(file)[0]
                .split(",")
                .map(|s| s.to_string())
                .collect(),
            boxes: (0..256).map(|_| vec![]).collect(),
            lenses: HashMap::new(),
        }
    }

    fn calc_hash_sum(&self) -> usize {
        self.steps.iter().map(|s| hash(s) as usize).sum()
    }

    fn init_lenses(&mut self) {
        self.steps
            .iter()
            .map(|s| {
                let mut tokens = s.split(['=', '-']);
                (
                    tokens.next().unwrap(),
                    tokens.next().unwrap().parse::<usize>().ok(),
                )
            })
            .map(|(lens, value)| (lens, value, hash(&lens)))
            .for_each(|(lens, value, box_id)| match value {
                Some(focal_length) => {
                    // add lens
                    self.lenses.insert(lens.to_string(), focal_length as u32);
                    if self.boxes[box_id as usize].iter().all(|l| l != lens) {
                        self.boxes[box_id as usize].push(lens.to_string());
                    }
                }
                None => self.boxes[box_id as usize].retain(|l| l != lens), // remove lens
            });
    }

    fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(box_id, b)| {
                b.iter()
                    .enumerate()
                    .map(|(lens_slot, label)| {
                        (box_id + 1) * (lens_slot + 1) * *self.lenses.get(label).unwrap() as usize
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

pub fn part1() -> usize {
    HolidayAsciiStringHelper::load("data/day15.txt").calc_hash_sum()
}

pub fn part2() -> usize {
    let mut helper = HolidayAsciiStringHelper::load("data/day15.txt");
    helper.init_lenses();
    helper.focusing_power()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 511257);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 239484);
    }
}
