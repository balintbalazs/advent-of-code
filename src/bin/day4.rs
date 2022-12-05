use std::fs;

use scan_fmt::scan_fmt;

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day4.txt").expect("Failed to read file");
    let fully_overlapping_pairs = input
        .lines()
        .map(|line| scan_fmt!(line, "{d}-{d},{d}-{d}", i32, i32, i32, i32).unwrap())
        .filter(|(t1s, t1f, t2s, t2f)| (t1s <= t2s && t1f >= t2f) || (t2s <= t1s && t2f >= t1f))
        .count();
    dbg!(fully_overlapping_pairs);

    let overlapping_pairs = input
        .lines()
        .map(|line| scan_fmt!(line, "{d}-{d},{d}-{d}", i32, i32, i32, i32).unwrap())
        .filter(|(t1s, t1f, t2s, t2f)| (t1s <= t2s && t1f >= t2s) || (t2s <= t1s && t2f >= t1s))
        .count();
    dbg!(overlapping_pairs);
}
