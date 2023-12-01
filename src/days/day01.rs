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
    todo!()
}
