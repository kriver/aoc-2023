use regex::Regex;

use crate::util::load;

pub fn input() -> Vec<String> {
    let values: Vec<String> = load("data/day01.txt");
    values
}

pub fn char2num(c: Option<u8>) -> u8 {
    c.map(|ascii| ascii - '0' as u8).unwrap()
}

pub fn part1(values: Vec<String>) -> u32 {
    let re = Regex::new(r"[^0-9]").unwrap();
    values
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|v| {
            let stripped = re.replace_all(&v, "");
            let mut digits = stripped.bytes();
            if digits.len() > 1 {
                (char2num(digits.next()) * 10 + char2num(digits.last())) as u32
            } else {
                (char2num(digits.next()) * 11) as u32
            }
        })
        .sum()
}

pub fn str2num(s: &str) -> u32 {
    match s {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => unreachable!("Invalid input"),
    }
}

pub fn part2(values: Vec<String>) -> u32 {
    let first = r"[0-9]|one|two|three|four|five|six|seven|eight|nine";
    let last = format!(".*({})", first);
    let first_re = Regex::new(first).expect("regex");
    let last_re = Regex::new(&last).expect("regex");
    values
        .into_iter()
        .map(|v| {
            let tens = str2num(first_re.find(&v).expect("match").as_str());
            let units = str2num(
                last_re
                    .captures(&v)
                    .expect("captures")
                    .get(1)
                    .expect("match")
                    .as_str(),
            );
            tens * 10 + units
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 55712);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 55413);
    }
}
