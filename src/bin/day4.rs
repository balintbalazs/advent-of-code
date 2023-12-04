use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day4.txt").expect("Failed to read file");
    let lines = input.lines();

    let part1 = lines
        .clone()
        .map(|line| {
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

            let score = if count > 0 { 1usize << (count - 1) } else { 0 };
            score
        })
        .sum::<usize>();

    dbg!(part1);

    let mut number_of_cards = HashMap::<usize, usize>::new();
    lines.for_each(|line| {
        let (card_id, numbers) = line.split_once(':').unwrap();
        let card_id: usize = card_id.split_once(' ').unwrap().1.trim().parse().unwrap();

        if let Some(count) = number_of_cards.get_mut(&card_id) {
            *count += 1;
        } else {
            number_of_cards.insert(card_id, 1);
        }

        let (winning_numbers, numbers) = numbers.split_once('|').unwrap();

        let winning_numbers = winning_numbers
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        let win_count = numbers
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .filter(|num| winning_numbers.get(num).is_some())
            .count();

        let multiplier = *number_of_cards.get(&card_id).unwrap();

        for copy_id in (card_id + 1)..(card_id + 1 + win_count) {
            if let Some(num) = number_of_cards.get_mut(&copy_id) {
                *num += multiplier;
            } else {
                number_of_cards.insert(copy_id, multiplier);
            }
        }
    });
    let part2: usize = number_of_cards.into_values().sum();
    dbg!(part2);
}
