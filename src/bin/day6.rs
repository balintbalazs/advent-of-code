use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("inputs/day6.txt").expect("Failed to read file");

    // figure out first run of 4 different chars
    let mut start = 0;
    let mut end = 4;
    while end <= input.len() {
        let window = &input[start..end];
        let mut set = HashSet::new();
        for char in window.chars() {
            set.insert(char);
        }
        if set.len() == 4 {
            break;
        }
        start += 1;
        end += 1;
    }
    dbg!(end);

    // figure out first run of 14 different chars
    let mut start = 0;
    let mut end = 14;
    while end <= input.len() {
        let window = &input[start..end];
        let mut set = HashSet::new();
        for char in window.chars() {
            set.insert(char);
        }
        if set.len() == 14 {
            break;
        }
        start += 1;
        end += 1;
    }
    dbg!(end);
}
