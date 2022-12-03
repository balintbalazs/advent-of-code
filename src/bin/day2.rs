use std::fs;

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("day2.txt").expect("Failed to read file");
    let score_part1: u32 = input.lines().map(|line| {
      let opp = line.chars().nth(0).unwrap();
      let me = line.chars().nth(2).unwrap();
      match (opp, me) {
        ('A', 'X') => 1 + 3,
        ('A', 'Y') => 2 + 6,
        ('A', 'Z') => 3 + 0,
        ('B', 'X') => 1 + 0,
        ('B', 'Y') => 2 + 3,
        ('B', 'Z') => 3 + 6,
        ('C', 'X') => 1 + 6,
        ('C', 'Y') => 2 + 0,
        ('C', 'Z') => 3 + 3,
        _ => 0
      }
    }).sum();
    dbg!(score_part1);

    let score_part2: u32 = input.lines().map(|line| {
      let opp = line.chars().nth(0).unwrap();
      let me = line.chars().nth(2).unwrap();
      match (opp, me) {
        ('A', 'X') => 3 + 0,
        ('A', 'Y') => 1 + 3,
        ('A', 'Z') => 2 + 6,
        ('B', 'X') => 1 + 0,
        ('B', 'Y') => 2 + 3,
        ('B', 'Z') => 3 + 6,
        ('C', 'X') => 2 + 0,
        ('C', 'Y') => 3 + 3,
        ('C', 'Z') => 1 + 6,
        _ => 0
      }
    }).sum();
    dbg!(score_part2);
}
