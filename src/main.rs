mod days;
mod util;

pub use days::*;

fn main() {
    let days = vec![&day01];

    std::env::args().skip(1).for_each(|arg| {
        let day = arg.parse::<usize>().unwrap();
        println!("--- Day {day:02} ---");
        days[day - 1]();
        println!("")
    });
}
