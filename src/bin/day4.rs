use std::{collections::HashSet, fs};

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day4.txt").expect("Failed to read file");
    let lines = input.lines();

    let part1 = lines.map(|line| {
        let (_, numbers) = line.split_once(':').unwrap();
        let (winning_numbers, numbers) = numbers.split_once('|').unwrap();

        let winning_numbers = winning_numbers
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        let count = numbers
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .filter(|num| winning_numbers.get(num).is_some())
            .count();

        let score = if count > 0 { 1usize << (count - 1)} else { 0 };
        score
    }).sum::<usize>();

    dbg!(part1);
}
