use crate::util::read_input;

pub fn day01() {
    problem1();
    problem2();
}

fn problem1() {
    let input = read_input(1, 1);
    let mut numbers: Vec<u64> = Vec::new();
    for line in input.lines() {
        let mut chars = line.chars().filter(|c| c.is_digit(10));
        let first = chars.next().unwrap();
        let n1 = first.to_digit(10).unwrap() as u64 * 10;
        let n2 = chars.last().unwrap_or(first).to_digit(10).unwrap() as u64;
        numbers.push(n1 + n2);
    }
    println!("{}", numbers.iter().sum::<u64>());
}

fn problem2() {
    let number_words = [
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ];

    let input = read_input(1, 2);
    let mut numbers: Vec<u64> = Vec::new();
    for line in input.lines() {
        let mut l = line;
        let mut first = None;
        let mut last = None;
        while l.len() > 0 {
            let mut found = false;
            for (word, number) in number_words.iter() {
                if let Some(rest) = l.strip_prefix(word) {
                    if first.is_none() {
                        first = Some(*number);
                        l = rest;
                        found = true;
                        break;
                    } else {
                        last = Some(*number);
                    }
                }
            }
            if !found {
                l = &l[1..];
            }
        }

        let n1 = first.unwrap() * 10;
        let n2 = last.or(first).unwrap();
        numbers.push(n1 + n2);
    }
    println!("{}", numbers.iter().sum::<u64>());
}
