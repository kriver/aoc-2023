use std::{collections::HashSet, iter::repeat, str::FromStr};

use itertools::Itertools;

use crate::util::load;

#[derive(Debug)]
struct Springs {
    pattern: Vec<char>,
    ranges: Vec<usize>,
}

impl FromStr for Springs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        let ranges = tokens[1]
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Ok(Springs {
            pattern: tokens[0].chars().collect(),
            ranges,
        })
    }
}

impl Springs {
    fn arrangements(&self) -> HashSet<String> {
        fn can_fit(pat: &[char], dmgd: usize) -> bool {
            dmgd == pat
                .iter()
                .take(dmgd)
                .filter(|c| **c == '?' || **c == '#')
                .count()
        }

        fn fit_dmgd(res: &mut HashSet<String>, pat: &[char], dmgd: &[usize], cur: &mut Vec<char>) {
            if dmgd.is_empty() {
                return;
            }
            // check whether dmgd[0] can fit here
            let range = dmgd[0];
            let there_is_more = dmgd.len() > 1;
            if can_fit(pat, range)
                && (!there_is_more || (pat.len() > range + 1 && pat[range] != '#'))
            {
                (0..range).for_each(|_| cur.push('#'));
                let mut start = range;
                if there_is_more {
                    // also ensure we have a '.' following if more to fill
                    cur.push('.');
                    start += 1;
                }
                recurse(res, &pat[start..], &dmgd[1..], cur);
                if there_is_more {
                    cur.pop();
                }
                (0..range).for_each(|_| {
                    cur.pop();
                });
            }
        }

        fn recurse(res: &mut HashSet<String>, pat: &[char], dmgd: &[usize], cur: &mut Vec<char>) {
            if pat.is_empty() {
                if dmgd.is_empty() {
                    res.insert(cur.iter().collect());
                }
            } else {
                match pat[0] {
                    '.' => {
                        // skip forward
                        cur.push('.');
                        recurse(res, &pat[1..], dmgd, cur);
                        cur.pop();
                    }
                    '#' => fit_dmgd(res, pat, dmgd, cur),
                    '?' => {
                        // try operational
                        cur.push('.');
                        recurse(res, &pat[1..], dmgd, cur);
                        cur.pop();
                        // try damaged
                        fit_dmgd(res, pat, dmgd, cur);
                    }
                    _ => unreachable!(),
                }
            }
        }
        let mut results = HashSet::new();
        // println!("{:?}", self.pattern);
        recurse(&mut results, &self.pattern, &self.ranges, &mut vec![]);
        // println!("{:?}", results);
        results
    }

    pub fn count_arrangements(&self) -> usize {
        self.arrangements().len()
    }
}

fn unfold(s: Springs) -> Springs {
    Springs {
        pattern: repeat(s.pattern).take(5).collect_vec().join(&'?'),
        ranges: repeat(s.ranges).take(5).flatten().collect_vec(),
    }
}

fn input() -> Vec<Springs> {
    load::<String>("data/day12.txt")
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn part1() -> usize {
    let springs = input();
    springs.into_iter().map(|s| s.count_arrangements()).sum()
}

pub fn part2() -> usize {
    let springs = input();
    springs
        .into_iter()
        .map(|s| unfold(s))
        .map(|s| s.count_arrangements())
        .sum()
}

pub fn arrangements_for(springs: &str) -> usize {
    springs.parse::<Springs>().unwrap().count_arrangements()
}

pub fn unfolded_arrangements_for(springs: &str) -> usize {
    unfold(springs.parse::<Springs>().unwrap()).count_arrangements()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrangements() {
        assert_eq!(arrangements_for("???.### 1,1,3"), 1);
        assert_eq!(arrangements_for(".??..??...?##. 1,1,3"), 4);
        assert_eq!(arrangements_for("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(arrangements_for("????.#...#... 4,1,1"), 1);
        assert_eq!(arrangements_for("????.######..#####. 1,6,5"), 4);
        assert_eq!(arrangements_for("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn test_arrangement_unfolded() {
        assert_eq!(unfolded_arrangements_for("???.### 1,1,3"), 1);
        assert_eq!(unfolded_arrangements_for(".??..??...?##. 1,1,3"), 16384);
        assert_eq!(unfolded_arrangements_for("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(unfolded_arrangements_for("????.#...#... 4,1,1"), 16);
        assert_eq!(unfolded_arrangements_for("????.######..#####. 1,6,5"), 2500);
        assert_eq!(unfolded_arrangements_for("?###???????? 3,2,1"), 506250);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 8193);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(), 0);
    // }
}
