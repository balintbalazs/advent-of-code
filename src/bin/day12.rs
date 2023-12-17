use std::{collections::HashMap, fs};

fn count_combinations(line: &str, reps: usize) -> usize {
    let (records, numbers) = line.split_once(' ').unwrap();
    let numbers: Vec<usize> = numbers.split(',').map(|n| n.parse().unwrap()).collect();
    let mut records = records.to_string();
    records.push('?');

    let mut records = records.repeat(reps);
    records.pop();
    let numbers = numbers.repeat(reps);
    // dbg!(&records);
    // dbg!(&numbers);
    let mut cache = HashMap::new();
    let counts = check_records(&records, &numbers, 0, &mut cache);
    // dbg!(counts);
    counts
}

fn check_records(
    records: &str,
    counts: &[usize],
    current_count: usize,
    cache: &mut HashMap<(String, Vec<usize>, usize), usize>,
) -> usize {
    // dbg!(&cache);
    if let Some(count) = cache.get(&(records.to_string(), counts.to_owned(), current_count)) {
        return *count;
    }
    // dbg!(records, counts, current_count);
    if counts.is_empty() {
        return if records.chars().all(|ch| ch == '.' || ch == '?') {
            // dbg!("return 1");
            1
        } else {
            // dbg!("return 0");
            0
        };
    }
    if records.is_empty() {
        if counts.len() == 1 && counts[0] == current_count {
            // dbg!("return 1");
            return 1;
        }
        // dbg!("return 0");
        return 0;
    }
    let first = records.chars().next().unwrap();
    let res = match first {
        '.' => {
            let counts = if current_count == counts[0] {
                &counts[1..]
            } else if current_count != 0 {
                // dbg!("return 0");
                return 0;
            } else {
                counts
            };
            check_records(&records[1..], counts, 0, cache)
        }
        '#' => {
            if current_count == counts[0] {
                // dbg!("return 0");
                0
            } else {
                check_records(&records[1..], counts, current_count + 1, cache)
            }
        }
        '?' => {
            let dot = {
                let mut records = records.to_owned();
                records.replace_range(0..1, ".");
                check_records(&records, counts, current_count, cache)
            };

            let hash = {
                let mut records_b = records.to_owned();
                records_b.replace_range(0..1, "#");
                check_records(&records_b, counts, current_count, cache)
            };

            dot + hash
        }
        ch => panic!("unknown symbol {ch} in records"),
    };
    cache.insert((records.to_string(), counts.to_owned(), current_count), res);

    // println!();
    res
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day12.txt").expect("Failed to read file");
    let lines = input.lines();

    let part1: usize = lines.clone().map(|line| count_combinations(line, 1)).sum();
    dbg!(part1);
    let part2: usize = lines.clone().map(|line| count_combinations(line, 5)).sum();
    dbg!(part2);
}
