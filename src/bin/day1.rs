use std::fs;

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day1.txt").expect("Failed to read file");
    let lines = input.lines();
    let part1: u32 = lines.clone().map(|line| {
        let left = line.chars().find(is_digit).expect("no number found in line");
        let right = line.chars().rev().find(is_digit).expect("no number found in line");
        let left = left as u32 - '0' as u32;
        let right = right as u32 - '0' as u32;
        left * 10 + right
    }).sum();
    dbg!(part1);
}

fn is_digit(c: &char) -> bool {
    c >= &'0' && c <= &'9'
}
