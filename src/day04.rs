use std::{
    collections::{HashSet, LinkedList},
    str::FromStr,
};

use crate::util::load;

#[derive(Debug)]
pub struct ScratchCard {
    pub id: u32,
    pub winning: HashSet<u32>,
    pub mine: HashSet<u32>,
}

impl FromStr for ScratchCard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split([':', '|']).collect::<Vec<&str>>();
        let id = parts[0]
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let winning = parts[1]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let mine = parts[2]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        Ok(ScratchCard { id, winning, mine })
    }
}

pub fn input() -> Vec<ScratchCard> {
    let values: Vec<String> = load("data/day04.txt");
    values
        .into_iter()
        .map(|x| x.parse::<ScratchCard>().unwrap())
        .collect()
}

fn winning_numbers(scratch_card: ScratchCard) -> HashSet<u32> {
    scratch_card
        .winning
        .intersection(&scratch_card.mine)
        .copied()
        .collect::<HashSet<u32>>()
}

pub fn part1() -> u32 {
    let games = input();
    games
        .into_iter()
        .map(winning_numbers)
        .map(|matches| matches.len() as u32)
        .filter(|count| *count > 0u32)
        .map(|count| 2u32.pow(count - 1))
        .sum()
}

pub fn part2() -> u32 {
    let games = input();
    games
        .into_iter()
        .fold(
            (0u32, LinkedList::<u32>::new()),
            |(num_cards, mut multipliers): (_, _), scratch_card| {
                let winning = winning_numbers(scratch_card);
                // current card multiplier (+1 for the original card)
                let mul = multipliers.pop_front().unwrap_or(0) + 1;
                // increment existing multipliers
                multipliers
                    .iter_mut()
                    .take(winning.len())
                    .for_each(|m| *m += mul);
                // add new multipliers
                (multipliers.len()..winning.len()).for_each(|_n| multipliers.push_back(mul));
                // return accumulator
                (num_cards + mul, multipliers)
            },
        )
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 21213);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 8549735);
    }
}
