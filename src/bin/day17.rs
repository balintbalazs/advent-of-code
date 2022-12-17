use std::fs;

fn rocks() -> Vec<Vec<String>> {
    vec![
        vec!["####".to_string()],
        vec![".#.".to_string(), "###".to_string(), ".#.".to_string()],
        // we index from bottom to top, so L is upside down
        vec!["###".to_string(), "..#".to_string(), "..#".to_string()],
        vec![
            "#".to_string(),
            "#".to_string(),
            "#".to_string(),
            "#".to_string(),
        ],
        vec!["##".to_string(), "##".to_string()],
    ]
}

fn intersects(part_of_well: &[Vec<char>], rock: &Vec<String>, left: usize) -> bool {
    for row in 0..rock.len() {
        for (col, char) in rock[row].chars().enumerate() {
            if char == '#' && part_of_well[row][col + left] == '#' {
                return true;
            }
        }
    }
    false
}

fn part1(jet_pattern: &str, num_rocks: usize) -> usize {
    let mut total_height = 0;
    let rocks = rocks();
    const WIDTH: usize = 7;
    // worst case - 2022 vertical bars stacked on top of each other
    let mut i = 0;
    let mut well = vec![vec!['.'; WIDTH]; 2022 * 4 + 10];
    for r in 0..num_rocks {
        // spawn
        let rock = &rocks[r % rocks.len()];
        let mut bottom = total_height + 3;
        let mut left = 2;

        loop {
            // push
            match &jet_pattern[i..=i] {
                ">" => {
                    if left + rock[0].len() < WIDTH {
                        if !intersects(&well[bottom..bottom + rock.len()], rock, left + 1) {
                            left += 1;
                        }
                    }
                }
                "<" => {
                    if left > 0 {
                        if !intersects(&well[bottom..bottom + rock.len()], rock, left - 1) {
                            left -= 1;
                        }
                    }
                }
                p => unreachable!("Invalid jet pattern {p}"),
            }
            i += 1;
            i %= jet_pattern.len();
            // fall
            if bottom > 0 && !intersects(&well[bottom - 1..bottom - 1 + rock.len()], rock, left) {
                bottom -= 1;
            } else {
                // settle
                for row in 0..rock.len() {
                    for (col, char) in rock[row].chars().enumerate() {
                        if char == '#' {
                            // don't overwrite other rocks with air
                            well[bottom + row][left + col] = char;
                        }
                    }
                }
                total_height = total_height.max(bottom + rock.len());

                // println!();
                // for row in (0..total_height+5).rev() {
                //     for c in &well[row] {
                //         print!("{c}");
                //     }
                //     println!();
                // }
                break;
            }
        }
    }
    total_height
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day17.txt").expect("Failed to read file");
    dbg!(part1(&input, 2022));
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        assert_eq!(3068, part1(TEST_DATA, 2022));
    }
}
