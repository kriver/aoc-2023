use crate::util::{Coord2D, Grid};

#[derive(Debug, PartialEq, Eq)]
enum StepCount {
    ODD,
    EVEN,
    UNKNOWN,
}

type Coord = Coord2D<i32>;
type Garden = Grid<i32, StepCount>;

fn input(file: &str) -> Garden {
    Grid::from_file(file, |c| match c {
        '#' => None,
        '.' => Some(StepCount::UNKNOWN),
        'S' => Some(StepCount::EVEN),
        _ => None,
    })
}

fn find_start(g: &Garden) -> Coord {
    g.squares
        .iter()
        .find(|(_, sc)| **sc == StepCount::EVEN)
        .map(|(c, _)| c.clone())
        .unwrap()
}

fn flood_fill(g: &mut Garden, steps: usize, start: Coord, finite: bool) {
    let deltas: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut q = vec![start];
    for i in 1..=steps {
        let mut newq = vec![];
        while !q.is_empty() {
            let Coord { x, y } = q.pop().unwrap();
            for (dx, dy) in deltas {
                let c = Coord {
                    x: x + dx,
                    y: y + dy,
                };
                if finite && (c.x < 0 || c.y < 0 || c.x >= g.width || c.y >= g.height) {
                    continue;
                }
                let small_c = Coord {
                    x: c.x.rem_euclid(g.width),
                    y: c.y.rem_euclid(g.height),
                };
                if let Some(_) = g.squares.get(&small_c) {
                    // valid grid point
                    match g.squares.get(&c) {
                        None | Some(StepCount::UNKNOWN) => {
                            // unvisited grid point
                            let oe = if i % 2 == 0 {
                                StepCount::EVEN
                            } else {
                                StepCount::ODD
                            };
                            g.squares.insert(c.clone(), oe);
                            newq.push(c);
                        }
                        _ => (),
                    }
                }
            }
        }
        // println!("After {} steps:", i);
        q.append(&mut newq);
        // println!("q: {:?}", q);
        // dump(g);
    }
}

fn dump(g: &Garden) {
    (0..g.height).into_iter().for_each(|y| {
        print!("\t");
        (0..g.width).into_iter().for_each(|x| {
            let c = Coord2D { x, y };
            match g.squares.get(&c) {
                None => print!("#"),
                Some(StepCount::UNKNOWN) => print!("."),
                Some(StepCount::ODD) => print!("o"),
                Some(StepCount::EVEN) => print!("e"),
            }
        });
        println!();
    });
}

pub fn part1() -> usize {
    let mut garden = input("data/day21.txt");
    let start = find_start(&garden);
    flood_fill(&mut garden, 64, start, true);
    garden
        .squares
        .into_iter()
        .filter(|(_, sc)| *sc == StepCount::EVEN)
        .count()
}

pub fn part2() -> usize {
    let mut garden = input("data/test.txt");
    let start = find_start(&garden);
    flood_fill(&mut garden, 1000, start, false);
    garden
        .squares
        .into_iter()
        .filter(|(_, sc)| *sc == StepCount::EVEN)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 3658);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
