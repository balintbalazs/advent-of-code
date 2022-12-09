use std::{
    collections::HashSet,
    fs,
    ops::{Add, Sub},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord(i32, i32);
#[derive(Clone, Copy)]
struct Step(i32, i32);

impl Sub for Coord {
    type Output = Step;

    fn sub(self, rhs: Self) -> Self::Output {
        Step(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add<Step> for Coord {
    type Output = Coord;

    fn add(self, rhs: Step) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Step {
    fn l_inf_norm(&self) -> i32 {
        i32::max(self.0.abs(), self.1.abs())
    }

    fn follow_step(&self) -> Self {
        if self.l_inf_norm() > 1 {
            let x = match self.0 {
                2 => 1,
                -2 => -1,
                n => n,
            };
            let y = match self.1 {
                2 => 1,
                -2 => -1,
                n => n,
            };
            Self(x, y)
        } else {
            Step(0,0)
        }
    }
}

fn count_tail_positions(input: &str) -> usize {
    let mut visited = HashSet::new();
    let start = Coord(0, 0);
    let mut head = start;
    let mut tail = start;
    visited.insert(tail);

    for line in input.lines() {
        let step = match &line[0..1] {
            "R" => Step(1, 0),
            "U" => Step(0, 1),
            "L" => Step(-1, 0),
            "D" => Step(0, -1),
            _ => unimplemented!(),
        };
        let num = line[2..].parse().unwrap();
        for _ in 0..num {
            let prev_head = head;
            head = head + step;
            if (head - tail).l_inf_norm() > 1 {
                tail = prev_head;
            }
            visited.insert(tail);
        }
    }
    visited.len()
}


fn count_nth_tail_positions(input: &str, n: usize) -> usize {
  let mut visited = HashSet::new();
  let start = Coord(0, 0);
  let mut rope = vec![start; n+1];
  visited.insert(start);

  for line in input.lines() {
      let step = match &line[0..1] {
          "R" => Step(1, 0),
          "U" => Step(0, 1),
          "L" => Step(-1, 0),
          "D" => Step(0, -1),
          _ => unimplemented!(),
      };
      let num = line[2..].parse().unwrap();
      for _ in 0..num {
          rope[0] = rope[0] + step;
          for i in 1..=n {
             let diff = rope[i-1] - rope[i];
            rope[i] = rope[i] + diff.follow_step();
          }
          visited.insert(rope[n]);
      }
  }
  visited.len()
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day9.txt").expect("Failed to read file");
    let count = count_tail_positions(&input);
    dbg!(count);
    let count_9th = count_nth_tail_positions(&input, 9);
    dbg!(count_9th);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = r###"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"###;

const LONGER_TEST_DATA: &str = r###"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"###;

    #[test]
    fn can_move_tail() {
        assert_eq!(13, count_tail_positions(TEST_DATA));
    }

    #[test]
    fn can_move_tail_alt() {
        assert_eq!(13, count_nth_tail_positions(TEST_DATA, 1));
    }

    #[test]
    fn can_move_9th_tail() {
        assert_eq!(1, count_nth_tail_positions(TEST_DATA, 9));
    }

    #[test]
    fn can_move_9th_tail_longer() {
        assert_eq!(36, count_nth_tail_positions(LONGER_TEST_DATA, 9));
    }
}
