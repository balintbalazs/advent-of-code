use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
struct Operation {
    lhs: String,
    rhs: String,
    operator: Operator,
}
#[derive(Debug)]
enum MonkeyJob {
    Shout(i64),
    Calculate(Operation),
}

#[derive(Debug)]
struct Monkeys(HashMap<String, MonkeyJob>);

impl Monkeys {
    fn from_str(input: &str) -> Self {
        let mut monkeys = HashMap::new();

        for line in input.lines() {
            let mut words = line.split(' ');
            let id = words.next().unwrap().trim_end_matches(':').to_string();
            let next = words.next().unwrap();
            let job = if let Ok(num) = next.parse::<i64>() {
                MonkeyJob::Shout(num)
            } else {
                let lhs = next.to_string();
                let operator = match words.next().unwrap() {
                    "+" => Operator::Add,
                    "-" => Operator::Sub,
                    "*" => Operator::Mul,
                    "/" => Operator::Div,
                    op => panic!("invalid operator {op}"),
                };
                let rhs = words.next().unwrap().to_string();
                MonkeyJob::Calculate(Operation { lhs, rhs, operator })
            };

            monkeys.insert(id, job);
        }

        Self(monkeys)
    }

    fn shout(&self, id: &str) -> i64 {
        let monkey_job = self.0.get(id).unwrap();

        let num = match monkey_job {
            MonkeyJob::Shout(num) => *num,
            MonkeyJob::Calculate(op) => {
                let lhs = self.shout(&op.lhs);
                let rhs = self.shout(&op.rhs);
                match op.operator {
                    Operator::Add => lhs + rhs,
                    Operator::Sub => lhs - rhs,
                    Operator::Mul => lhs * rhs,
                    Operator::Div => lhs / rhs,
                }
            }
        };

        num
    }

    fn path_to_humn(&self, id: &str) -> Option<VecDeque<String>> {
        if id == "humn" {
            return Some([id.to_string()].into_iter().collect());
        };
        let monkey_job = self.0.get(id).unwrap();

        match &monkey_job {
            MonkeyJob::Shout(_) => None,
            MonkeyJob::Calculate(op) => {
                if let Some(mut path) = self.path_to_humn(&op.lhs) {
                    path.push_front(id.to_string());
                    Some(path)
                } else if let Some(mut path) = self.path_to_humn(&op.rhs) {
                    path.push_front(id.to_string());
                    Some(path)
                } else {
                    None
                }
            }
        }
    }

    fn solve_for_humn(&self, mut path_to_humn: VecDeque<String>, target: i64) -> i64 {
        let current_node = path_to_humn.pop_front().unwrap();
        if current_node == "humn" {
            println!("humn should be: {}", target);
            return target;
        }

        let current_node = self.0.get(&current_node).unwrap();
        let op = match current_node {
            MonkeyJob::Shout(_) => {
                panic!("All non-humn nodes in path should have an operation instead of a number")
            }
            MonkeyJob::Calculate(op) => op,
        };
        if op.lhs == path_to_humn[0] {
            // lhs op rhs = target
            let rhs = self.shout(&op.rhs);
            let lhs = match op.operator {
                Operator::Add => target - rhs,
                Operator::Sub => target + rhs,
                Operator::Mul => target / rhs,
                Operator::Div => target * rhs,
            };
            self.solve_for_humn(path_to_humn, lhs)
        } else if op.rhs == path_to_humn[0] {
            let lhs = self.shout(&op.lhs);
            let rhs = match op.operator {
                Operator::Add => target - lhs,
                Operator::Sub => lhs - target,
                Operator::Mul => target / lhs,
                Operator::Div => lhs / target,
            };
            self.solve_for_humn(path_to_humn, rhs)
        } else {
            panic!("no path to human from root");
        }
    }
}

fn part2(monkeys: &mut Monkeys) -> i64 {
    let mut path_to_humn = monkeys.path_to_humn("root").unwrap();
    path_to_humn.pop_front(); //remove root

    let root = monkeys.0.get("root").unwrap();
    let op = match root {
        MonkeyJob::Shout(_) => panic!("Root should have something to calculate"),
        MonkeyJob::Calculate(op) => op,
    };
    if op.lhs == path_to_humn[0] {
        let target = monkeys.shout(&op.rhs);
        monkeys.solve_for_humn(path_to_humn, target)
    } else if op.rhs == path_to_humn[0] {
        let target = monkeys.shout(&op.lhs);
        monkeys.solve_for_humn(path_to_humn, target)
    } else {
        panic!("no path to human from root");
    }
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day21.txt").expect("Failed to read file");
    let mut monkeys = Monkeys::from_str(&input);
    dbg!(monkeys.shout("root"));
    dbg!(part2(&mut monkeys));
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_part1() {
        let monkeys = Monkeys::from_str(TEST_DATA);
        assert_eq!(152, monkeys.shout("root"));
    }

    #[test]
    fn test_part2() {
        let mut monkeys = Monkeys::from_str(TEST_DATA);
        assert_eq!(301, part2(&mut monkeys));
    }
}
