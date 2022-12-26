use std::{collections::HashSet, fs};

#[derive(Clone, Copy, Debug)]
#[repr(i8)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Facing {
    fn turn(&mut self, t: Turn) {
        let f = *self as i8 + t as i8;
        let f = f.rem_euclid(4);
        *self = match f {
            0 => Facing::Right,
            1 => Facing::Down,
            2 => Facing::Left,
            3 => Facing::Up,
            _ => unimplemented!(),
        };
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos {
    r: usize,
    c: usize,
}

#[derive(Debug, Clone, Copy)]
#[repr(i8)]
enum Turn {
    Left = -1,
    Right = 1,
}
#[derive(Debug)]
enum Move {
    Forward(usize),
    Turn(Turn),
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    col_limits: Vec<(usize, usize)>,
    row_limits: Vec<(usize, usize)>,
    obstacles: HashSet<Pos>,
    starting_col: usize,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let starting_col = lines[0].chars().position(|ch| ch == '.').unwrap() + 1;

        let mut col_limits = vec![(0, 0)];
        let mut row_limits = vec![(None, None); width + 1];
        let mut obstacles = HashSet::new();

        for (&line, r) in lines.iter().zip(1..) {
            let mut min_col = None;
            let mut max_col = None;
            let line_len = line.len();

            for (ch, c) in line.chars().zip(1..) {
                //deal with column limits for row r
                match (min_col, max_col) {
                    (None, None) => {
                        if ch != ' ' {
                            min_col = Some(c)
                        }
                    }
                    (Some(_), None) => {
                        if ch == ' ' {
                            max_col = Some(c - 1)
                        }
                    }
                    (None, Some(_)) => unreachable!(),
                    (Some(_), Some(_)) => (),
                }

                //deal with row limits
                match row_limits[c] {
                    (None, None) => {
                        if ch != ' ' {
                            row_limits[c].0 = Some(r)
                        }
                    }
                    (Some(_), None) => {
                        if ch == ' ' {
                            row_limits[c].1 = Some(r - 1)
                        }
                    }
                    (None, Some(_)) => unreachable!("c={c}, r={r}"),
                    (Some(_), Some(_)) => (),
                }
                if c == line_len {
                    for i in (c + 1)..=width {
                        if row_limits[i].1.is_none() {
                            row_limits[i].1 = Some(r - 1);
                        }
                    }
                }

                // obstacles
                if ch == '#' {
                    obstacles.insert(Pos { r, c });
                }
            }
            col_limits.push((min_col.unwrap(), max_col.unwrap_or(line.len())));
        }

        let row_limits = row_limits
            .into_iter()
            .map(|(min, max)| (min.unwrap_or(1), max.unwrap_or(height)))
            .collect();
        Self {
            width,
            height,
            col_limits,
            row_limits,
            obstacles,
            starting_col,
        }
    }

    fn starting_col(&self) -> usize {
        self.starting_col
    }
}

#[derive(Debug)]
struct Board {
    map: Map,
    facing: Facing,
    position: Pos,
    moves: Vec<Move>,
}

impl Board {
    fn from_str(input: &str) -> Self {
        let input = input.to_string().replace("\r\n", "\n");
        let mut parts = input.split("\n\n");
        let map_input = parts.next().unwrap();
        let map = Map::from_str(map_input);

        let position = Pos {
            r: 1,
            c: map.starting_col(),
        };

        let moves_input = parts.next().unwrap();
        let mut moves = vec![];
        let mut buffer = "".to_string();

        for c in moves_input.chars() {
            match c {
                '0'..='9' => buffer.push(c),
                'L' => {
                    if buffer.len() > 0 {
                        let num = buffer.parse().unwrap();
                        buffer = "".to_string();
                        moves.push(Move::Forward(num));
                    }
                    moves.push(Move::Turn(Turn::Left));
                }
                'R' => {
                    if buffer.len() > 0 {
                        let num = buffer.parse().unwrap();
                        buffer = "".to_string();
                        moves.push(Move::Forward(num));
                    }
                    moves.push(Move::Turn(Turn::Right));
                }
                c => panic!("Invalid char in moves {c}"),
            }
        }
        if buffer.len() > 0 {
            let num = buffer.parse().unwrap();
            moves.push(Move::Forward(num));
        }

        Self {
            map,
            facing: Facing::Right,
            position,
            moves,
        }
    }

    fn do_moves(&mut self) {
        for m in &self.moves {
            match m {
                Move::Turn(t) => self.facing.turn(*t),
                Move::Forward(n) => {
                    for _ in 0..*n {
                        let new_pos = match self.facing {
                            Facing::Down => {
                                let mut r = self.position.r + 1;
                                if self.map.row_limits[self.position.c].1 < r {
                                    r = self.map.row_limits[self.position.c].0;
                                }
                                Pos {
                                    r,
                                    c: self.position.c,
                                }
                            }
                            Facing::Right => {
                                let mut c = self.position.c + 1;
                                if self.map.col_limits[self.position.r].1 < c {
                                    c = self.map.col_limits[self.position.r].0;
                                }
                                Pos {
                                    r: self.position.r,
                                    c,
                                }
                            }
                            Facing::Up => {
                                let mut r = self.position.r - 1;
                                if self.map.row_limits[self.position.c].0 > r {
                                    r = self.map.row_limits[self.position.c].1;
                                }
                                Pos {
                                    r,
                                    c: self.position.c,
                                }
                            }
                            Facing::Left => {
                                let mut c = self.position.c - 1;
                                if self.map.col_limits[self.position.r].0 > c {
                                    c = self.map.col_limits[self.position.r].1;
                                }
                                Pos {
                                    r: self.position.r,
                                    c,
                                }
                            }
                        };
                        if self.map.obstacles.contains(&new_pos) {
                            break;
                        } else {
                            self.position = new_pos;
                        }
                    }
                }
            }
        }
    }

    fn password(&self) -> usize {
        self.position.r * 1000 + self.position.c * 4 + self.facing as usize
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day22.txt").expect("Failed to read file");
    let mut board = Board::from_str(&input);
    board.do_moves();
    dbg!(board.password());
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = "        ....    
        .#..    
        #...    
        ....    
...#........    
........#...    
..#....#.#.#    
..........#.    
        ........
        .....#..
        .#......
        ......#.

LL1LL1";

    #[test]
    fn test_part1() {
        let mut board = Board::from_str(TEST_DATA);
        board.do_moves();
        dbg!(&board.position);
        dbg!(&board.facing);
        dbg!(board.password());
    }
}
