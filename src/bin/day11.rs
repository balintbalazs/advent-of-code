use std::{collections::HashSet, fs};

fn dilate(
    galaxies: Vec<(i64, i64)>,
    empty_rows: &[i64],
    empty_cols: &[i64],
    by: i64,
) -> Vec<(i64, i64)> {
    // empty_rows.sort();
    // empty_cols.sort();

    galaxies
        .into_iter()
        .map(|(r, c)| {
            let dilated_row_count = empty_rows.iter().filter(|er| **er < r).count() as i64 * by;
            let dilated_col_count = empty_cols.iter().filter(|ec| **ec < c).count() as i64 * by;
            (dilated_row_count + r, dilated_col_count + c)
        })
        .collect()
}

fn all_pair_distances(galaxies: Vec<(i64, i64)>) -> i64 {
    let mut res = 0;

    for i in 0..(galaxies.len() - 1) {
        for j in (i + 1)..galaxies.len() {
            let v = (galaxies[i].0 - galaxies[j].0).abs();
            let h = (galaxies[i].1 - galaxies[j].1).abs();
            res += v + h;
        }
    }
    res
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day11.txt").expect("Failed to read file");
    let lines = input.lines();

    let mut empty_rows = vec![];
    let image = lines
        .enumerate()
        .map(|(r, line)| {
            if line.chars().all(|ch| ch == '.') {
                empty_rows.push(r as i64)
            }
            line.chars().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = image[0].len() as i64;

    let mut empty_cols: HashSet<i64> = (0..width).collect();
    for row in &image {
        for (c, ch) in row.iter().enumerate() {
            if *ch == '#' {
                empty_cols.remove(&(c as i64));
            }
        }
    }

    let mut empty_cols = empty_cols.into_iter().collect::<Vec<_>>();
    empty_cols.sort();

    let mut galaxies = vec![];

    for (r, row) in image.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if ch == &'#' {
                galaxies.push((r as i64, c as i64));
            }
        }
    }

    // dbg!(&galaxies);

    let part1 = all_pair_distances(dilate(galaxies.clone(), &empty_rows, &empty_cols, 1));
    dbg!(part1);
    let part2 = all_pair_distances(dilate(galaxies, &empty_rows, &empty_cols, 999_999));
    dbg!(part2);
}
