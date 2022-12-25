use std::{
    collections::{HashSet, VecDeque},
    fs,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    r: i64,
    c: i64,
}

#[derive(Debug, Clone, Copy)]
struct Dir {
    r: i64,
    c: i64,
}

impl Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, rhs: Dir) -> Self::Output {
        Pos {
            r: self.r + rhs.r,
            c: self.c + rhs.c,
        }
    }
}

impl Sub for Pos {
    type Output = Dir;

    fn sub(self, rhs: Self) -> Self::Output {
        Dir {
            r: self.r - rhs.r,
            c: self.c - rhs.c,
        }
    }
}

impl Mul<Dir> for i64 {
    type Output = Dir;

    fn mul(self, rhs: Dir) -> Self::Output {
        Dir {
            r: self * rhs.r,
            c: self * rhs.c,
        }
    }
}

impl Dir {
    fn try_from_char(ch: char) -> Option<Dir> {
        match ch {
            '>' => Some(Dir { r: 0, c: 1 }),
            '<' => Some(Dir { r: 0, c: -1 }),
            'v' => Some(Dir { r: 1, c: 0 }),
            '^' => Some(Dir { r: -1, c: 0 }),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Blizzard {
    pos: Pos,
    dir: Dir,
}

#[derive(Debug)]
struct Valley {
    width: i64,
    height: i64,
    starting_point: Pos,
    exit_point: Pos,
    blizzards: Vec<Blizzard>,
}

impl Valley {
    fn from_str(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let height = lines.len() - 2;
        let width = lines[0].len() - 2;
        let starting_point_col = lines[0].chars().position(|ch| ch == '.').unwrap() - 1; // ignore firs col of wall
        let starting_point = Pos {
            r: -1,
            c: starting_point_col as i64,
        };
        let exit_point_col = lines[lines.len() - 1]
            .chars()
            .position(|ch| ch == '.')
            .unwrap()
            - 1;
        let exit_point = Pos {
            r: height as i64,
            c: exit_point_col as i64,
        };
        let mut blizzards = Vec::new();
        for (r, line) in lines[1..=height].iter().enumerate() {
            let line = &line[1..=width];
            for (c, ch) in line.chars().enumerate() {
                let pos = Pos {
                    r: r as i64,
                    c: c as i64,
                };
                if let Some(dir) = Dir::try_from_char(ch) {
                    blizzards.push(Blizzard { pos, dir });
                }
            }
        }
        Self {
            width: width as i64,
            height: height as i64,
            starting_point,
            exit_point,
            blizzards,
        }
    }

    fn blizzards_at(&self, time: i64) -> Vec<Pos> {
        let mut blizzards = Vec::new();
        for blizzard in &self.blizzards {
            let mut blizzard = blizzard.pos + time * blizzard.dir;
            blizzard.r = blizzard.r.rem_euclid(self.height);
            blizzard.c = blizzard.c.rem_euclid(self.width);
            blizzards.push(blizzard);
        }

        blizzards
    }

    fn find_way(&self, start_time: i64, backwards: bool) -> i64 {
        let moves = vec![
            Dir { r: 1, c: 0 },
            Dir { r: -1, c: 0 },
            Dir { r: 0, c: 1 },
            Dir { r: 0, c: -1 },
            Dir { r: 0, c: 0 },
        ];

        // do breadth-first search
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        if backwards {
            q.push_back((self.exit_point, start_time));
        } else {
            q.push_back((self.starting_point, start_time));
        }

        // blizzard positions repeat with this period
        let period = num::integer::lcm(self.width, self.height);

        while let Some((pos, time)) = q.pop_front() {
            if visited.contains(&(pos, time.rem_euclid(period))) {
                continue;
            }
            visited.insert((pos, time.rem_euclid(period)));
            let time = time + 1;
            let blizzards = self.blizzards_at(time);
            for m in &moves {
                let next_pos = pos + *m;
                if !blizzards.contains(&next_pos) {
                    if !backwards && next_pos == self.exit_point {
                        return time;
                    }
                    if backwards && next_pos == self.starting_point {
                        return time;
                    }
                    if (next_pos.r >= 0
                        && next_pos.r < self.height
                        && next_pos.c >= 0
                        && next_pos.c < self.width)
                        || (!backwards && next_pos == self.starting_point)
                        || (backwards && next_pos == self.exit_point)
                    {
                        // step to next pos is possible
                        q.push_back((next_pos, time));
                    }
                }
            }
        }

        panic!("exit not found but loop stopped");
    }
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day24.txt").expect("Failed to read file");
    let valley = Valley::from_str(&input);
    // dbg!(valley.height);
    // dbg!(valley.width);
    let part1 = valley.find_way(0, false);
    dbg!(part1);
    let trip_back = valley.find_way(part1, true);
    let part2 = valley.find_way(trip_back, false);
    dbg!(part2);
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_part1() {
        let valley = Valley::from_str(TEST_DATA);
        dbg!(&valley);
        dbg!(valley.blizzards_at(24));
        dbg!(valley.find_way(0, false));
    }

    #[test]
    fn test_part2() {
        let valley = Valley::from_str(TEST_DATA);
        let part1 = valley.find_way(0, false);
        dbg!(part1);
        let trip_back = valley.find_way(part1, true);
        dbg!(trip_back);
        let part2 = valley.find_way(trip_back, false);
        dbg!(part2);
    }
}
