use std::{collections::HashSet, fs};

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day11.txt").expect("Failed to read file");
    let lines = input.lines();

    let mut empty_rows = vec![];
    let image = lines
        .enumerate()
        .map(|(r, line)| {
            if line.chars().all(|ch| ch == '.') {
                empty_rows.push(r)
            }
            line.chars().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = image[0].len();

    let mut empty_cols: HashSet<usize> = (0..width).collect();
    for row in &image {
        for (c, ch) in row.iter().enumerate() {
            if *ch == '#' {
                empty_cols.remove(&c);
            }
        }
    }

    // for row in &image {
    //     for ch in row {
    //         print!("{ch}");
    //     }
    //     println!();
    // }

    let mut empty_cols = empty_cols.into_iter().collect::<Vec<_>>();
    empty_cols.sort();

    // dbg!(&empty_cols);

    let mut expanded = vec![];
    for (r, mut row) in image.into_iter().enumerate() {
        // expand horiozontally
        for c in empty_cols.iter().rev() {
            row.insert(*c, '.');
        }
        // expand vertically
        if empty_rows.contains(&r) {
            expanded.push(row.clone());
        }
        expanded.push(row);
    }

    // for row in expanded.iter() {
    //     for ch in row {
    //         print!("{ch}");
    //     }
    //     println!();
    // }

    let mut galaxies = vec![];

    for (r, row) in expanded.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if ch == &'#' {
                galaxies.push((r as i32, c as i32));
            }
        }
    }

    // dbg!(&galaxies);

    let mut part1 = 0;

    for i in 0..(galaxies.len()-1) {
        for j in (i+1)..galaxies.len() {
            let v = (galaxies[i].0 -galaxies[j].0).abs();
            let h = (galaxies[i].1 -galaxies[j].1).abs();
            part1 += v + h;
        }
    }

    dbg!(part1);
}
