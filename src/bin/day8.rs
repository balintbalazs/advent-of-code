use std::fs;

// fn from_str(&str) -> Vec<Vec<u8>

fn main() {
    let input = fs::read_to_string("inputs/day8.txt").expect("Failed to read file");
    // let input = TEST_INPUT;
    let mut forest = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for char in line.chars() {
            let height: u8 = char.to_digit(10).unwrap() as u8;
            row.push(height);
        }
        if row.len() > 0 {
            forest.push(row);
        }
    }
    let width = forest[0].len();
    let height = forest.len();

    let mut visible_from_left = vec![vec![true; width]; height];
    for row in 0..height {
        let mut tallest = forest[row][0];
        for col in 1..width {
            let visible = tallest < forest[row][col];
            if visible {
                tallest = forest[row][col];
            } else {
                visible_from_left[row][col] = false;
            }
        }
    }

    let mut visible_from_right = vec![vec![true; width]; height];
    for row in 0..height {
        let mut tallest = forest[row][width - 1];
        for col in (0..width - 1).rev() {
            let visible = tallest < forest[row][col];
            if visible {
                tallest = forest[row][col];
            } else {
                visible_from_right[row][col] = false;
            }
        }
    }

    let mut visible_from_top = vec![vec![true; width]; height];
    for col in 0..width {
        let mut tallest = forest[0][col];
        for row in 1..height {
            let visible = tallest < forest[row][col];
            if visible {
                tallest = forest[row][col];
            } else {
                visible_from_top[row][col] = false;
            }
        }
    }

    let mut visible_from_bottom = vec![vec![true; width]; height];
    for col in 0..width {
        let mut tallest = forest[height - 1][col];
        for row in (0..height - 1).rev() {
            let visible = tallest < forest[row][col];
            if visible {
                tallest = forest[row][col];
            } else {
                visible_from_bottom[row][col] = false;
            }
        }
    }

    let visible = visible_from_left.iter().flatten();
    let visible = visible
        .zip(visible_from_right.iter().flatten())
        .map(|(a, b)| a | b);
    let visible = visible
        .zip(visible_from_top.iter().flatten())
        .map(|(a, b)| a | b);
    let visible = visible
        .zip(visible_from_bottom.iter().flatten())
        .map(|(a, b)| a | b);
    let visible_count = visible.filter(|x| *x).count();

    dbg!(visible_count);

    let mut view_to_left = vec![vec![0; width]; height];
    for row in 0..height {
        for col in 1..width {
            let mut other = col - 1;
            view_to_left[row][col] = 1;
            while other > 0 && forest[row][other] < forest[row][col] {
                view_to_left[row][col] += 1;
                other -= 1;
            }
        }
    }

    let mut view_to_right = vec![vec![0; width]; height];
    for row in 0..height {
        for col in (0..width - 1).rev() {
            let mut other = col + 1;
            view_to_right[row][col] = 1;
            while other < width - 1 && forest[row][other] < forest[row][col] {
                view_to_right[row][col] += 1;
                other += 1;
            }
        }
    }

    let mut view_to_top = vec![vec![0; width]; height];
    for col in 0..width {
        for row in 1..height {
            let mut other = row - 1;
            view_to_top[row][col] = 1;
            while other > 0 && forest[other][col] < forest[row][col] {
                view_to_top[row][col] += 1;
                other -= 1;
            }
        }
    }

    let mut view_to_bottom = vec![vec![0; width]; height];
    for col in 0..width {
        for row in (0..height - 1).rev() {
            let mut other = row + 1;
            view_to_bottom[row][col] = 1;
            while other < height - 1 && forest[other][col] < forest[row][col] {
                view_to_bottom[row][col] += 1;
                other += 1;
            }
        }
    }

    // dbg!(forest);
    // dbg!(&view_to_bottom);

    let score = view_to_left.iter().flatten();
    let score = score
        .zip(view_to_right.iter().flatten())
        .map(|(a, b)| a * b);
    let score = score.zip(view_to_top.iter().flatten()).map(|(a, b)| a * b);
    let score = score
        .zip(view_to_bottom.iter().flatten())
        .map(|(a, b)| a * b);
    let max_score = score.max();

    dbg!(max_score);
}

const TEST_INPUT: &str = r###"
30373
25512
65332
33549
35390
"###;
