use crate::util::{Coord2D, Grid};

#[derive(Debug, PartialEq, Eq, Clone)]
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

fn flood_fill(g: &mut Garden, steps: usize, start: Coord) -> usize {
    let deltas: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut q = vec![start];
    for i in 1..=steps {
        let mut new_q = vec![];
        while !q.is_empty() {
            let Coord { x, y } = q.pop().unwrap();
            for (dx, dy) in deltas {
                let c = Coord {
                    x: x + dx,
                    y: y + dy,
                };
                if c.x < 0 || c.y < 0 || c.x >= g.width || c.y >= g.height {
                    continue;
                }
                if let Some(StepCount::UNKNOWN) = g.squares.get(&c) {
                    // unvisited grid point
                    let oe = if i % 2 == 0 {
                        StepCount::EVEN
                    } else {
                        StepCount::ODD
                    };
                    g.squares.insert(c.clone(), oe);
                    new_q.push(c);
                }
            }
        }
        q.append(&mut new_q);
    }
    g.squares
        .iter()
        .filter(|(_, sc)| **sc == StepCount::EVEN)
        .count()
}

/* shape is a diamond
    -  65 steps: touching borders at center of boundaries:
                (65,0), (65,130), (0,65), (130,65)
    - 130 steps: filling full initial grid
    - 132 steps: first step onto corner touching grid (+ 2)
    How does it grow from center and corners?
    Start from corner (0,0)
    - 130 steps: fill half (diagonally)
    - 260 steps: fill completely (130 + 130)
    Start from center (0,65)
    - 132 steps: touch opposite boundary
    - 133 steps: first step off opposite center (+ 1)
    - 195 steps: fill completely (130 + 65)
    - 197 steps: first step onto corner touching grid (+ 2)
*/
fn flood_part2(garden: Garden) -> usize {
    fn steps_for(g: &Garden, steps: usize, c: Coord) -> usize {
        let mut c = g.clone();
        flood_fill(&mut c, steps, Coord2D::new(65, 65))
    }
    let steps = 26501365;
    let completely_filled = steps_for(&garden, 130, Coord2D::new(65, 65));
    println!("center: {:?}", completely_filled);
    println!(
        "center left: {:?}",
        steps_for(&garden, 65, Coord2D::new(0, 65))
    );
    0
}

fn _dump(g: &Garden) {
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
    flood_fill(&mut garden, 64, start)
}

pub fn part2() -> usize {
    let garden = input("data/day21.txt");
    flood_part2(garden)
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
