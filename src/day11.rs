use itertools::Itertools;
use std::collections::HashSet;

use crate::util::{Coord2D, Grid};

type Coord = Coord2D<usize>;
type Galaxies = HashSet<Coord>;

fn input() -> Galaxies {
    Grid::from_file("data/day11.txt", |c, _| if c == '#' { Some(()) } else { None })
        .squares
        .keys()
        .cloned()
        .collect()
}

fn calc_expansion(nums: HashSet<usize>, factor: usize) -> Vec<usize> {
    (0..=*nums.iter().max().unwrap())
        .fold((0, vec![]), |(exp, mut acc), i| {
            acc.push(exp);
            (exp + if nums.contains(&i) { 0 } else { factor }, acc)
        })
        .1
}

fn manhatten_dist(c1: &Coord, c2: &Coord) -> usize {
    c1.x.abs_diff(c2.x) + c1.y.abs_diff(c2.y)
}

fn expanded_distance(g1: Coord, g2: Coord, x_exp: &Vec<usize>, y_exp: &Vec<usize>) -> usize {
    manhatten_dist(&g1, &g2)
        + manhatten_dist(
            &Coord {
                x: x_exp[g1.x],
                y: y_exp[g1.y],
            },
            &Coord {
                x: x_exp[g2.x],
                y: y_exp[g2.y],
            },
        )
}

fn distance_sum(factor: usize) -> usize {
    let galaxies = input();
    let x_exp = calc_expansion(galaxies.iter().map(|c| c.x).collect(), factor);
    let y_exp = calc_expansion(galaxies.iter().map(|c| c.y).collect(), factor);
    galaxies
        .into_iter()
        .combinations(2)
        .map(|g| expanded_distance(g[0], g[1], &x_exp, &y_exp))
        .sum()
}

pub fn part1() -> usize {
    distance_sum(1)
}

pub fn part2() -> usize {
    distance_sum(1_000_000 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 10173804);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 634324905172);
    }
}
