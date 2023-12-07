use std::str::FromStr;

use anyhow::anyhow;

use crate::util::read_input;

pub fn day05() {
    problem1();
    problem2();
}

fn problem1() {
    let input = read_input(5, 1);
    let sm = SeedMapper::from_str(&input).unwrap();
    let nearest_location = sm
        .final_locations()
        .iter()
        .fold(u64::MAX, |acc, v| acc.min(*v));
    println!("Nearest Location: {nearest_location}");
}

fn problem2() {
    // let input = read_input(5, 1);
}

#[derive(Debug, Clone)]
struct SeedMapper {
    seeds: Vec<u64>,
    maps: Vec<SeedMap>,
}

impl SeedMapper {
    fn final_locations(&self) -> Vec<u64> {
        self.maps.iter().fold(self.seeds.clone(), |acc, sm| {
            let mut vals = Vec::new();
            for start in acc {
                vals.push(sm.destination(start));
            }
            vals
        })
    }
}

impl FromStr for SeedMapper {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().filter(|l| !l.trim().is_empty());
        let seeds: Vec<u64> = lines
            .next()
            .ok_or(anyhow!("Missing seeds"))?
            .split_once(':')
            .ok_or(anyhow!("Unparseable seeds"))?
            .1
            .trim()
            .split(' ')
            .map(|n| u64::from_str(n))
            .collect::<Result<Vec<_>, _>>()?;

        let mut maps = Vec::new();
        let mut map_lines: Vec<SeedMapLine> = Vec::new();
        for line in lines {
            let line = line.trim();
            if line.ends_with(':') {
                maps.push(SeedMap(std::mem::take(&mut map_lines)));
                continue;
            }
            map_lines.push(SeedMapLine::from_str(line)?);
        }
        map_lines.sort_by_key(|k| k.start);
        maps.push(SeedMap(std::mem::take(&mut map_lines)));
        Ok(SeedMapper { seeds, maps })
    }
}

#[derive(Debug, Clone)]
struct SeedMap(Vec<SeedMapLine>);

impl SeedMap {
    fn destination(&self, start: u64) -> u64 {
        self.0
            .iter()
            .find_map(|l| l.destination(start))
            .unwrap_or(start)
    }
}

impl FromStr for SeedMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sms = s
            .lines()
            .map(|l| SeedMapLine::from_str(l))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(SeedMap(sms))
    }
}

#[derive(Debug, Clone, Copy)]
struct SeedMapLine {
    destination: u64,
    start: u64,
    range: u64,
    counter: u64,
}

impl SeedMapLine {
    fn new(destination: u64, start: u64, range: u64) -> SeedMapLine {
        SeedMapLine {
            destination,
            start,
            range,
            counter: 0,
        }
    }

    fn destination(&self, start: u64) -> Option<u64> {
        if start < self.start || start >= self.start + self.range {
            return None;
        }
        let diff = start - self.start;
        Some(self.destination + diff)
    }
}

impl FromStr for SeedMapLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let destination = split
            .next()
            .ok_or(anyhow!("Missing destination"))?
            .parse()?;
        let start = split.next().ok_or(anyhow!("Missing start"))?.parse()?;
        let range = split.next().ok_or(anyhow!("Missing range"))?.parse()?;
        Ok(SeedMapLine::new(destination, start, range))
    }
}

#[cfg(test)]
mod tests {
    use crate::util::read_example;

    use super::*;

    #[test]
    fn test_destination() {
        let sm = SeedMapLine::from_str("50 98 2").unwrap();
        assert_eq!(sm.destination(98), Some(50));
        assert_eq!(sm.destination(10), None);
    }

    #[test]
    fn test_problem1() {
        let input = read_example(5, 1);
        let mapper = SeedMapper::from_str(&input).unwrap();
        let min = mapper.final_locations();
        assert_eq!(min, &[82, 43, 86, 35]);
    }

    #[test]
    fn test_problem2() {
        // let input = read_example(5, 1);
    }
}
