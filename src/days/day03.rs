use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    str::FromStr,
};

use crate::util::read_input;

pub fn day03() {
    problem1();
    problem2();
}

fn problem1() {
    let input = read_input(3, 1);
    let schematic = Schematic::from_str(&input).unwrap();
    let part_num_sum: u64 = schematic.part_numbers().into_iter().sum();
    println!("{part_num_sum}");
}

fn problem2() {
    let input = read_input(3, 2);
    let schematic = Schematic::from_str(&input).unwrap();
    let gear_ratios: u64 = schematic
        .gear_ratios()
        .into_iter()
        .map(|(a, b)| a * b)
        .sum();
    println!("{gear_ratios}");
}

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Nothing,
    Number(char),
    Symbol(char),
}

impl Cell {
    pub fn from_char(s: char) -> Cell {
        match s {
            '.' => Cell::Nothing,
            n @ '0'..='9' => Cell::Number(n),
            s => Cell::Symbol(s),
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Cell::Number(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pair(usize, usize);

impl Pair {
    pub fn new(x: usize, y: usize) -> Self {
        Pair(x, y)
    }

    pub fn adjacent_dir(&self, dir: Dir) -> Option<Pair> {
        let x = match dir {
            Dir::Up => self.0,
            Dir::Down => self.0,
            Dir::Left => self.0.checked_sub(1)?,
            Dir::Right => self.0 + 1,
            Dir::TopLeft => self.0.checked_sub(1)?,
            Dir::TopRight => self.0 + 1,
            Dir::BottomLeft => self.0.checked_sub(1)?,
            Dir::BottomRight => self.0 + 1,
        };

        let y = match dir {
            Dir::Up => self.1.checked_sub(1)?,
            Dir::Down => self.1 + 1,
            Dir::Left => self.1,
            Dir::Right => self.1,
            Dir::TopLeft => self.1.checked_sub(1)?,
            Dir::TopRight => self.1.checked_sub(1)?,
            Dir::BottomLeft => self.1 + 1,
            Dir::BottomRight => self.1 + 1,
        };

        Some(Pair::new(x, y))
    }

    pub fn adjacent(&self) -> HashSet<Pair> {
        [
            Dir::Up,
            Dir::Down,
            Dir::Left,
            Dir::Right,
            Dir::TopLeft,
            Dir::TopRight,
            Dir::BottomLeft,
            Dir::BottomRight,
        ]
        .into_iter()
        .map(|dir| self.adjacent_dir(dir))
        .filter_map(|p| p)
        .collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Schematic {
    grid: Vec<Vec<Cell>>,
}

impl Schematic {
    pub fn is_symbol(&self, coord: Pair) -> bool {
        let line = match self.grid.get(coord.1) {
            Some(line) => line,
            None => return false,
        };
        match line.get(coord.0) {
            Some(Cell::Symbol(_)) => true,
            _ => false,
        }
    }

    pub fn is_asterisk(&self, coord: Pair) -> bool {
        let line = match self.grid.get(coord.1) {
            Some(line) => line,
            None => return false,
        };
        match line.get(coord.0) {
            Some(Cell::Symbol('*')) => true,
            _ => false,
        }
    }

    pub fn part_numbers(&self) -> Vec<u64> {
        let mut parts = Vec::new();
        for (y, line) in self.grid.iter().enumerate() {
            let mut digits = Vec::new();
            let mut symbol_adjacent = false;
            for (x, cell) in line.iter().enumerate() {
                if let Cell::Number(n) = cell {
                    digits.push(n);
                    let adjacent = Pair::new(x, y).adjacent();
                    let ad = adjacent.iter().any(|p| self.is_symbol(*p));
                    symbol_adjacent = ad || symbol_adjacent;
                    continue;
                }
                if symbol_adjacent && digits.len() > 0 {
                    let s: String = digits.iter().map(Deref::deref).collect();
                    let n: u64 = s.parse().unwrap();
                    parts.push(n);
                }
                digits.clear();
                symbol_adjacent = false;
            }
            if symbol_adjacent && digits.len() > 0 {
                let s: String = digits.iter().map(Deref::deref).collect();
                let n: u64 = s.parse().unwrap();
                parts.push(n);
            }
            digits.clear();
        }
        return parts;
    }

    pub fn gear_ratios(&self) -> Vec<(u64, u64)> {
        let mut parts = Vec::new();
        for (y, line) in self.grid.iter().enumerate() {
            let mut digits = Vec::new();
            let mut asterisks = HashSet::new();
            for (x, cell) in line.iter().enumerate() {
                if let Cell::Number(n) = cell {
                    digits.push(n);
                    let adjacent = Pair::new(x, y).adjacent();
                    for possible in adjacent.iter() {
                        if self.is_asterisk(*possible) {
                            asterisks.insert(*possible);
                        }
                    }
                    continue;
                }
                if asterisks.len() > 0 && digits.len() > 0 {
                    let s: String = digits.iter().map(Deref::deref).collect();
                    let n: u64 = s.parse().unwrap();
                    for asterisk in asterisks.clone() {
                        parts.push((asterisk, n));
                    }
                }
                digits.clear();
                asterisks.clear();
            }
            if asterisks.len() > 0 && digits.len() > 0 {
                let s: String = digits.iter().map(Deref::deref).collect();
                let n: u64 = s.parse().unwrap();
                for asterisk in asterisks {
                    parts.push((asterisk, n));
                }
            }
            digits.clear();
        }
        let mut parts_map = HashMap::new();
        for (asterisk, number) in parts {
            let v: &mut Vec<u64> = parts_map
                .entry(asterisk)
                .or_insert_with(|| Vec::new())
                .as_mut();
            v.push(number);
        }

        let mut parts = Vec::new();
        for (_asterisk, ast_parts) in parts_map {
            if ast_parts.len() == 2 {
                parts.push((ast_parts[0], ast_parts[1]));
            }
        }
        return parts;
    }
}

impl FromStr for Schematic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut schematic = Schematic::default();
        for line in s.lines() {
            let mut schematic_line = Vec::new();
            for ch in line.chars() {
                schematic_line.push(Cell::from_char(ch));
            }
            schematic.grid.push(schematic_line);
        }
        Ok(schematic)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::read_example;

    use super::*;

    #[test]
    fn test_problem1() {
        let input = read_example(3, 1);
        let schematic: Schematic = input.parse().unwrap();
        let part_number_sum: u64 = schematic.part_numbers().iter().sum();
        println!("{schematic:?}");
        assert_eq!(part_number_sum, 4361);
    }

    #[test]
    fn test_problem2() {
        let input = read_example(3, 1);
        let schematic = Schematic::from_str(&input).unwrap();
        let gear_ratios = schematic.gear_ratios();
        println!("{gear_ratios:?}");
        let total: u64 = gear_ratios.into_iter().map(|(a, b)| a * b).sum();
        assert_eq!(total, 467835);
    }
}
