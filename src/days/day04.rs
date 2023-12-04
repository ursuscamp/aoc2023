use std::{collections::HashSet, str::FromStr};

use anyhow::anyhow;

use crate::util::read_input;

pub fn day04() {
    problem1();
    problem2();
}

fn problem1() {
    let input = read_input(4, 1);
    let cards: Vec<Card> = input.lines().map(|l| Card::from_str(l).unwrap()).collect();
    let points: u64 = cards.iter().map(Card::points).sum();
    println!("{points}");
}

fn problem2() {
    let input = read_input(4, 2);
}

pub struct Card {
    id: u64,
    winners: HashSet<u64>,
    have: HashSet<u64>,
}

impl Card {
    pub fn points(&self) -> u64 {
        let union = self.winners.intersection(&self.have);
        let c = union.count();
        if c == 0 {
            return 0;
        }
        2u64.pow(c as u32 - 1)
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, hand) = s.split_once(':').ok_or(anyhow!("Invalid format"))?;
        let id: u64 = game
            .strip_prefix("Card")
            .ok_or(anyhow!("Invalid format"))?
            .trim()
            .parse()?;

        let (winners, have) = hand.split_once(" | ").ok_or(anyhow!("Missing hand"))?;
        let winners: anyhow::Result<HashSet<u64>> = winners
            .split(' ')
            .filter(|s| s.len() > 0)
            .map(|n| Ok(n.parse()?))
            .collect();
        let winners = winners?;

        let have: anyhow::Result<HashSet<u64>> = have
            .split(' ')
            .filter(|s| s.len() > 0)
            .map(|n| Ok(n.parse()?))
            .collect();
        let have = have?;

        Ok(Card { id, winners, have })
    }
}

#[cfg(test)]
mod tests {
    use crate::util::read_example;

    use super::*;

    #[test]
    fn test_problem1() {
        let input = read_example(4, 1);
        let cards: u64 = input
            .lines()
            .map(|line| Card::from_str(line).unwrap())
            .map(|c| c.points())
            .sum();
        assert_eq!(cards, 13);
    }

    #[test]
    fn test_problem2() {
        let input = read_example(4, 1);
    }
}
