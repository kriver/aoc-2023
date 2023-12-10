use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

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

pub fn char2num(ascii: char) -> u8 {
    ascii as u8 - '0' as u8
}
