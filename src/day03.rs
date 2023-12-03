use std::{cmp, collections::HashMap};

use crate::util::{char2num, load, Coord2D};

#[derive(Debug)]
struct Parts {
    part_nums: HashMap<Coord2D, Vec<u32>>,
    pub parts: HashMap<Coord2D, char>,
}

impl Parts {
    fn new() -> Self {
        Parts {
            part_nums: HashMap::new(),
            parts: HashMap::new(),
        }
    }

    fn add_part_num_1(&mut self, x: i32, y: i32, part_num: u32) {
        // println!("  adding {}, {}", x, y);
        let c = Coord2D::new(x, y);
        match self.part_nums.get_mut(&c) {
            Some(v) => v.push(part_num),
            None => {
                self.part_nums.insert(c, vec![part_num]);
                ()
            }
        }
    }

    fn add_part_num(&mut self, xl: i32, xr: i32, y: i32, part_num: u32) {
        if part_num > 0 {
            // println!("Adding {} for {} - {}, {}", part_num, xl, xr, y);
            // left
            if xl > 0 {
                self.add_part_num_1(xl - 1, y, part_num);
            }
            // right
            self.add_part_num_1(xr + 1, y, part_num);
            // above/below
            (cmp::max(0, xl - 1)..=(xr + 1)).for_each(|x| {
                if y > 0 {
                    self.add_part_num_1(x, y - 1, part_num);
                }
                self.add_part_num_1(x, y + 1, part_num);
            });
        }
    }

    fn add_part(&mut self, x: i32, y: i32, part: char) {
        // println!("Adding {} at {}, {}", part, x, y);
        self.parts.insert(Coord2D::new(x, y), part);
    }

    fn load(&mut self) {
        let mut start = 0i32;
        let mut num = 0u32;
        let lines: Vec<String> = load("data/day03.txt");
        lines.into_iter().enumerate().for_each(|(y, line)| {
            line.chars()
                .into_iter()
                .enumerate()
                .for_each(|(x, c)| match c {
                    '0'..='9' => {
                        if num == 0 {
                            start = x as i32
                        }
                        num = num * 10 + char2num(c) as u32
                    }
                    '.' => {
                        self.add_part_num(start, x as i32 - 1, y as i32, num);
                        num = 0
                    }
                    _ => {
                        self.add_part_num(start, x as i32 - 1, y as i32, num);
                        self.add_part(x as i32, y as i32, c);
                        num = 0
                    }
                });
            self.add_part_num(start, line.len() as i32 - 1, y as i32, num);
            num = 0
        })
    }

    fn part_number_sum(&self, _part: &char, c: &Coord2D) -> u32 {
        // println!("Checking [{}] at {:?} -> {:?}", _part, c, self.part_nums.get(c));
        match self.part_nums.get(c) {
            Some(v) => v.iter().sum(),
            None => 0,
        }
    }

    fn part_number_product(&self, _part: &char, c: &Coord2D) -> u32 {
        // println!("Checking [{}] at {:?} -> {:?}", _part, c, self.part_nums.get(c));
        match self.part_nums.get(c) {
            Some(v) if v.len() == 2 => v.iter().product(),
            Some(_) | None => 0,
        }
    }
}

pub fn part1() -> u32 {
    let mut p = Parts::new();
    p.load();
    p.parts
        .iter()
        .map(|(coord, part)| p.part_number_sum(part, coord))
        .sum()
}

pub fn part2() -> u32 {
    let mut p = Parts::new();
    p.load();
    p.parts
        .iter()
        .filter(|(_coord, part)| **part == '*')
        .map(|(coord, part)| p.part_number_product(part, coord))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 556367);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 89471771);
    }
}
