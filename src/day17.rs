use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt::Debug,
};

use crate::util::{char2num, load, Coord2D};

type Coord = Coord2D<i32>;
type Map = Vec<Vec<usize>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

const DIRS: &[Direction; 4] = &[
    Direction::NORTH,
    Direction::EAST,
    Direction::SOUTH,
    Direction::WEST,
];

#[derive(Debug, Eq)]
struct State {
    cost: usize,
    pos: Coord,
    dir: Direction,
    cnt_straight: usize,
}

impl State {
    fn new(x: i32, y: i32, dir: Direction) -> Self {
        State {
            cost: 0,
            pos: Coord2D::new(x, y),
            dir,
            cnt_straight: 0,
        }
    }

    fn step(&self, map: &Map, nd: &Direction, dx: i32, dy: i32) -> State {
        fn new_cnt(d1: &Direction, d2: &Direction, cnt: usize) -> usize {
            if d1 == d2 {
                cnt + 1
            } else {
                1
            }
        }
        let (nx, ny) = (self.pos.x + dx, self.pos.y + dy);
        State {
            cost: self.cost + map[ny as usize][nx as usize],
            pos: Coord2D::new(nx, ny),
            dir: *nd,
            cnt_straight: new_cnt(&self.dir, &nd, self.cnt_straight),
        }
    }

    fn key(&self) -> i32 {
        let d = match self.dir {
            Direction::NORTH => 1,
            Direction::EAST => 2,
            Direction::SOUTH => 3,
            Direction::WEST => 4,
        };
        (self.cost as i32) << 20
            | (self.cnt_straight as i32) << 18
            | d << 16
            | (self.pos.x << 8)
            | self.pos.y
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        Some(Ordering::Equal) == self.partial_cmp(other)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.cost.partial_cmp(&other.cost) {
            Some(Ordering::Equal) => (self.pos.x + self.pos.y)
                .partial_cmp(&(other.pos.x + other.pos.y))
                .map(|o| o.reverse()),
            ord => ord.map(|o| o.reverse()),
        }
    }
}

fn input() -> Map {
    load::<String>("data/day17.txt")
        .into_iter()
        .map(|l| l.chars().map(|c| char2num(c) as usize).collect())
        .collect()
}

trait Move {
    fn step(&self, s: &State, d: &Direction) -> Option<State>;
}

struct Crucible {
    map: Map,
}

impl Move for Crucible {
    fn step(&self, s: &State, d: &Direction) -> Option<State> {
        match d {
            Direction::NORTH
                if s.pos.y > 0
                    && s.dir != Direction::SOUTH
                    && (s.dir != Direction::NORTH || s.cnt_straight < 3) =>
            {
                Some(s.step(&self.map, &d, 0, -1))
            }
            Direction::EAST
                if s.pos.x < self.map[0].len() as i32 - 1
                    && s.dir != Direction::WEST
                    && (s.dir != Direction::EAST || s.cnt_straight < 3) =>
            {
                Some(s.step(&self.map, &d, 1, 0))
            }
            Direction::SOUTH
                if s.pos.y < self.map.len() as i32 - 1
                    && s.dir != Direction::NORTH
                    && (s.dir != Direction::SOUTH || s.cnt_straight < 3) =>
            {
                Some(s.step(&self.map, &d, 0, 1))
            }
            Direction::WEST
                if s.pos.x > 0
                    && s.dir != Direction::EAST
                    && (s.dir != Direction::WEST || s.cnt_straight < 3) =>
            {
                Some(s.step(&self.map, &d, -1, 0))
            }
            _ => None,
        }
    }
}

fn travel(movable: impl Move, dst: Coord, mut q: BinaryHeap<State>) -> usize {
    let mut visited = HashSet::new();
    // let mut it = 0;
    loop {
        match q.pop() {
            None => unreachable!("No path found"),
            Some(state) => {
                // it += 1;
                // if it % 500000 == 0 {
                //     println!(
                //         "{:10} At ({}, {}) with cost {} (#Q = {})",
                //         it,
                //         state.pos.x,
                //         state.pos.y,
                //         state.cost,
                //         q.len()
                //     );
                // }
                if state.pos == dst {
                    break state.cost;
                }
                DIRS.iter()
                    .filter_map(|d| movable.step(&state, d))
                    .for_each(|s| {
                        let k = s.key();
                        if !visited.contains(&k) {
                            visited.insert(k);
                            q.push(s)
                        }
                    });
            }
        }
    }
}

pub fn part1() -> usize {
    let mut q = BinaryHeap::new();
    q.push(State::new(0, 0, Direction::EAST));
    let map = input();
    let dst = Coord2D::new(map[0].len() as i32 - 1, map.len() as i32 - 1);
    travel(Crucible { map }, dst, q)
}

pub fn part2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 722);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
