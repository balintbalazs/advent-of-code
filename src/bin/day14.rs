use std::{collections::HashMap, fs};

fn north(grid: &mut Vec<Vec<char>>) {
    let height = grid.len();
    let width = grid[0].len();

    for r in 1..height {
        for c in 0..width {
            if grid[r][c] == 'O' {
                let mut rr = r;
                while rr > 0 && grid[rr - 1][c] == '.' {
                    grid[rr - 1][c] = 'O';
                    grid[rr][c] = '.';
                    rr -= 1;
                }
            }
        }
    }
}

fn south(grid: &mut Vec<Vec<char>>) {
    let height = grid.len();
    let width = grid[0].len();

    for r in (0..height - 1).rev() {
        for c in 0..width {
            if grid[r][c] == 'O' {
                let mut rr = r;
                while rr < height - 1 && grid[rr + 1][c] == '.' {
                    grid[rr + 1][c] = 'O';
                    grid[rr][c] = '.';
                    rr += 1;
                }
            }
        }
    }
}

fn west(grid: &mut Vec<Vec<char>>) {
    let height = grid.len();
    let width = grid[0].len();

    for c in 1..width {
        for r in 0..height {
            if grid[r][c] == 'O' {
                let mut cc = c;
                while cc > 0 && grid[r][cc - 1] == '.' {
                    grid[r][cc - 1] = 'O';
                    grid[r][cc] = '.';
                    cc -= 1;
                }
            }
        }
    }
}

fn east(grid: &mut Vec<Vec<char>>) {
    let height = grid.len();
    let width = grid[0].len();

    for c in (0..width - 1).rev() {
        for r in 0..height {
            if grid[r][c] == 'O' {
                let mut cc = c;
                while cc < width - 1 && grid[r][cc + 1] == '.' {
                    grid[r][cc + 1] = 'O';
                    grid[r][cc] = '.';
                    cc += 1;
                }
            }
        }
    }
}

fn cycle(grid: &mut Vec<Vec<char>>) {
    north(grid);
    west(grid);
    south(grid);
    east(grid);
}

fn total_load(grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    grid.iter()
        .enumerate()
        .map(|(r, line)| (height - r) * line.iter().filter(|ch| **ch == 'O').count())
        .sum()
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day14.txt").expect("Failed to read file");
    let lines = input.lines();
    let mut grid: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let grid2 = grid.clone();
    let height = grid.len();
    let width = grid[0].len();

    for r in 1..height {
        for c in 0..width {
            if grid[r][c] == 'O' {
                let mut rr = r;
                while rr > 0 && grid[rr - 1][c] == '.' {
                    grid[rr - 1][c] = 'O';
                    grid[rr][c] = '.';
                    rr -= 1;
                }
            }
        }
    }

    let part1 = total_load(&grid);

    dbg!(part1);

    let mut grid = grid2;

    let mut count: usize = 1;
    let mut grids = HashMap::new();
    let mut loads = vec![part1];

    let (first_rep, period) = loop {
        cycle(&mut grid);
        let load = total_load(&grid);
        // dbg!(count);
        // dbg!(load);        
        if let Some(first_rep) = grids.get(&grid) {
            break (*first_rep, count - first_rep);
        };
        grids.insert(grid.clone(), count);
        loads.push(load);
        count += 1;
    };
    // dbg!(first_rep, period);

    let part2 = loads[(1_000_000_000 - first_rep) % period + first_rep];
    dbg!(part2);

    // for line in grid.iter() {
    //     for ch in line.iter() {
    //         print!("{ch}");
    //     }
    //     println!();
    // }
}
