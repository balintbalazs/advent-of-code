use std::{collections::VecDeque, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Left => (0, -1),
            Right => (0, 1),
            Up => (-1, 0),
            Down => (1, 0),
        }
    }
}
use Direction::{Down, Left, Right, Up};

fn energize(grid: &Vec<Vec<char>>, starting_beam: (i32, i32, Direction)) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let mut beams = VecDeque::new();
    let mut incoming_directions = vec![vec![vec![]; width]; height];
    beams.push_back(starting_beam);

    while let Some(beam) = beams.pop_front() {
        let (r, c, dir) = beam;
        let ur = r as usize;
        let uc = c as usize;
        if !incoming_directions[ur][uc].contains(&dir) {
            incoming_directions[ur][uc].push(dir);
            let out_directions = match grid[ur][uc] {
                '.' => {
                    vec![dir]
                }
                '-' => match dir {
                    Left | Right => vec![dir],
                    Up | Down => vec![Left, Right],
                },
                '|' => match dir {
                    Up | Down => vec![dir],
                    Left | Right => vec![Up, Down],
                },
                '/' => match dir {
                    Left => vec![Down],
                    Right => vec![Up],
                    Up => vec![Right],
                    Down => vec![Left],
                },
                '\\' => match dir {
                    Left => vec![Up],
                    Right => vec![Down],
                    Up => vec![Left],
                    Down => vec![Right],
                },
                t => panic!("unknown grid tile {t}"),
            };
            for out_direction in out_directions {
                let (dr, dc) = out_direction.delta();
                let r = r + dr;
                let c = c + dc;
                if r >= 0 && r < height as i32 && c >= 0 && c < width as i32 {
                    beams.push_back((r, c, out_direction));
                }
            }
        }
    }

    incoming_directions
        .iter()
        .map(|line| line.iter().filter(|dirs| dirs.len() > 0).count())
        .sum()
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day16.txt").expect("Failed to read file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let starting_beam = (0, 0, Right);

    // for line in &incoming_directions {
    //     for dirs  in line {
    //         if dirs.is_empty() {print!(".")} else {print!("#")}
    //     }
    //     println!()
    // }

    let part1: usize = energize(&grid, starting_beam);
    dbg!(part1);

    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    let top_row = (0..width)
        .map(|c| energize(&grid, (0, c, Down)))
        .max()
        .unwrap();
    let bottom_row = (0..width)
        .map(|c| energize(&grid, (height - 1, c, Up)))
        .max()
        .unwrap();
    let left_col = (0..height)
        .map(|r| energize(&grid, (r, 0, Right)))
        .max()
        .unwrap();
    let right_col = (0..height)
        .map(|r| energize(&grid, (r, width - 1, Left)))
        .max()
        .unwrap();

    let binding = [top_row, bottom_row, left_col, right_col];
    let part2 = binding.iter().max().unwrap();
    dbg!(part2);
}
