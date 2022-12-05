use std::{collections::{HashSet, btree_set::Intersection}, fs};

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day3.txt").expect("Failed to read file");
    let sum_priorities: u32 = input
        .lines()
        .map(|items| {
            let len = items.len() / 2;
            let first_half = items[0..len].as_bytes();
            let second_half = items[len..].as_bytes();
            for item in first_half {
                if second_half.contains(item.into()) {
                    return item;
                }
            }
            &0 // if there is no overlap
        })
        .map(|item| match item {
            97..=122 => *item as u32 - 96,     // a-z
            65..=90 => *item as u32 - 64 + 26, // A-Z
            _ => 0,
        })
        .sum();
    dbg!(sum_priorities);

    let mut elf0 = HashSet::new();
    let mut elf1 = HashSet::new();
    let mut elf2 = HashSet::new();
    let mut badges = Vec::new();
    for (i, items) in input.lines().enumerate() {
        match i % 3 {
            0 => {
                for item in items.as_bytes() {
                    elf0.insert(item);
                }
            }
            1 => {
                for item in items.as_bytes() {
                    elf1.insert(item);
                }
            }
            2 => {
                for item in items.as_bytes() {
                    elf2.insert(item);
                }
                let inter: HashSet<&u8> = elf0.intersection(&elf1).map(|i| i.clone()).collect();
                let badge = **inter.intersection(&elf2).next().unwrap();
                badges.push(badge);
                elf0.clear();
                elf1.clear();
                elf2.clear();
            }
            _ => unimplemented!(),
        }
    }
    let badge_prios: u32 = badges
        .iter()
        .map(|item| match item {
            97..=122 => *item as u32 - 96,     // a-z
            65..=90 => *item as u32 - 64 + 26, // A-Z
            _ => 0,
        })
        .sum();
    dbg!(badge_prios);
}
