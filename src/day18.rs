use std::{collections::HashSet, str::FromStr};

use num::integer::Average;

use crate::util::{load, Coord2D};

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('L') => Ok(Direction::LEFT),
            Some('R') => Ok(Direction::RIGHT),
            Some('U') => Ok(Direction::UP),
            Some('D') => Ok(Direction::DOWN),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::LEFT => (-1, 0),
            Direction::RIGHT => (1, 0),
            Direction::UP => (0, -1),
            Direction::DOWN => (0, 1),
        }
    }
}

struct Dig {
    dir: Direction,
    len: usize,
    color: String,
}

impl FromStr for Dig {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_ascii_whitespace().collect::<Vec<_>>();
        Ok(Dig {
            dir: tokens[0].parse().unwrap(),
            len: tokens[1].parse().unwrap(),
            color: tokens[2][2..7].to_string(),
        })
    }
}
type Coord = Coord2D<i32>;
type Grid = HashSet<Coord>;

fn input(file: &str) -> Vec<Dig> {
    load::<String>(file)
        .into_iter()
        .map(|l| l.parse().unwrap())
        .collect()
}

pub fn part1() -> usize {
    let digs = input("data/day18.txt");
    let mut grid: Grid = HashSet::new();
    grid.insert(Coord2D::new(0, 0));
    // start digging
    let (mut x, mut y) = (0, 0);
    let (mut xmin, mut ymin, mut xmax, mut ymax) = (0, 0, 0, 0);
    for dig in digs {
        let (dx, dy) = dig.dir.delta();
        for _ in 0..dig.len {
            x += dx;
            y += dy;
            grid.insert(Coord2D::new(x, y));
            xmin = xmin.min(x);
            ymin = ymin.min(y);
            xmax = xmax.max(x);
            ymax = ymax.max(y);
        }
    }
    // find an interior point
    y = ymin.average_floor(&ymax);
    x = (xmin..=xmax)
        .into_iter()
        .filter(|ax| grid.contains(&Coord2D::new(*ax, y)))
        .take(2)
        .sum::<i32>()
        / 2;
    // floodfill
    let mut to_fill = vec![Coord2D::new(x, y)];
    loop {
        match to_fill.pop() {
            None => break,
            Some(c) => {
                grid.insert(c);
                for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nc = Coord2D::new(c.x + dx, c.y + dy);
                    if !grid.contains(&nc) {
                        to_fill.push(nc);
                    }
                }
            }
        }
    }
    // dig size
    grid.len()
}

pub fn part2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 53844);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
