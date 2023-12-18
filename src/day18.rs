use std::{cmp::Ordering, collections::BinaryHeap, str::FromStr};

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
    fn delta(&self) -> (i64, i64) {
        match self {
            Direction::LEFT => (-1, 0),
            Direction::RIGHT => (1, 0),
            Direction::UP => (0, -1),
            Direction::DOWN => (0, 1),
        }
    }
}

#[derive(Debug)]
struct Dig {
    dir: Direction,
    len: i64,
    color: String,
}

impl FromStr for Dig {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_ascii_whitespace().collect::<Vec<_>>();
        Ok(Dig {
            dir: tokens[0].parse().unwrap(),
            len: tokens[1].parse().unwrap(),
            color: tokens[2][2..8].to_string(),
        })
    }
}

impl Dig {
    fn convert(&self) -> Self {
        let dir = match self.color.chars().nth(5).unwrap() {
            '0' => Direction::RIGHT,
            '1' => Direction::DOWN,
            '2' => Direction::LEFT,
            '3' => Direction::UP,
            c => unreachable!("Invalid color {}", c),
        };
        let len = i64::from_str_radix(&self.color[0..5], 16).unwrap();
        Dig {
            dir,
            len,
            color: "".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HorEdge {
    y: i64,
    x1: i64,
    x2: i64,
}

impl HorEdge {
    fn new(y: i64, x1: i64, x2: i64) -> Self {
        HorEdge { y, x1, x2 }
    }
}

impl Ord for HorEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for HorEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.y.partial_cmp(&other.y) {
            Some(Ordering::Equal) => match self.x1.partial_cmp(&other.x1) {
                Some(o) => Some(o.reverse()),
                None => None,
            },
            Some(o) => Some(o.reverse()),
            None => None,
        }
    }
}

struct State {
    digs: Vec<Dig>,
    edges: BinaryHeap<HorEdge>,
}

impl State {
    fn load(file: &str, convert: bool) -> Self {
        let digs: Vec<Dig> = load::<String>(file)
            .into_iter()
            .map(|l| l.parse::<Dig>().unwrap())
            .map(|d| if convert { d.convert() } else { d })
            .collect();
        State {
            digs,
            edges: BinaryHeap::new(),
        }
    }

    fn dig(&mut self) {
        let mut prev = Coord2D::new(0, 0);
        for dig in self.digs.iter() {
            let (dx, dy) = dig.dir.delta();
            let next = Coord2D::new(prev.x + dx * dig.len, prev.y + dy * dig.len);
            if dy == 0 {
                self.edges.push(HorEdge {
                    y: prev.y,
                    x1: prev.x.min(next.x),
                    x2: prev.x.max(next.x),
                });
            };
            prev = next;
        }
    }

    fn combine(&self, mut edges: BinaryHeap<HorEdge>, edge: HorEdge) -> (u64, BinaryHeap<HorEdge>) {
        fn handle_edge(
            result: &mut BinaryHeap<HorEdge>,
            prev: Option<HorEdge>,
            y: i64,
            mut e: HorEdge,
        ) -> (u64, Option<HorEdge>) {
            match prev {
                None => {
                    e.y = y;
                    result.push(e);
                    (0, None)
                }
                Some(p) if p.x2 < e.x1 => {
                    //  _  | |
                    // |p| |e|
                    e.y = y;
                    result.push(e);
                    result.push(p);
                    (0, None)
                }
                Some(p) if p.x2 == e.x1 => {
                    //   _
                    // _|
                    result.push(HorEdge::new(y, p.x1, e.x2));
                    (0, None)
                }
                Some(p) if p.x1 == e.x1 => {
                    if p.x2 == e.x2 {
                        // |_|
                        (p.x1.abs_diff(p.x2) + 1, None)
                    } else {
                        //  _
                        // |_
                        result.push(HorEdge::new(y, p.x2, e.x2));
                        (p.x1.abs_diff(p.x2), None)
                    }
                }
                Some(p) if p.x1 < e.x2 => {
                    if p.x2 < e.x2 {
                        // |  _  |
                        // | | | |
                        result.push(HorEdge::new(y, e.x1, p.x1));
                        result.push(HorEdge::new(y, p.x2, e.x2));
                        (p.x1.abs_diff(p.x2) - 1, None)
                    } else {
                        //  _|
                        // |
                        result.push(HorEdge::new(y, e.x1, p.x1));
                        (p.x1.abs_diff(e.x2), None)
                    }
                }
                Some(p) if p.x1 == e.x2 => {
                    // |_
                    //   |
                    (0, Some(HorEdge::new(y, e.x1, p.x2)))
                }
                Some(p) => {
                    // | |  _
                    // |e| |p|
                    e.y = y;
                    result.push(e);
                    (0, Some(p))
                }
            }
        }
        let y = edge.y;
        let mut result = BinaryHeap::new();
        let mut dropped = 0;
        let mut prev = Some(edge);
        loop {
            match edges.pop() {
                None => break,
                Some(e) => {
                    let (d, p) = handle_edge(&mut result, prev, y, e);
                    dropped += d;
                    prev = p;
                }
            };
        }
        prev.into_iter().for_each(|e| result.push(e));
        (dropped, result)
    }

    fn flood_fill(&mut self) -> u64 {
        let mut volume = 0;
        let first = self.edges.pop().unwrap();
        // println!("\tfirst edge {:?}", first);
        let mut y = first.y;
        let mut edges = BinaryHeap::new();
        edges.push(first);
        loop {
            // println!("==========================================================");
            let popped = self.edges.pop();
            // println!("\tcurrent edges are {:?}", edges);
            // println!("\tnext edge {:?}", popped);
            match popped {
                None => break,
                Some(edge) if edge.y == y => {
                    let (d, e) = self.combine(edges, edge);
                    edges = e;
                    volume += d;
                    // println!(
                    //     "V={} combined-A to {:?}, dropped {}",
                    //     volume,
                    //     edges.len(),
                    //     d
                    // );
                }
                Some(edge) => {
                    // add this y-level
                    volume += edges.iter().map(|e| e.x2.abs_diff(e.x1) + 1).sum::<u64>();
                    // println!("V={} for current y-level add", volume);
                    // new y-level
                    y = edge.y;
                    // add blocks
                    edges
                        .iter()
                        .for_each(|e| volume += (e.x2.abs_diff(e.x1) + 1) * (e.y.abs_diff(y) - 1));
                    // println!("V={} after block add for {:?}", volume, edges.len());
                    // combine
                    let (d, e) = self.combine(edges, edge);
                    edges = e;
                    volume += d;
                    // println!(
                    //     "V={} combined-B to {:?}, dropped {}",
                    //     volume,
                    //     edges.len(),
                    //     d
                    // );
                }
            }
        }
        volume
    }
}

pub fn part1() -> u64 {
    let mut grid = State::load("data/day18.txt", false);
    grid.dig();
    grid.flood_fill()
}

pub fn part2() -> u64 {
    let mut grid = State::load("data/day18.txt", true);
    grid.dig();
    grid.flood_fill()
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
        assert_eq!(part2(), 42708339569950);
    }
}
