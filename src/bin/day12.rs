use std::{
    collections::{HashSet, VecDeque},
    fs,
    ops::{Add, Sub},
};

use crate::Part::*;

#[derive(PartialEq)]
enum Part {
    Part1,
    Part2,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord(i32, i32);
#[derive(Clone, Copy)]
struct Step(i32, i32);

impl Sub for Coord {
    type Output = Step;

    fn sub(self, rhs: Self) -> Self::Output {
        Step(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add<Step> for Coord {
    type Output = Coord;

    fn add(self, rhs: Step) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn solve(input: &str, part: Part) -> Option<u32> {
    let mut height_map: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();

    let width = height_map[0].len();
    let height = height_map.len();

    let mut start = Coord(0, 0);
    let mut end = Coord(0, 0);

    for row in 0..height {
        for col in 0..width {
            height_map[row][col] = match height_map[row][col] {
                b'S' => {
                    start = Coord(row as i32, col as i32);
                    b'a'
                }
                b'E' => {
                    end = Coord(row as i32, col as i32);
                    b'z'
                }
                height => height,
            };
        }
    }

    let mut next_positions = VecDeque::new();

    match part {
        Part1 => next_positions.push_back((start, 0)),
        Part2 => {
            for row in 0..height {
                for col in 0..width {
                    if height_map[row][col] == b'a' {
                        next_positions.push_back((Coord(row as i32, col as i32), 0));
                    }
                }
            }
        }
    }

    let directions = vec![Step(1, 0), Step(-1, 0), Step(0, 1), Step(0, -1)];

    let mut done = HashSet::new();

    let width = width as i32;
    let height = height as i32;

    while let Some((pos, distance)) = next_positions.pop_front() {
        let current_height = height_map[pos.0 as usize][pos.1 as usize];
        for direction in &directions {
            let neighbor = pos + *direction;
            if neighbor.0 >= 0 && neighbor.0 < height && neighbor.1 >= 0 && neighbor.1 < width {
                let next_height = height_map[neighbor.0 as usize][neighbor.1 as usize];
                if next_height < current_height || next_height - current_height <= 1 {
                    if !done.contains(&neighbor) {
                        done.insert(neighbor);
                        if neighbor == end {
                            return Some(distance + 1);
                        }
                        next_positions.push_back((neighbor, distance + 1));
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("inputs/day12.txt").unwrap();
    dbg!(solve(&input, Part1));
    dbg!(solve(&input, Part2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_round_part1() {
        assert_eq!(Some(31), solve(TEST_INPUT, Part1));
    }

    #[test]
    fn test_round_part2() {
        assert_eq!(Some(29), solve(TEST_INPUT, Part2));
    }
}
