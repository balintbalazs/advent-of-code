use std::fs;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::{Down, Left, Right, Up};

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day10.txt").expect("Failed to read file");
    let lines = input.lines();
    let mut start = (0, 0);
    let mut tiles = lines
        .enumerate()
        .map(|(row, line)| {
            let ch = line
                .chars()
                .enumerate()
                .map(|(col, ch)| {
                    if ch == 'S' {
                        start = (row, col);
                    }
                    ch
                })
                .collect::<Vec<_>>();
            ch
        })
        .collect::<Vec<_>>();

    let mut starts = Vec::new();
    let mut dirs = Vec::new();

    let up = (start.0 - 1, start.1);
    if let '7' | '|' | 'F' = tiles[up.0][up.1] {
        starts.push(up);
        dirs.push(Up);
    }

    let left = (start.0, start.1 - 1);
    if let 'F' | '-' | 'L' = tiles[left.0][left.1] {
        starts.push(left);
        dirs.push(Left);
    }

    let right = (start.0, start.1 + 1);
    if let '7' | '-' | 'J' = tiles[right.0][right.1] {
        starts.push(right);
        dirs.push(Right);
    }

    let down = (start.0 + 1, start.1);
    if let 'J' | '|' | 'L' = tiles[down.0][down.1] {
        starts.push(down);
        dirs.push(Down);
    }

    assert_eq!(starts.len(), 2);

    let mut part1 = 1;

    while starts[0] != starts[1] {
        for (i, (d, l)) in dirs.iter_mut().zip(starts.iter_mut()).enumerate() {
            *d = match d {
                Up => match tiles[l.0][l.1] {
                    'F' => Right,
                    '7' => Left,
                    '|' => Up,
                    _ => panic!("invalid direction"),
                },
                Down => match tiles[l.0][l.1] {
                    'L' => Right,
                    'J' => Left,
                    '|' => Down,
                    _ => panic!("invalid direction"),
                },
                Right => match tiles[l.0][l.1] {
                    '7' => Down,
                    'J' => Up,
                    '-' => Right,
                    _ => panic!("invalid direction"),
                },
                Left => match tiles[l.0][l.1] {
                    'F' => Down,
                    'L' => Up,
                    '-' => Left,
                    _ => panic!("invalid direction"),
                },
            };
            match d {
                Up => l.0 -= 1,
                Down => l.0 += 1,
                Left => l.1 -= 1,
                Right => l.1 += 1,
            }
        }

        part1 += 1;
    }

    dbg!(part1);
}
