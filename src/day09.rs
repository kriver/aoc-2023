use crate::util::load;

fn input() -> Vec<Vec<i32>> {
    let lines = load::<String>("data/day09.txt");
    lines
        .into_iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn find_next(series: Vec<i32>) -> (i32, i32) {
    fn recurse(s: Vec<i32>) -> (i32, i32) {
        let all_the_same = s[1..]
            .iter()
            .fold((true, s[0]), |(same, prev), n| (same && (prev == *n), *n))
            .0;
        if all_the_same {
            (s[0], *s.last().unwrap())
        } else {
            let (f, l) = recurse(
                s[1..]
                    .into_iter()
                    .fold((vec![], s[0]), |(mut result, prev): (_, _), n| {
                        result.push(prev - n);
                        (result, *n)
                    })
                    .0,
            );
            (s[0] + f, s.last().unwrap() - l)
        }
    }
    recurse(series)
}

pub fn part1() -> i32 {
    let data = input();
    data.into_iter()
        .map(|series| find_next(series))
        .map(|fl| fl.1)
        .sum()
}

pub fn part2() -> i32 {
    let data = input();
    data.into_iter()
        .map(|series| find_next(series))
        .map(|fl| fl.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1743490457);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1053);
    }
}
