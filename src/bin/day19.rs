use std::{fs, ops::{Add, Sub}};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct RockTypes {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Add for RockTypes {
    type Output = RockTypes;

    fn add(self, rhs: Self) -> Self::Output {
        RockTypes {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Sub for RockTypes {
    type Output = RockTypes;

    fn sub(self, rhs: Self) -> Self::Output {
        RockTypes {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

#[derive(Debug, Clone)]
struct Costs {
    ore_robot: RockTypes,
    clay_robot: RockTypes,
    obsidian_robot: RockTypes,
    geode_robot: RockTypes,
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: u32,
    costs: Costs,
    robots: RockTypes,
    resources: RockTypes,
}

impl Blueprint {
    fn from_str(input: &str) -> Self {
        let mut costs = [[0; 4]; 4];
        let mut words = input.split(' ');
        let id = words.nth(1).unwrap().trim_end_matches(':').parse().unwrap();
        let ore_robot_cost = RockTypes {
            ore: words.nth(4).unwrap().parse().unwrap(),
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        let clay_robot_cost = RockTypes {
            ore: words.nth(5).unwrap().parse().unwrap(),
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        let obsidian_robot_cost = RockTypes {
            ore: words.nth(5).unwrap().parse().unwrap(),
            clay: words.nth(2).unwrap().parse().unwrap(),
            obsidian: 0,
            geode: 0,
        };
        
        let geode_robot_cost = RockTypes {
            ore: words.nth(5).unwrap().parse().unwrap(),
            clay: words.nth(2).unwrap().parse().unwrap(),
            obsidian: 0,
            geode: 0,
        };

        Self {
            id,
            costs: Costs {
                ore_robot: ore_robot_cost,
                clay_robot: clay_robot_cost,
                obsidian_robot: obsidian_robot_cost,
                geode_robot: geode_robot_cost,
            },
            robots: RockTypes { ore: 1, clay: 0, obsidian: 0, geode: 0 },
            resources: RockTypes { ore: 0, clay: 0, obsidian: 0, geode: 0 },
        }
    }

    fn collect_resources(&mut self, time: u32) {
        for _ in 0..time {
            self.resources = self.resources + self.robots;
        }
    }

    fn build_robots_crack_geodes(&self, mut time: u32) -> u32 {
        // let options = [0,0,0,0];

        // if self.robots[2] > 0 {
        //     // wait to build a geode robot
        // }
        // if self.robots[1] > 0 {

        // }
        todo!()

    }
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day19.txt").expect("Failed to read file");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn can_parse() {
        let blueprint = Blueprint::from_str(&TEST_DATA.lines().next().unwrap());
        dbg!(&blueprint);
        dbg!(blueprint.build_robots_crack_geodes(24));
    }
}
