pub fn read_input(day: usize, problem: usize) -> String {
    std::fs::read_to_string(format!("inputs/day{:02}/problem{:02}.txt", day, problem)).unwrap()
}
