use crate::util::load;

enum Square {
    EMPTY,
    FIXED,
    MOVING,
}

type Grid = Vec<Vec<Square>>;

fn input() -> Grid {
    load::<String>("data/day14.txt")
        .into_iter()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Square::EMPTY,
                    '#' => Square::FIXED,
                    'O' => Square::MOVING,
                    _ => unreachable!("Invalid char '{}'", c),
                })
                .collect()
        })
        .collect()
}

pub fn part1() -> usize {
    let grid = input();
    let sz = grid.len();
    let mut load = 0;
    for x in 0..grid[0].len() {
        let mut l = sz;
        for y in 0..sz {
            match grid[y][x] {
                Square::MOVING => {
                    load += l;
                    l -= 1;
                }
                Square::FIXED => {
                    l = sz - y - 1;
                }
                Square::EMPTY => (),
            }
        }
    }
    load
}

pub fn part2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
