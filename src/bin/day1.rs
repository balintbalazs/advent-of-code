use std::fs;

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day1.txt").expect("Failed to read file");
    let mut elves: Vec<u32> = input
        .split("\r\n\r\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        }).collect();
    elves.sort();
    let top3: u32 = elves.iter().rev().take(3).sum();
    dbg!(top3);
}
