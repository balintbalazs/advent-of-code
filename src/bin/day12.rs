use std::fs;

fn count_combinations(line: &str) -> usize {
    let (records, numbers) = line.split_once(' ').unwrap();
    let numbers: Vec<usize> = numbers.split(',').map(|n| n.parse().unwrap()).collect();
    // dbg!(records);
    // dbg!(&numbers);
    let counts = check_records(records, &numbers);
    // dbg!(counts);
    counts
}

fn check_records(records: &str, counts: &Vec<usize>) -> usize {
    if records.contains('?') {
        let a = records.replacen('?', ".", 1);
        let b = records.replacen('?', "#", 1);
        check_records(&a, counts) + check_records(&b, counts)
    } else if *counts
        == records
            .split('.')
            .filter(|a| !a.is_empty())
            .map(|a| a.len())
            .collect::<Vec<_>>()
    {
        1
    } else {
        0
    }
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day12.txt").expect("Failed to read file");
    let lines = input.lines();

    let part1: usize = lines.map(count_combinations).sum();

    dbg!(part1);
}
