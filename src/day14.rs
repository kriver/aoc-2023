use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

use crate::util::{Coord2D, Grid};

#[derive(Debug, PartialEq, Eq)]
enum Square {
    FIXED,
    MOVING,
}

fn input() -> Grid<usize, Square> {
    Grid::from_file("data/day14.txt", |c| match c {
        '.' => None,
        '#' => Some(Square::FIXED),
        'O' => Some(Square::MOVING),
        _ => unreachable!("Invalid char '{}'", c),
    })
}

impl Grid<usize, Square> {
    fn calc_load(&self) -> usize {
        let mut load = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(Square::MOVING) = self.squares.get(&Coord2D::new(x, y)) {
                    load += self.height - y;
                }
            }
        }
        load
    }

    fn key(&self) -> String {
        self.squares
            .iter()
            .filter(|(_, v)| **v == Square::MOVING)
            .map(|(k, _)| k.y * 100 + k.x)
            .sorted()
            .map(|n| format!("{:04}", n))
            .collect()
    }

    fn find_next_empty_space(&self, x: usize, y: usize, dx: i32, dy: i32) -> (usize, usize) {
        let mut nx = x as i32 + dx;
        let mut ny = y as i32 + dy;
        while self.squares.contains_key(&Coord2D {
            x: nx as usize,
            y: ny as usize,
        }) {
            nx += dx;
            ny += dy;
        }
        (nx as usize, ny as usize)
    }

    fn tilt_north(&mut self) {
        let mut coord = Coord2D::new(0, 0);
        for x in 0..self.width {
            coord.x = x;
            let mut new_y = 0;
            for y in 0..self.height {
                coord.y = y;
                if let Some(s) = self.squares.get(&coord) {
                    match s {
                        Square::MOVING if y != new_y => {
                            self.squares.remove(&coord);
                            self.squares.insert(Coord2D::new(x, new_y), Square::MOVING);
                            new_y = self.find_next_empty_space(x, new_y, 0, 1).1;
                        }
                        _ => new_y = y + 1,
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let mut coord = Coord2D::new(0, 0);
        for x in 0..self.width {
            coord.x = x;
            let mut new_y = self.height - 1;
            for y in (0..self.height).rev() {
                coord.y = y;
                if let Some(s) = self.squares.get(&coord) {
                    match s {
                        Square::MOVING if y != new_y => {
                            self.squares.remove(&coord);
                            self.squares.insert(Coord2D::new(x, new_y), Square::MOVING);
                            new_y = self.find_next_empty_space(x, new_y, 0, -1).1;
                        }
                        _ => {
                            if y > 0 {
                                new_y = y - 1;
                            }
                        }
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        let mut coord = Coord2D::new(0, 0);
        for y in 0..self.height {
            coord.y = y;
            let mut new_x = 0;
            for x in 0..self.width {
                coord.x = x;
                if let Some(s) = self.squares.get(&coord) {
                    match s {
                        Square::MOVING if x != new_x => {
                            self.squares.remove(&coord);
                            self.squares.insert(Coord2D::new(new_x, y), Square::MOVING);
                            new_x = self.find_next_empty_space(new_x, y, 1, 0).0;
                        }
                        _ => new_x = x + 1,
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        let mut coord = Coord2D::new(0, 0);
        for y in 0..self.height {
            coord.y = y;
            let mut new_x = self.width - 1;
            for x in (0..self.width).rev() {
                coord.x = x;
                if let Some(s) = self.squares.get(&coord) {
                    match s {
                        Square::MOVING if x != new_x => {
                            self.squares.remove(&coord);
                            self.squares.insert(Coord2D::new(new_x, y), Square::MOVING);
                            new_x = self.find_next_empty_space(new_x, y, -1, 0).0;
                        }
                        _ => {
                            if x > 0 {
                                new_x = x - 1;
                            }
                        }
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }
}

impl Display for Grid<usize, Square> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(s) = self.squares.get(&Coord2D::new(x, y)) {
                    match s {
                        Square::MOVING => write!(f, "O")?,
                        Square::FIXED => write!(f, "#")?,
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part1() -> usize {
    let mut grid = input();
    grid.tilt_north();
    grid.calc_load()
}

pub fn part2() -> usize {
    let cycles = 1_000_000_000;
    let mut grid = input();
    let mut visited: HashMap<String, usize> = HashMap::new();
    let mut it = 0;
    let period = loop {
        if it == cycles {
            unreachable!("Expected to exit early");
        }
        it += 1;
        grid.cycle();
        let key = grid.key();
        if visited.contains_key(&key) {
            // found repeat
            break it - visited.get(&key).unwrap(); // period
        }
        visited.insert(key, it);
    };
    // skip a bunch of repeating cycles
    let left = (cycles - it) % period;
    // do the remainder
    for _ in 0..left {
        grid.cycle();
    }
    grid.calc_load()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 109385);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 93102);
    }
}
