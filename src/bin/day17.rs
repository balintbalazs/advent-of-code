use std::{
    collections::{HashMap, HashSet},
    fs,
};

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

fn intersects(well: &Vec<Vec<char>>, rock: &Vec<String>, left: usize, bottom: usize) -> bool {
    for row in 0..rock.len() {
        for (col, char) in rock[row].chars().enumerate() {
            let row = (row + bottom) % well.len();
            if char == '#' && well[row][col + left] == '#' {
                return true;
            }
        }
    }
    false
}

fn drop_rocks(jet_pattern: &str, num_rocks: usize) -> usize {
    let mut total_height = 0;
    let rocks = rocks();
    const WIDTH: usize = 7;
    const HEIGHT: usize = 100;
    // worst case - 2022 vertical bars stacked on top of each other
    let mut i = 0;
    let mut well = vec![vec!['.'; WIDTH]; HEIGHT];

    let mut previous_total_heights = vec![];
    let mut seen_combo = HashMap::new();
    let mut cycle_length = None;
    let mut cycle_start = None;
    // let mut found = false;
    for r in 0.. {
        // spawn
        let rock = &rocks[r % rocks.len()];
        let mut bottom = total_height + 3;
        let mut left = 2;
        for row in total_height..bottom + rock.len() {
            well[row % HEIGHT] = vec!['.'; WIDTH];
        }

        loop {
            // push
            match &jet_pattern[i..=i] {
                ">" => {
                    if left + rock[0].len() < WIDTH {
                        if !intersects(&well, rock, left + 1, bottom) {
                            left += 1;
                        }
                    }
                }
                "<" => {
                    if left > 0 {
                        if !intersects(&well, rock, left - 1, bottom) {
                            left -= 1;
                        }
                    }
                }
                p => unreachable!("Invalid jet pattern {p}"),
            }
            i += 1;
            i %= jet_pattern.len();
            // fall
            if bottom > 0 && !intersects(&well, rock, left, bottom - 1) {
                bottom -= 1;
            } else {
                // settle
                for row in 0..rock.len() {
                    for (col, char) in rock[row].chars().enumerate() {
                        if char == '#' {
                            // don't overwrite other rocks with air
                            well[(bottom + row) % HEIGHT][left + col] = char;
                        }
                    }
                }
                total_height = total_height.max(bottom + rock.len());
                previous_total_heights.push(total_height);

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

        // if r % 35 == 15 {
        //     print!("*");
        // } else {
        //     print!(" ");
        // }
        // println!(" Rock num {r:>3}, type {rr},  push {i:>2}, total height {total_height:>5}");
        // if !found {
        match cycle_start {
            Some(cycle_start) => {
                let cycle_length = cycle_length.unwrap();
                // dbg!(r);
                // dbg!(cycle_start);
                if r == cycle_start + cycle_length {
                    // let prev_r = r - cycle_length;
                    let base_height = previous_total_heights[cycle_start];
                    let cycle_height = previous_total_heights[r] - base_height;
                    let full_cycles = (num_rocks - cycle_start) / cycle_length;
                    let last_cycle_index = (num_rocks - cycle_start) % cycle_length;
                    let last_cycle_height =
                        previous_total_heights[cycle_start + last_cycle_index - 1] - base_height;

                    dbg!(base_height);
                    dbg!(cycle_height);
                    dbg!(full_cycles);
                    dbg!(last_cycle_height);

                    let result = base_height + cycle_height * full_cycles + last_cycle_height;
                    return result;
                }
            }
            None => {
                let rr = r % 5;
                if let Some(prev_r) = seen_combo.get(&(rr, i)) {
                    // println!("Repeats!");
                    // dbg!(prev_r);
                    // dbg!(r);
                    // dbg!(rr);
                    // dbg!(i);

                    cycle_start = Some(r);
                    cycle_length = Some(r - prev_r);
                } else {
                    seen_combo.insert((rr, i), r);
                }
            }
        }

        // break;
        // dbg!(result);
        // found = true;

        // }
    }
    unreachable!()
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day17.txt").expect("Failed to read file");
    dbg!(drop_rocks(&input, 2022));
    dbg!(drop_rocks(&input, 1_000_000_000_000));
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        assert_eq!(3068, drop_rocks(TEST_DATA, 2022));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1514285714288, drop_rocks(TEST_DATA, 1_000_000_000_000));
    }
}
