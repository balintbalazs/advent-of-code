use std::fs;

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day1.txt").expect("Failed to read file");
    println!("Hello day1")
}
