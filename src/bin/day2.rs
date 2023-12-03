use std::fs;

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day2.txt").expect("Failed to read file");
    let lines = input.lines();
    let lines1 = lines.clone();
    let mut ids_of_possible_games = vec![];
    'outer: for line in lines1 {
        // id is the same as index + 1
        // split
        let mut tokens = line.split(": ");
        let game_id = tokens.next().unwrap();
        let sets = tokens.next().unwrap();
        // parse game id
        let game_id: u32 = game_id.split(' ').nth(1).unwrap().parse().unwrap();
        // parse sets
        let sets = sets.split(&[';', ',']).map(str::trim);
        for set in sets {
            if let [number, color] = set.split(' ').collect::<Vec<_>>()[..] {
                let number: u32 = number.parse().unwrap();
                // if any color in any set has too high count, skip adding
                // game id to list and continue with outer loop
                match color {
                    "red" => {
                        if number > 12 {
                            continue 'outer;
                        }
                    }
                    "green" => {
                        if number > 13 {
                            continue 'outer;
                        }
                    }
                    "blue" => {
                        if number > 14 {
                            continue 'outer;
                        }
                    }
                    a => panic!("unknown color: {a}"),
                }
            }
        }
        // all sets are possible, save game id
        ids_of_possible_games.push(game_id);
    }
    let part1: u32 = ids_of_possible_games.iter().sum();
    dbg!(part1);

    let mut powers = vec![];
    for line in lines {
        // id is the same as index + 1
        // split
        let mut tokens = line.split(": ");
        let _game_id = tokens.next().unwrap(); // ignore game id for part 2
        let sets = tokens.next().unwrap();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        // parse sets
        let sets = sets.split(&[';', ',']).map(str::trim);
        for set in sets {
            if let [number, color] = set.split(' ').collect::<Vec<_>>()[..] {
                let number: u32 = number.parse().unwrap();
                match color {
                    "red" => {
                        if number > red {
                            red = number;
                        }
                    }
                    "green" => {
                        if number > green {
                            green = number;
                        }
                    }
                    "blue" => {
                        if number > blue {
                            blue = number;
                        }
                    }
                    a => panic!("unknown color: {a}"),
                }
            }
        }
        powers.push(red * green * blue);
    }
    let part2: u32 = powers.iter().sum();
    dbg!(part2);
}
