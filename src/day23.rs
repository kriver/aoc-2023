use std::collections::{HashMap, HashSet};

use crate::util::{Coord2D, Grid};

type Coord = Coord2D<i32>;
type Forest = Grid<i32, Trail>;
#[derive(Debug)]
enum Trail {
    Path,
    Slope(Coord),
}

fn input(file: &str) -> Forest {
    Grid::from_file(file, |c, &coord| match c {
        '#' => None,
        '.' => Some(Trail::Path),
        '>' => Some(Trail::Slope(Coord::new(coord.x + 1, coord.y))),
        'v' => Some(Trail::Slope(Coord::new(coord.x, coord.y + 1))),
        '<' => Some(Trail::Slope(Coord::new(coord.x - 1, coord.y))),
        '^' => Some(Trail::Slope(Coord::new(coord.x, coord.y - 1))),
        _ => unreachable!("invalid square"),
    })
}

fn find_longest(forest: Forest, start: Coord, end: Coord) -> usize {
    fn next_steps(
        forest: &Forest,
        Coord { x, y }: &Coord,
        visited: &mut HashSet<Coord>,
    ) -> Vec<(Coord, Option<Coord>)> {
        let mut next = vec![];
        for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let c = Coord::new(x + dx, y + dy);
            if !visited.contains(&c) {
                match forest.squares.get(&c) {
                    Some(Trail::Path) => next.push((c, None)),
                    Some(Trail::Slope(Coord { x: sx, y: sy })) if sx != x || sy != y => {
                        next.push((Coord::new(*sx, *sy), Some(c)));
                    }
                    None | _ => (),
                }
            }
        }
        next
    }
    fn recurse(forest: &Forest, pos: &Coord, end: &Coord, path: &mut HashSet<Coord>) -> usize {
        // println!("At {:?} with path of {}", pos, path.len());
        let mut longest = 0;
        if pos == end {
            // println!("!!! Found path of {}", path.len());
            return path.len() - 1; // ignore start
        }
        let next = next_steps(forest, pos, path);
        for (n, slope) in next {
            slope.iter().for_each(|s| {
                path.insert(*s);
            });
            path.insert(n);
            longest = longest.max(recurse(forest, &n, end, path));
            path.remove(&n);
            slope.iter().for_each(|s| {
                path.remove(s);
            });
        }
        longest
    }
    recurse(&forest, &start, &end, &mut HashSet::from([start]))
}

fn find_longest_2(forest: &mut Forest, start: Coord, end: Coord) -> usize {
    fn next_steps(forest: &Forest, Coord { x, y }: &Coord, visited: &HashSet<Coord>) -> Vec<Coord> {
        let mut next = vec![];
        for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let c = Coord::new(x + dx, y + dy);
            if visited.contains(&c) {
                continue;
            }
            match forest.squares.get(&c) {
                Some(Trail::Path) => next.push(c),
                Some(Trail::Slope(Coord { x: sx, y: sy })) if sx != x || sy != y => {
                    next.push(Coord::new(*sx, *sy));
                }
                None | _ => (),
            }
        }
        next
    }
    fn find_fork_paths(forest: &mut Forest, start: &Coord) -> HashMap<Coord, Vec<(Coord, usize)>> {
        // count steps between forks
        let mut forks: HashMap<Coord, Vec<(Coord, usize)>> = HashMap::new();
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut queue = vec![(*start, *start)];
        while !queue.is_empty() {
            let (prev, node) = queue.pop().unwrap();
            let mut fork_len = 1;
            visited.insert(node);
            let mut pos = node.clone();
            loop {
                let next = next_steps(forest, &pos, &visited);
                if next.len() == 1 {
                    pos = next[0];
                    visited.insert(pos.clone());
                    fork_len += 1;
                } else {
                    let item_fwd = (pos, fork_len);
                    forks
                        .entry(prev)
                        .and_modify(|v| v.push(item_fwd))
                        .or_insert(vec![item_fwd]);
                    let item_bwd = (prev, fork_len);
                    forks
                        .entry(pos)
                        .and_modify(|v| v.push(item_bwd))
                        .or_insert(vec![item_bwd]);
                    for n in next {
                        queue.push((pos, n));
                    }
                    break;
                }
            }
        }
        forks
    }
    let forks = find_fork_paths(forest, &start);
    // find longest path between forks
    let mut queue = vec![(start, 0, HashSet::from([start.clone()]))];
    let mut longest = 0;
    while !queue.is_empty() {
        let (node, current, visited) = queue.pop().unwrap();
        if node == end {
            longest = longest.max(current - 1);
        } else {
            for (n, l) in forks.get(&node).unwrap().iter() {
                if visited.contains(n) {
                    continue;
                }
                let mut v = visited.clone();
                v.insert(*n);
                queue.push((*n, current + *l, v));
            }
        }
    }
    longest
}

pub fn part1() -> usize {
    let forest = input("data/day23.txt");
    let start = Coord::new(1, 0);
    let end = Coord::new(forest.width - 2, forest.height - 1);
    find_longest(forest, start, end)
}

pub fn part2() -> usize {
    let mut forest = input("data/day23.txt");
    forest.squares.values_mut().for_each(|t| *t = Trail::Path);
    let start = Coord::new(1, 0);
    let end = Coord::new(forest.width - 2, forest.height - 1);
    find_longest_2(&mut forest, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1998);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 6434);
    }
}
