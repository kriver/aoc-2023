use std::collections::HashSet;

use crate::util::{load, Coord2D};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
    Void,
}

type Grid = Vec<Vec<Tile>>;

#[derive(Debug)]
struct Tile {
    c1: Direction,
    c2: Direction,
}

impl Tile {
    fn new(c1: Direction, c2: Direction) -> Self {
        Tile { c1, c2 }
    }

    fn connects(&self, d: Direction) -> Option<Direction> {
        if self.c1 == d {
            Some(self.c2)
        } else if self.c2 == d {
            Some(self.c1)
        } else {
            None
        }
    }

    fn update(&mut self, c1: Direction, c2: Direction) {
        self.c1 = c1;
        self.c2 = c2;
    }

    fn _pipe(&self) -> char {
        match self.c1 {
            Direction::North => match self.c2 {
                Direction::East => 'L',
                Direction::South => '|',
                Direction::West => 'J',
                _ => '.',
            },
            Direction::East => match self.c2 {
                Direction::North => 'L',
                Direction::South => 'F',
                Direction::West => '-',
                _ => '.',
            },
            Direction::South => match self.c2 {
                Direction::North => '|',
                Direction::East => 'F',
                Direction::West => '7',
                _ => '.',
            },
            Direction::West => match self.c2 {
                Direction::North => 'J',
                Direction::East => '-',
                Direction::South => '7',
                _ => '.',
            },
            Direction::Void => '.',
        }
    }
}

type Coord = Coord2D<usize>;

#[derive(Debug)]
struct Move {
    c: Coord,
    came_from: Direction,
}

impl Move {
    fn new(c: Coord, came_from: Direction) -> Self {
        Move { c, came_from }
    }
}

fn input() -> (Coord, Grid) {
    let lines = load::<String>("data/day10.txt");
    let mut start = Coord2D::new(0, 0);
    let grid = lines
        .into_iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '|' => Tile::new(Direction::North, Direction::South),
                    '-' => Tile::new(Direction::East, Direction::West),
                    'L' => Tile::new(Direction::North, Direction::East),
                    'J' => Tile::new(Direction::North, Direction::West),
                    '7' => Tile::new(Direction::South, Direction::West),
                    'F' => Tile::new(Direction::South, Direction::East),
                    '.' => Tile::new(Direction::Void, Direction::Void),
                    'S' => {
                        start = Coord2D::new(x, y);
                        Tile::new(Direction::Void, Direction::Void)
                    }
                    _ => unreachable!("Unexpected char {}", c),
                })
                .collect()
        })
        .collect();
    (start, grid)
}

fn find_starts(grid: &mut Grid, Coord2D { x, y }: Coord) -> (Move, Move) {
    let mut dirs = vec![];
    let mut starts = vec![];
    if x > 0 {
        if let Some(_) = grid[y][x - 1].connects(Direction::East) {
            dirs.push(Direction::West);
            starts.push(Move::new(Coord2D::new(x - 1, y), Direction::East));
        }
    }
    if x < grid[0].len() - 1 {
        if let Some(_) = grid[y][x + 1].connects(Direction::West) {
            dirs.push(Direction::East);
            starts.push(Move::new(Coord2D::new(x + 1, y), Direction::West));
        }
    }
    if y > 0 {
        if let Some(_) = grid[y - 1][x].connects(Direction::South) {
            dirs.push(Direction::North);
            starts.push(Move::new(Coord2D::new(x, y - 1), Direction::South));
        }
    }
    if y < grid.len() - 1 {
        if let Some(_) = grid[y + 1][x].connects(Direction::North) {
            dirs.push(Direction::South);
            starts.push(Move::new(Coord2D::new(x, y + 1), Direction::North));
        }
    }
    // update start pipe
    grid[y][x].update(dirs[0], dirs[1]);
    (starts.pop().unwrap(), starts.pop().unwrap())
}

fn next_step(grid: &Grid, m: Move) -> Move {
    let Coord2D { x, y } = m.c;
    let d = grid[y][x].connects(m.came_from).unwrap();
    match d {
        Direction::North => Move::new(Coord2D { x, y: y - 1 }, Direction::South),
        Direction::East => Move::new(Coord2D { x: x + 1, y }, Direction::West),
        Direction::South => Move::new(Coord2D { x, y: y + 1 }, Direction::North),
        Direction::West => Move::new(Coord2D { x: x - 1, y }, Direction::East),
        Direction::Void => unreachable!("invalid direction"),
    }
}

fn follow_pipe(grid: &mut Grid, start: Coord) -> HashSet<Coord> {
    let mut pipe = HashSet::new();
    pipe.insert(start.clone());
    let (mut m1, mut m2) = find_starts(grid, start);
    while m1.c != m2.c {
        pipe.insert(m1.c);
        pipe.insert(m2.c);
        m1 = next_step(&grid, m1);
        m2 = next_step(&grid, m2);
    }
    pipe.insert(m1.c);
    pipe
}

fn scaled_pipe(pipe: &HashSet<Coord>, grid: &Grid) -> HashSet<Coord> {
    fn set_pipe(p: &mut HashSet<Coord>, x: usize, y: usize, c: &Direction) {
        match c {
            Direction::North => p.insert(Coord2D::new(3 * x + 1, 3 * y)),
            Direction::South => p.insert(Coord2D::new(3 * x + 1, 3 * y + 2)),
            Direction::East => p.insert(Coord2D::new(3 * x + 2, 3 * y + 1)),
            Direction::West => p.insert(Coord2D::new(3 * x, 3 * y + 1)),
            Direction::Void => true,
        };
    }
    let mut scaled_pipe = HashSet::new();
    pipe.iter().for_each(|Coord { x, y }| {
        scaled_pipe.insert(Coord2D::new(3 * x + 1, 3 * y + 1));
        set_pipe(&mut scaled_pipe, *x, *y, &grid[*y][*x].c1);
        set_pipe(&mut scaled_pipe, *x, *y, &grid[*y][*x].c2);
    });
    scaled_pipe
}

fn is_outside(c: &Coord, grid: &Grid, outside: &HashSet<Coord>) -> bool {
    let Coord { x, y } = c;
    if *x == 0 || outside.contains(&Coord { x: *x - 1, y: *y }) {
        true
    } else if *x == grid[0].len() * 3 - 1 || outside.contains(&Coord { x: *x + 1, y: *y }) {
        true
    } else if *y == 0 || outside.contains(&Coord { x: *x, y: *y - 1 }) {
        true
    } else if *y == grid.len() * 3 - 1 || outside.contains(&Coord { x: *x, y: *y + 1 }) {
        true
    } else {
        false
    }
}

fn flood_fill(coord: Coord, grid: &Grid, pipe: &HashSet<Coord>, outside: &mut HashSet<Coord>) {
    // unscaled coord
    let mut to_check = vec![coord];
    loop {
        if let Some(c) = to_check.pop() {
            if !pipe.contains(&c) && !outside.contains(&c) {
                if is_outside(&c, &grid, &outside) {
                    outside.insert(c);
                    let Coord { x, y } = c;
                    if x > 0 {
                        to_check.push(Coord { x: x - 1, y });
                    }
                    if x < grid[0].len() * 3 - 1 {
                        to_check.push(Coord { x: x + 1, y });
                    }
                    if y > 0 {
                        to_check.push(Coord { x, y: y - 1 });
                    }
                    if y < grid.len() * 3 - 1 {
                        to_check.push(Coord { x, y: y + 1 });
                    }
                }
            }
        } else {
            break;
        }
    }
}

pub fn part1() -> usize {
    let (start, mut grid) = input();
    (follow_pipe(&mut grid, start).len() + 1) / 2
}

pub fn part2() -> usize {
    let (start, mut grid) = input();
    let mut outside: HashSet<Coord> = HashSet::new();
    // flood fill (scale 3x3 times to have gaps between pipes for easy filling)
    let pipe = follow_pipe(&mut grid, start);
    let scaled_pipe = scaled_pipe(&pipe, &grid);
    (0..grid.len() * 3).into_iter().for_each(|y| {
        (0..grid[0].len() * 3).into_iter().for_each(|x| {
            let c = Coord2D { x, y };
            flood_fill(c, &grid, &scaled_pipe, &mut outside);
        })
    });
    // non-scaled outside
    let mut outside_cnt = 0;
    for y in (0..grid.len() * 3).step_by(3) {
        for x in (0..grid[0].len() * 3).step_by(3) {
            if !pipe.contains(&Coord { x: x / 3, y: y / 3 }) && outside.contains(&Coord { x, y }) {
                outside_cnt += 1;
            }
        }
    }
    // visual dump
    let inside = grid.len() * grid[0].len() - (pipe.len() + outside_cnt);
    // println!("Pipe {}/{}", pipe.len(), grid.len() * grid[0].len());
    // println!("Out  {}/{}", outside_cnt, grid.len() * grid[0].len());
    // println!("In   {}/{}", inside, grid.len() * grid[0].len());
    // (0..grid.len() * 3).into_iter().for_each(|y| {
    //     (0..grid[0].len() * 3).into_iter().for_each(|x| {
    //         let t = &grid[y / 3][x / 3];
    //         let c = Coord2D { x, y };
    //         if scaled_pipe.contains(&c) {
    //             print!("{}", t.pipe());
    //         } else if outside.contains(&c) {
    //             print!("o");
    //         } else {
    //             print!("?");
    //         }
    //     });
    //     println!();
    // });
    // return the result
    inside
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 6815);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 269);
    }
}
