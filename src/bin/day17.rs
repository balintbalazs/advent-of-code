use std::{collections::VecDeque, fs};
struct State {
    row: usize,
    col: usize,
    total_loss: usize,
    straight_length: usize,
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day17.ex").expect("Failed to read file");
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| (ch as usize - '0' as usize))
                .collect()
        })
        .collect();

    for line in &grid {
        for num in line {
            print!("{num}")
        }
        println!()
    }

    let height = grid.len();
    let width = grid[0].len();

    let start = State {
        row: 0,
        col: 0,
        total_loss: 0,
        straight_length: 0,
    };

    let mut q = VecDeque::new();
    q.push_back(start);
    loop {
        let State {
            row,
            col,
            mut total_loss,
            straight_length,
        } = q.pop_front().unwrap();
        total_loss += grid[row][col];
        if row = height -1 && col = width - 1 
    }
}
