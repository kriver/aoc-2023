use std::{cmp, str::FromStr};

use regex::Regex;

use crate::util::load;

#[derive(Debug, Default)]
struct Grab {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Grab {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r" *([0-9]+) ([a-z]+)").unwrap();
        let mut grab = Grab::default();
        let colours: Vec<&str> = s.split(',').collect();
        colours.into_iter().for_each(|c| {
            re.captures(c).map(|capt| {
                let cnt: u32 = capt.get(1).map(|n| n.as_str().parse().unwrap()).unwrap();
                match capt.get(2).expect("capture").as_str() {
                    "red" => grab.red = cnt,
                    "green" => grab.green = cnt,
                    "blue" => grab.blue = cnt,
                    _ => unreachable!("expected colour",),
                }
            });
        });
        Ok(grab)
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    grabs: Vec<Grab>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Game ([0-9]+):(.*)").unwrap();
        let capt = re.captures(s).unwrap();
        let id = capt.get(1).unwrap().as_str().parse().unwrap();
        let grabs = capt
            .get(2)
            .unwrap()
            .as_str()
            .split(';')
            .into_iter()
            .map(|g| g.parse().unwrap())
            .collect();
        Ok(Game { id, grabs })
    }
}

pub fn part1() -> u32 {
    let games: Vec<Game> = load("data/day02.txt");
    games
        .into_iter()
        .filter(|game| {
            game.grabs.iter().fold(true, |acc, grab| {
                acc && grab.red <= 12 && grab.green <= 13 && grab.blue <= 14
            })
        })
        .map(|game| game.id)
        .sum()
}

pub fn part2() -> u32 {
    let games: Vec<Game> = load("data/day02.txt");
    games
        .into_iter()
        .map(|game| {
            game.grabs.into_iter().reduce(|acc, grab| Grab {
                red: cmp::max(acc.red, grab.red),
                green: cmp::max(acc.green, grab.green),
                blue: cmp::max(acc.blue, grab.blue),
            })
        })
        .map(|grab| grab.unwrap())
        .map(|grab| grab.red * grab.green * grab.blue)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 2879);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 65122);
    }
}
