use crate::util::load;

#[derive(Debug)]
struct Grid {
    horz: Vec<u32>,
    vert: Vec<u32>,
}

fn input() -> Vec<Grid> {
    let mut grids = vec![];
    let last = load::<String>("data/day13.txt").into_iter().fold(
        (vec![], vec![]),
        |(mut horz, vert), l| {
            if l == "" {
                grids.push(Grid { horz, vert });
                (vec![], vec![])
            } else {
                let (h, v) = l
                    .chars()
                    .enumerate()
                    .fold((0u32, vert), |(mut h, mut v), (i, c)| {
                        if i >= v.len() {
                            v.push(0);
                        }
                        h <<= 1;
                        v[i] <<= 1;
                        if c == '#' {
                            h |= 1;
                            v[i] |= 1
                        }
                        (h, v)
                    });
                horz.push(h);
                (horz, v)
            }
        },
    );
    grids.push(Grid {
        horz: last.0,
        vert: last.1,
    });
    grids
}

fn find_mirror(n: Vec<u32>, part2: bool) -> usize {
    fn num_different_bits(a: u32, b: u32) -> u32 {
        let mut n = a ^ b;
        let mut nz = 0;
        while n > 0 {
            nz += n & 1;
            n >>= 1;
        }
        nz
    }

    let mut prev = n[0];
    for i in 1..n.len() {
        let mut nzb = num_different_bits(prev, n[i]);
        if nzb <= 1 {
            for j in 0..(i - 1) {
                if 2 * i - j - 1 < n.len() {
                    nzb += num_different_bits(n[j], n[2 * i - j - 1]);
                    if nzb > 1 {
                        break;
                    }
                }
            }
            if (!part2 && nzb == 0) || (part2 && nzb == 1) {
                return i;
            }
        }
        prev = n[i]
    }
    0 // no mirror found
}

fn find_mirrors(grid: Grid, part2: bool) -> usize {
    100 * find_mirror(grid.horz, part2) + find_mirror(grid.vert, part2)
}

pub fn part1() -> usize {
    input().into_iter().map(|g| find_mirrors(g, false)).sum()
}

pub fn part2() -> usize {
    input().into_iter().map(|g| find_mirrors(g, true)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 27202);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 41566);
    }
}
