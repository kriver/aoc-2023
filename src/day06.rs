type TimeDistance = (f64, f64);

// const TEST: [TimeDistance; 3] = [(7.0, 9.0), (15.0, 40.0), (30.0, 200.0)];
const DATA: [TimeDistance; 4] = [
    (48.0, 261.0),
    (93.0, 1192.0),
    (84.0, 1019.0),
    (66.0, 1063.0),
];

fn possible_wins(td: TimeDistance) -> u64 {
    let discriminant = td.0.powi(2) - 4.0 * td.1;
    let r1 = (td.0 - discriminant.sqrt()) / 2.0;
    let r2 = (td.0 + discriminant.sqrt()) / 2.0;
    // need next integer larger/smaller even if already integer
    let i1 = (r1 + 1.0).floor() as u64;
    let i2 = (r2 - 1.0).ceil() as u64;
    i2 - i1 + 1
}

pub fn part1() -> u64 {
    DATA.into_iter().map(|td| possible_wins(td)).product()
}

pub fn part2() -> u64 {
    // Need 64-bit numbers for precision
    possible_wins((48938466.0, 261119210191063.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1312850);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 36749103);
    }
}
