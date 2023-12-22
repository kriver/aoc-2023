use core::cmp::Ordering::Equal;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::util::{load, Coord3D};

type Coord = Coord3D<usize>;
type Pile = HashMap<Coord, usize>;
type Support = HashMap<usize, Vec<usize>>;

#[derive(Debug, Eq)]
struct Brick {
    start: Coord,
    end: Coord,
}

impl Brick {
    fn lowest_position(&self) -> usize {
        self.start.z.min(self.end.z)
    }

    fn positions(&self) -> Vec<Coord> {
        let mut positions = Vec::new();
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                for z in self.start.z..=self.end.z {
                    positions.push(Coord::new(x, y, z));
                }
            }
        }
        positions
    }
}

impl PartialEq for Brick {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Equal)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.lowest_position().partial_cmp(&other.lowest_position())
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(['~', ',']).collect::<Vec<_>>();
        Ok(Brick {
            start: Coord::new(
                tokens[0].parse().unwrap(),
                tokens[1].parse().unwrap(),
                tokens[2].parse().unwrap(),
            ),
            end: Coord::new(
                tokens[3].parse().unwrap(),
                tokens[4].parse().unwrap(),
                tokens[5].parse().unwrap(),
            ),
        })
    }
}

fn input(file: &str) -> Vec<Brick> {
    load::<String>(file)
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn drop(bricks: Vec<Brick>) -> (Pile, Support, Support) {
    let mut supports = HashMap::new();
    let mut supported_by = HashMap::new();
    let mut pile = HashMap::new();
    (0..bricks.len()).for_each(|i| {
        supports.insert(i, Vec::new());
        supported_by.insert(i, Vec::new());
    });
    for (brick_id, brick) in bricks.into_iter().enumerate() {
        let mut p = brick.positions();
        loop {
            let at_rest = p
                .iter()
                .any(|c| c.z == 1 || pile.contains_key(&Coord3D::new(c.x, c.y, c.z - 1)));
            if at_rest {
                break;
            }
            p.iter_mut().for_each(|c| c.z -= 1);
        }
        p.into_iter().for_each(|c| {
            if let Some(id) = pile.get(&Coord3D::new(c.x, c.y, c.z - 1)) {
                if *id != brick_id {
                    supports.get_mut(id).unwrap().push(brick_id);
                    supported_by.get_mut(&brick_id).unwrap().push(*id);
                }
            }
            pile.insert(c, brick_id);
        });
    }
    (pile, supports, supported_by)
}

pub fn part1() -> usize {
    let mut bricks = input("data/day22.txt");
    bricks.sort();
    let (_, supports, supported_by) = drop(bricks);
    println!("Supports     = {:?}", supports);
    println!("Supported-by = {:?}", supported_by);
    // we can possibly disintegrate the ones that are part of a multi-brick support for another brick
    let multi_support = supported_by
        .values() // the ones supporting
        .filter(|v| v.len() > 1)
        .flatten()
        .collect::<HashSet<_>>();
    // we can't disintegrate the ones that are the single support for another brick
    let single_support = supported_by
        .values() // the ones supporting
        .filter(|v| v.len() == 1)
        .flatten()
        .collect::<HashSet<_>>();
    // we can always disintegrate the ones on top
    let supports_none = supports
        .iter()
        .filter_map(|(id, v)| if v.is_empty() { Some(id) } else { None })
        .collect::<HashSet<_>>();

    // the ones at the bottom
    // let supported_by_none = supported_by
    //     .iter()
    //     .filter_map(|(id, v)| if v.is_empty() { Some(id) } else { None })
    //     .collect::<HashSet<_>>();

    println!("Supported by multiple = {:?}", multi_support.len());
    println!("Supported by one = {:?}", single_support.len());
    // println!("Supported by none = {:?}", supported_by_none);
    println!("Supports none = {:?}", supports_none.len());
    multi_support
        .difference(&single_support)
        .cloned()
        .collect::<HashSet<_>>()
        .union(&supports_none)
        .count()
}

pub fn part2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 534); // 532 ??
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
