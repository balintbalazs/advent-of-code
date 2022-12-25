use std::{
    fmt::Debug,
    fs,
    iter::Sum,
    ops::{Add, Index},
    str::FromStr,
};

#[derive(Clone, PartialEq, Eq)]
struct Snafu(Vec<i32>);

#[derive(Debug, PartialEq, Eq)]
enum ParseSnafuError {
    InvalidDigit(char),
}

impl FromStr for Snafu {
    type Err = ParseSnafuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num = vec![];
        for c in s.trim().chars().rev() {
            let digit = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                d => return Err(ParseSnafuError::InvalidDigit(d)),
            };
            num.push(digit);
        }
        while num.last() == Some(&0) {
            num.pop();
        }
        Ok(Self(num))
    }
}

impl Debug for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in self.0.iter().rev() {
            let d = match digit {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => unimplemented!(),
            };
            write!(f, "{}", d)?;
        }
        Ok(())
    }
}

impl Index<usize> for Snafu {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.0.len() {
            &self.0[index]
        } else {
            &0
        }
    }
}

impl Add for Snafu {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let result_length = self.0.len().max(rhs.0.len()) + 1; // add one in case of carry
        let mut result = vec![0; result_length];
        let mut carry = 0;
        for i in 0..result_length {
            let mut sum = carry + self[i] + rhs[i];
            carry = 0;
            while sum > 2 {
                carry += 1;
                sum -= 5;
            }
            while sum < -2 {
                carry -= 1;
                sum += 5;
            }
            result[i] = sum;
        }
        if result.last() == Some(&0) {
            result.pop();
        }

        Self(result)
    }
}

impl Sum<Snafu> for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Snafu(vec![0]);
        for num in iter {
            sum = sum + num;
        }
        sum
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day25.txt").expect("Failed to read file");
    let nums: Result<Vec<_>, _> = input.lines().map(Snafu::from_str).collect();
    let nums = nums.unwrap();
    let part1: Snafu = nums.into_iter().sum();
    dbg!(part1);
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = "1=-0-2
    12111
    2=0=
    21
    2=01
    111
    20012
    112
    1=-1=
    1-12
    12
    1=
    122";

    #[test]
    fn test_part1() {
        let nums: Result<Vec<_>, _> = TEST_DATA.lines().map(Snafu::from_str).collect();
        let nums = nums.unwrap();
        dbg!(&nums);
        let sum: Snafu = nums.into_iter().sum();
        dbg!(&sum);
        assert_eq!("2=-1=0".parse::<Snafu>().unwrap(), sum);
    }
}
