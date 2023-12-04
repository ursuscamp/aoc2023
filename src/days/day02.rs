use std::str::FromStr;

use crate::util::read_input;

pub fn day02() {
    problem1();
    problem2();
}

fn problem1() {
    let input = read_input(2, 1);
    let values = input
        .lines()
        .map(|line| Game::from_str(line))
        .map(Result::unwrap);

    let start: Handful = "12 red, 13 green, 14 blue".parse().unwrap();

    let result: u64 = values
        .filter(|game| game.is_possible(start.clone()))
        .map(|game| game.id)
        .sum();
    println!("{result}");
}

fn problem2() {
    let input = read_input(2, 2);
    let values = input
        .lines()
        .map(|line| Game::from_str(line))
        .map(Result::unwrap);

    let result: u64 = values.map(|g| g.min_color_set().power()).sum();
    println!("{result}");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => anyhow::bail!("Invalid color {s}"),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Handful {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

impl Handful {
    pub fn contains(&self, other: &Handful) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    pub fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }

    pub fn aggregate_min(&mut self, other: &Handful) {
        let Handful { red, green, blue } = other;
        self.red = self.red.max(*red);
        self.blue = self.blue.max(*blue);
        self.green = self.green.max(*green);
    }
}

impl FromStr for Handful {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let results = s
            .split(", ")
            .map(|s| s.trim())
            .map(|s| {
                s.split_once(' ').map(|(count, color)| {
                    (
                        u64::from_str_radix(count, 10).unwrap(),
                        Color::from_str(color).unwrap(),
                    )
                })
            })
            .map(Option::unwrap);
        let mut handful = Handful::default();
        for (count, color) in results {
            match color {
                Color::Red => handful.red += count,
                Color::Green => handful.green += count,
                Color::Blue => handful.blue += count,
            }
        }

        Ok(handful)
    }
}

#[derive(Debug)]
pub struct Game {
    id: u64,
    results: Vec<Handful>,
}

impl Game {
    pub fn is_possible(&self, totals: Handful) -> bool {
        for handful in &self.results {
            if !totals.contains(handful) {
                return false;
            }
        }
        true
    }

    pub fn min_color_set(&self) -> Handful {
        let mut min_set = Handful::default();
        for handful in &self.results {
            min_set.aggregate_min(handful);
        }
        min_set
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, handfuls) = s.split_once(':').unwrap();
        let id: u64 = game_id.strip_prefix("Game ").unwrap().parse().unwrap();
        let results: Vec<Handful> = handfuls
            .split("; ")
            .map(|h| Handful::from_str(h).unwrap())
            .collect();
        Ok(Game { id, results })
    }
}
