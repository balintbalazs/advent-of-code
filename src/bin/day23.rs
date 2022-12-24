use std::{
    collections::{HashMap, HashSet},
    fs,
};

enum Direction {
    N,
    S,
    E,
    W,
}

struct Field {
    elves: HashSet<(i64, i64)>,
    directions: Vec<Direction>,
}

impl Field {
    fn from_str(input: &str) -> Self {
        let mut elves = HashSet::new();
        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    elves.insert((row as i64, col as i64));
                }
            }
        }

        Self {
            elves,
            directions: vec![Direction::N, Direction::S, Direction::W, Direction::E],
        }
    }

    fn print(&self) {
        let (rows, cols) = self.size();
        let (minrow, mincol) = self.min();
        let mut output = vec![vec!['.'; cols]; rows];
        for (r, c) in &self.elves {
            output[(r - minrow) as usize][(c - mincol) as usize] = '#';
        }
        for line in output {
            for ch in line {
                print!("{ch}");
            }
            println!();
        }
        println!();
    }

    fn min(&self) -> (i64, i64) {
        let mut minrow = i64::MAX;
        let mut mincol = i64::MAX;

        for &(r, c) in &self.elves {
            if r < minrow {
                minrow = r;
            }
            if c < mincol {
                mincol = c;
            }
        }
        (minrow, mincol)
    }

    fn max(&self) -> (i64, i64) {
        let mut maxrow = i64::MIN;
        let mut maxcol = i64::MIN;

        for &(r, c) in &self.elves {
            if r > maxrow {
                maxrow = r;
            }
            if c > maxcol {
                maxcol = c;
            }
        }
        (maxrow, maxcol)
    }

    fn size(&self) -> (usize, usize) {
        let (minrow, mincol) = self.min();
        let (maxrow, maxcol) = self.max();

        (
            (maxrow - minrow + 1) as usize,
            (maxcol - mincol + 1) as usize,
        )
    }

    fn empty_ground(&self) -> usize {
        let size = self.size();
        let total_area = size.0 * size.1;

        total_area as usize - self.elves.len()
    }

    fn step(&mut self) -> bool {
        // elements in vec: (current_pos, next_pos)
        let mut next_positions = Vec::with_capacity(self.elves.len());
        // keep track which elves moved
        let mut moved = Vec::with_capacity(self.elves.len());

        for &(r, c) in &self.elves {
            let ne = self.elves.contains(&(r - 1, c + 1));
            let n = self.elves.contains(&(r - 1, c));
            let nw = self.elves.contains(&(r - 1, c - 1));
            let w = self.elves.contains(&(r, c - 1));
            let sw = self.elves.contains(&(r + 1, c - 1));
            let s = self.elves.contains(&(r + 1, c));
            let se = self.elves.contains(&(r + 1, c + 1));
            let e = self.elves.contains(&(r, c + 1));

            let mut m = false;
            // all free around, no need to move
            if !(ne || n || nw || w || sw || s || se || e) {
                next_positions.push(((r, c), (r, c)));
            } else {
                // check for possible moves
                for dir in &self.directions {
                    match dir {
                        Direction::N => {
                            if !(ne || n || nw) {
                                next_positions.push(((r, c), (r - 1, c)));
                                m = true;
                                break;
                            }
                        }
                        Direction::S => {
                            if !(se || s || sw) {
                                next_positions.push(((r, c), (r + 1, c)));
                                m = true;
                                break;
                            }
                        }
                        Direction::E => {
                            if !(ne || e || se) {
                                next_positions.push(((r, c), (r, c + 1)));
                                m = true;
                                break;
                            }
                        }
                        Direction::W => {
                            if !(nw || w || sw) {
                                next_positions.push(((r, c), (r, c - 1)));
                                m = true;
                                break;
                            }
                        }
                    }
                }
                if !m {
                    next_positions.push(((r, c), (r, c)));
                }
            }
            moved.push(m);
        }

        if moved.iter().all(|m| !*m) {
            // nothing moved
            return false;
        }

        // do move, considering overlapping moves
        let mut target_positions = HashMap::new();
        for (_, next_pos) in &next_positions {
            if let Some(count) = target_positions.get(next_pos) {
                target_positions.insert(next_pos, count + 1);
            } else {
                target_positions.insert(next_pos, 1);
            }
        }

        let mut elves = HashSet::new();

        for &(current_pos, next_pos) in &next_positions {
            if target_positions.get(&next_pos).unwrap() == &1 {
                elves.insert(next_pos);
            } else {
                elves.insert(current_pos);
            }
        }
        self.elves = elves;

        // change order of directions
        self.directions.rotate_left(1);

        true
    }
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day23.txt").expect("Failed to read file");
    let mut field = Field::from_str(&input);
    for _ in 0..10 {
        field.step();
    }
    // part1
    dbg!(field.empty_ground());

    // part 2
    let mut round = 11;
    while field.step() {
        round += 1;
    }
    dbg!(round);
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn test_part1() {
        let mut field = Field::from_str(TEST_DATA);
        field.print();
        for _ in 0..10 {
            field.step();
        }
        field.print();
        assert_eq!(110, field.empty_ground());
    }

    #[test]
    fn test_part2() {
        let mut field = Field::from_str(TEST_DATA);
        let mut round = 1;
        while field.step() {
            round += 1;
        }
        assert_eq!(20, round);
    }
}
