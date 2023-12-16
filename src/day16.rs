use std::{collections::HashSet, fmt::Display};

use crate::util::{load_grid_map, Coord2D, Grid};

type Coord = Coord2D<usize>;

#[derive(Debug)]
enum Object {
    NONE,
    SLASH,
    BACKSLASH,
    VERTICAL,
    HORIZONTAL,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
}

impl Direction {
    fn is_horizontal(&self) -> bool {
        *self == Direction::LEFT || *self == Direction::RIGHT
    }

    fn is_vertical(&self) -> bool {
        *self == Direction::TOP || *self == Direction::BOTTOM
    }

    fn reflect_slash(&self) -> Direction {
        match self {
            Direction::LEFT => Direction::BOTTOM,
            Direction::RIGHT => Direction::TOP,
            Direction::TOP => Direction::RIGHT,
            Direction::BOTTOM => Direction::LEFT,
        }
    }

    fn reflect_backslash(&self) -> Direction {
        match self {
            Direction::LEFT => Direction::TOP,
            Direction::RIGHT => Direction::BOTTOM,
            Direction::TOP => Direction::LEFT,
            Direction::BOTTOM => Direction::RIGHT,
        }
    }
}

#[derive(Debug)]
struct Beam {
    pos: Coord,
    dir: Direction,
}

impl Beam {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        Beam {
            pos: Coord2D::new(x, y),
            dir,
        }
    }

    fn reflect_slash(mut self) -> Self {
        self.dir = self.dir.reflect_slash();
        self
    }

    fn reflect_backslash(mut self) -> Self {
        self.dir = self.dir.reflect_backslash();
        self
    }
}

#[derive(Debug)]
struct Square {
    object: Object,
    energized: HashSet<Direction>,
}

impl Square {
    fn new(object: Object) -> Self {
        Square {
            object,
            energized: HashSet::new(),
        }
    }

    fn energize(&mut self, beam: Beam) -> Vec<Beam> {
        if self.energized.contains(&beam.dir) {
            vec![]
        } else {
            self.energized.insert(beam.dir);
            match self.object {
                Object::NONE => vec![beam],
                Object::SLASH => vec![beam.reflect_slash()],
                Object::BACKSLASH => vec![beam.reflect_backslash()],
                Object::HORIZONTAL if beam.dir.is_horizontal() => vec![beam],
                Object::HORIZONTAL => vec![
                    Beam {
                        pos: beam.pos,
                        dir: Direction::LEFT,
                    },
                    Beam {
                        pos: beam.pos,
                        dir: Direction::RIGHT,
                    },
                ],
                Object::VERTICAL if beam.dir.is_vertical() => vec![beam],
                Object::VERTICAL => vec![
                    Beam {
                        pos: beam.pos,
                        dir: Direction::TOP,
                    },
                    Beam {
                        pos: beam.pos,
                        dir: Direction::BOTTOM,
                    },
                ],
            }
        }
    }

    fn is_energized(&self) -> bool {
        !self.energized.is_empty()
    }
}

impl Display for Grid<usize, Square> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let s = self.squares.get(&Coord2D::new(x, y)).unwrap();
                write!(f, "{}", if s.is_energized() { '#' } else { '.' })?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid<usize, Square> {
    fn load() -> Self {
        load_grid_map::<usize, Square, _>("data/day16.txt", |c| match c {
            '.' => Some(Square::new(Object::NONE)),
            '|' => Some(Square::new(Object::VERTICAL)),
            '-' => Some(Square::new(Object::HORIZONTAL)),
            '/' => Some(Square::new(Object::SLASH)),
            '\\' => Some(Square::new(Object::BACKSLASH)),
            _ => unreachable!("Invalid char '{}'", c),
        })
    }

    fn move1(&self, Coord { x, y }: &Coord, dir: &Direction) -> Option<Coord> {
        match dir {
            Direction::BOTTOM if *y < self.height - 1 => Some(Coord2D::new(*x, *y + 1)),
            Direction::TOP if *y > 0 => Some(Coord2D::new(*x, *y - 1)),
            Direction::LEFT if *x > 0 => Some(Coord2D::new(*x - 1, *y)),
            Direction::RIGHT if *x < self.width - 1 => Some(Coord2D::new(*x + 1, *y)),
            _ => None,
        }
    }

    fn energize(&mut self, mut beams: Vec<Beam>) {
        loop {
            if beams.is_empty() {
                break;
            }
            let beam = beams.pop().unwrap();
            let new_beams = self.squares.get_mut(&beam.pos).unwrap().energize(beam);
            new_beams
                .into_iter()
                .filter_map(|b| {
                    self.move1(&b.pos, &b.dir)
                        .map(|p| Beam { pos: p, dir: b.dir })
                })
                .for_each(|b| beams.push(b));
        }
    }

    fn energy(&self) -> usize {
        self.squares.values().filter(|s| s.is_energized()).count()
    }
}

pub fn part1() -> usize {
    let mut g = Grid::load();
    g.energize(vec![Beam::new(0, 0, Direction::RIGHT)]);
    g.energy()
}

pub fn part2() -> usize {
    let mut beams = vec![];
    for i in 0..110 {
        beams.push(Beam::new(i, 0, Direction::BOTTOM));
        beams.push(Beam::new(i, 109, Direction::TOP));
        beams.push(Beam::new(0, i, Direction::RIGHT));
        beams.push(Beam::new(109, i, Direction::LEFT));
    }
    beams
        .into_iter()
        .map(|b| {
            let mut g = Grid::load();
            g.energize(vec![b]);
            g.energy()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 8323);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 8491);
    }
}
