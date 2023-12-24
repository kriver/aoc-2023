use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use itertools::Itertools;

use crate::util::{load, Coord3D};

type Coord = Coord3D<f32>;

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
struct HailStone {
    p: Coord,
    v: Coord,
}

impl Display for HailStone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "p: {}, v: {}", self.p, self.v)
    }
}

impl FromStr for HailStone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split([',', '@', ' ']).filter(|s| !s.is_empty()).collect();
        Ok(HailStone {
            p: Coord::new(
                tokens[0].parse().unwrap(),
                tokens[1].parse().unwrap(),
                tokens[2].parse().unwrap(),
            ),
            v: Coord::new(
                tokens[3].parse().unwrap(),
                tokens[4].parse().unwrap(),
                tokens[5].parse().unwrap(),
            ),
        })
    }
}

impl HailStone {
    fn slope(&self) -> f32 {
        self.v.y / self.v.x
    }

    fn y_intercept(&self) -> f32 {
        self.p.y - self.slope() * self.p.x
    }

    fn intersection(&self, other: &HailStone) -> Option<Coord> {
        let slope = self.slope();
        let intercept = self.y_intercept();
        let ab = slope - other.slope();
        let dc = other.y_intercept() - intercept;
        let dc_ab = dc / ab;
        if ab != 0.0 {
            let i = Coord::new(dc_ab, slope * dc_ab + intercept, 0.0);
            // println!("Intersection of {} and {} is {}", self, other, i);
            Some(i)
        } else {
            None
        }
    }
}

impl Coord {
    fn is_in(&self, c1: &Coord, c2: &Coord) -> bool {
        (c1.x <= self.x) && (self.x <= c2.x) && (c1.y <= self.y) && (self.y <= c2.y)
    }

    fn is_in_future(&self, hs: &HailStone) -> bool {
        fn comp_ok(dc: f32, sc: f32, c: f32) -> bool {
            if dc >= 0.0 {
                sc >= c
            } else {
                sc < c
            }
        }
        comp_ok(hs.v.x, self.x, hs.p.x) && comp_ok(hs.v.y, self.y, hs.p.y)
    }
}

fn input(file: &str) -> Vec<HailStone> {
    load::<String>(file)
        .into_iter()
        .map(|l| l.parse().unwrap())
        .collect_vec()
}

pub fn part1() -> usize {
    let hs = input("data/day24.txt");
    let (min, max) = (200000000000000.0, 400000000000000.0);
    // let hs = input("data/test.txt");
    // let (min, max) = (7.0, 27.0);
    let c1 = Coord::new(min, min, 0.0);
    let c2 = Coord::new(max, max, 0.0);
    hs.into_iter()
        .combinations(2)
        .filter_map(|c| c[0].intersection(&c[1]).map(|i| (c, i)))
        .filter(|(_, i)| i.is_in(&c1, &c2))
        .filter(|(c, i)| i.is_in_future(&c[0]))
        .filter(|(c, i)| i.is_in_future(&c[1]))
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
        assert_eq!(part1(), 19976);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
