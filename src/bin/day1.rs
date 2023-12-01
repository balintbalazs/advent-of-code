use std::fs;

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day1.txt").expect("Failed to read file");
    let lines = input.lines();
    let part1: u32 = lines.clone().map(|line| {
        let left = line.chars().find(is_digit).expect("no number found in line");
        let right = line.chars().rev().find(is_digit).expect("no number found in line");
        let left = left as u32 - '0' as u32;
        let right = right as u32 - '0' as u32;
        left * 10 + right
    }).sum();
    dbg!(part1);

    let numbers = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let part2: u32 = lines.map(|l| {
        let mut l = l.to_string();
        for (index, number) in numbers.iter().enumerate() {
            l = l.replace((index as u8 + '0' as u8) as char, number);
        }
        let mut r = l.clone();
        let left = loop {
            let n = match l {
                line if line.starts_with("one") => 1,
                line if line.starts_with("two") => 2,
                line if line.starts_with("three") => 3,
                line if line.starts_with("four") => 4,
                line if line.starts_with("five") => 5,
                line if line.starts_with("six") => 6,
                line if line.starts_with("seven") => 7,
                line if line.starts_with("eight") => 8,
                line if line.starts_with("nine") => 9,
                _ => {
                    l.remove(0);
                    continue;
                }
            };
            break n;
        };
        let right = loop {
            let n = match r {
                line if line.ends_with("one") => 1,
                line if line.ends_with("two") => 2,
                line if line.ends_with("three") => 3,
                line if line.ends_with("four") => 4,
                line if line.ends_with("five") => 5,
                line if line.ends_with("six") => 6,
                line if line.ends_with("seven") => 7,
                line if line.ends_with("eight") => 8,
                line if line.ends_with("nine") => 9,
                _ => {
                    r.pop();
                    continue;
                }
            };
            break n;
        };
        left * 10 + right
    }).sum();
    dbg!(part2);
}

fn is_digit(c: &char) -> bool {
    c >= &'0' && c <= &'9'
}
