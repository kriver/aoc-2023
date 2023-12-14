use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Coord2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coord2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Coord2D { x, y }
    }
}

pub fn load<T>(filename: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

/**
 * T: coordinate type
 * S: single grid square type
 */
#[derive(Debug)]
pub struct Grid<T, S> {
    pub width: usize,
    pub height: usize,
    pub squares: HashMap<Coord2D<T>, S>,
}

pub fn load_grid_map<T, S, F>(filename: &str, into_square: F) -> Grid<T, S>
where
    T: Eq + Hash + From<usize>,
    F: Fn(char) -> Option<S>,
{
    let lines = load::<String>(filename);
    let height = lines.len();
    let width = lines[0].len();
    Grid {
        width,
        height,
        squares: lines
            .into_iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        // try_into().unwrap() for usize -> T
                        let coord = Coord2D::new(x.try_into().unwrap(), y.try_into().unwrap());
                        into_square(c).map(|s| (coord, s))
                    })
                    .collect::<HashMap<_, _>>()
            })
            .collect(),
    }
}

pub fn char2num(ascii: char) -> u8 {
    ascii as u8 - '0' as u8
}
