use std::fs;

fn next_in_sequence(seq: &Vec<i64>) -> i64 {
    let mut diffs = Vec::new();
    for i in 1..seq.len() {
        diffs.push(seq[i] - seq[i - 1]);
    }
    let next_diff = if diffs.iter().all(|d| *d == 0) {
        0
    } else {
        next_in_sequence(&diffs)
    };

    seq[seq.len() - 1] + next_diff
}

fn prev_in_sequence(seq: &Vec<i64>) -> i64 {
    let mut diffs = Vec::new();
    for i in 1..seq.len() {
        diffs.push(seq[i] - seq[i - 1]);
    }
    let prev_diff = if diffs.iter().all(|d| *d == 0) {
        0
    } else {
        prev_in_sequence(&diffs)
    };

    seq[0] - prev_diff
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day9.txt").expect("Failed to read file");
    let sequences = input.lines().map(|line| {
        let nums: Vec<i64> = line.split(' ').map(|n| n.parse::<i64>().unwrap()).collect();
        nums
    });

    let part1 = sequences
        .clone()
        .map(|sequence| next_in_sequence(&sequence))
        .sum::<i64>();

    dbg!(part1);

    let part2 = sequences
        .map(|sequence| prev_in_sequence(&sequence))
        .sum::<i64>();

    dbg!(part2);
}
