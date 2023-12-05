use std::str::FromStr;

use crate::util::load;

type Range = (u64, u64);

#[derive(Debug, Clone)]
pub struct MapRange {
    pub src: u64,
    pub src_end: u64,
    pub dst: u64,
    pub len: u64,
}

impl FromStr for MapRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<u64> = s
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        Ok(MapRange {
            src: nums[1],
            src_end: nums[1] + nums[2] - 1,
            dst: nums[0],
            len: nums[2],
        })
    }
}

impl MapRange {
    pub fn map(&self, id: u64) -> Option<u64> {
        if id >= self.src && id < (self.src + self.len) {
            Some(self.dst + (id - self.src))
        } else {
            None
        }
    }

    // returns an optional mapped range, and a list of unmapped ranges
    pub fn map_range(&self, (start, len): &Range) -> (Option<Range>, Vec<Range>) {
        let end = start + len - 1;
        // fully before or after
        if end < self.src || *start > self.src_end {
            return (None, vec![(*start, *len)]);
        }
        // part before
        let before = if *start < self.src {
            Some((*start, self.src - start))
        } else {
            None
        };
        // part after
        let after = if end > self.src_end {
            Some((self.src_end + 1, end - self.src_end))
        } else {
            None
        };
        // overlapping part (unmapped)
        let new_start = if before.is_some() { self.src } else { *start };
        let new_end = if after.is_some() { self.src_end } else { end };
        (
            Some((self.dst + new_start - self.src, new_end - new_start + 1)),
            vec![before, after].into_iter().filter_map(|r| r).collect(),
        )
    }
}

pub fn input() -> (Vec<u64>, Vec<Vec<MapRange>>) {
    let lines: Vec<String> = load("data/day05.txt");
    let seeds = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut maps = vec![];
    let mut map = vec![];
    for line in lines[2..].iter() {
        if line.starts_with(|c: char| c.is_alphabetic()) {
            continue; // skip over name line
        }
        if line.is_empty() {
            maps.push(map.clone());
            map.clear()
        } else {
            map.push(line.parse::<MapRange>().unwrap());
        }
    }
    maps.push(map);
    (seeds, maps)
}

fn apply_map_range(src: &Vec<Range>, mr: &MapRange) -> (Vec<Range>, Vec<Range>) {
    src.into_iter()
        .fold((vec![], vec![]), |(mut mapped, mut unmapped), r| {
            let (extra_mapped, mut extra_unmapped) = mr.map_range(r);
            extra_mapped.map(|m| mapped.push(m));
            unmapped.append(&mut extra_unmapped);
            (mapped, unmapped)
        })
}

pub fn part1() -> u64 {
    let (seeds, maps) = input();
    seeds
        .into_iter()
        .map(|s| {
            maps.iter().fold(s, |id, map| {
                let mut acc = id;
                for range in map.iter() {
                    if let Some(new_id) = range.map(id) {
                        acc = new_id;
                        break;
                    }
                }
                acc
            })
        })
        .min()
        .unwrap()
}

pub fn part2() -> u64 {
    let (seeds, mappings) = input();
    let pairs: Vec<Range> = seeds.chunks(2).map(|x| (x[0], x[1])).collect();
    mappings
        .into_iter()
        .fold(pairs, |ranges, mapping| {
            let (mut unmapped, mut mapped) = mapping.into_iter().fold(
                (ranges, vec![]),
                |(src_ranges, mut dst_ranges): (Vec<Range>, Vec<Range>), map_range| {
                    let (mut mapped, unmapped) = apply_map_range(&src_ranges, &map_range);
                    dst_ranges.append(&mut mapped);
                    (unmapped, dst_ranges)
                },
            );
            mapped.append(&mut unmapped);
            mapped
        })
        .into_iter()
        .map(|(start, _len)| start)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 579439039);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 7873084);
    }
}
