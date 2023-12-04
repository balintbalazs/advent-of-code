use std::fs;

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day4.txt").expect("Failed to read file");
    let lines = input.lines();

    let part1 = lines.map(|line| {
        let (_, numbers) = line.split_once(':').unwrap();
        let (winning, ticket) = numbers.split_once('|').unwrap();
        
        todo!()
    });

    todo!()
}