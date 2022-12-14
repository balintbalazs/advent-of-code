use std::fs;

const AIR: char = '⬛';
const ROCK: char = '🟫';
const SAND: char = '🟡';

const SAND_INPUT: usize = 500;

fn from_string(input: &str) -> (Vec<Vec<char>>, usize) {
    let mut max_col = 0;
    let mut max_row = 0;

    // let mut min_col = usize::MAX;

    let rock_paths: Vec<_> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| {
                    let mut nums = pair.split(",");
                    let col = nums.next().unwrap().parse::<usize>().unwrap();
                    if col > max_col {
                        max_col = col;
                    }
                    // if col < min_col {
                    //     min_col = col;
                    // }
                    let row = nums.next().unwrap().parse::<usize>().unwrap();
                    if row > max_row {
                        max_row = row;
                    }
                    (row, col)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut cave = vec![vec![AIR; SAND_INPUT + max_row + 3]; max_row + 1];

    for rock_path in rock_paths {
        let mut prev = rock_path[0];
        for coord in rock_path {
            if prev.0 != coord.0 && prev.1 == coord.1 {
                let from = prev.0.min(coord.0);
                let to = prev.0.max(coord.0);
                for row in from..=to {
                    cave[row][coord.1] = ROCK;
                }
            } else if prev.1 != coord.1 && prev.0 == coord.0 {
                let from = prev.1.min(coord.1);
                let to = prev.1.max(coord.1);
                for col in from..=to {
                    cave[coord.0][col] = ROCK;
                }
            }
            prev = coord;
        }
    }

    (cave, SAND_INPUT - max_row - 1)
}

fn simulate(mut cave: Vec<Vec<char>>) -> (Vec<Vec<char>>, u32) {
    let mut row = 0;
    let mut col = SAND_INPUT;
    let mut sands = 0;
    while row < cave.len() - 1 {
        if cave[row + 1][col] == AIR {
            row += 1;
        } else if cave[row + 1][col - 1] == AIR {
            row += 1;
            col -= 1;
        } else if cave[row + 1][col + 1] == AIR {
            row += 1;
            col += 1;
        } else if cave[row][col] == SAND {
            break;
        } else {
            cave[row][col] = SAND;
            sands += 1;
            row = 0;
            col = SAND_INPUT;
        }
    }
    (cave, sands)
}

fn add_floor(mut cave: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let len = cave[0].len();
    cave.push(vec![AIR; len]);
    cave.push(vec![ROCK; len]);
    cave
}

fn print_cave(cave: &Vec<Vec<char>>, min_col: usize) {
    for row in cave {
        for c in row.iter().skip(min_col - 1) {
            print!("{c}");
        }
        println!();
    }
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day14.txt").expect("Failed to read file");
    let (cave, min_col) = from_string(&input);
    print_cave(&cave, min_col);
    println!();
    let (cave, sands) = simulate(cave);
    print_cave(&cave, min_col);
    println!();
    
    
    let cave = add_floor(cave);
    let (cave, sands2) = simulate(cave);
    print_cave(&cave, min_col);
    println!();
    
    dbg!(sands);
    dbg!(sands + sands2);

}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn can_parse_input() {
        let (cave, min_col) = from_string(TEST_DATA);
        print_cave(&cave, min_col);
        println!();

        let (cave, sands) = simulate(cave);
        print_cave(&cave, min_col);
        assert_eq!(24, sands);
        println!();

        let cave = add_floor(cave);
        print_cave(&cave, min_col);
        println!();

        let (cave, sands2) = simulate(cave);
        print_cave(&cave, min_col);

        assert_eq!(93, sands + sands2);
    }
}
