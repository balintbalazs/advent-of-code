use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: u32,
    leads_to: Vec<String>,
}

fn from_str(line: &str) -> (String, Valve) {
    let mut words = line.split(" ");
    let name = words.nth(1).unwrap().to_string();
    let flow_rate = words
        .nth(2)
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .trim_end_matches(";")
        .parse()
        .unwrap();

    let leads_to: Vec<_> = words
        .skip(4)
        .map(|w| w.trim_end_matches(",").to_string())
        .collect();

    let valve = Valve {
        flow_rate,
        leads_to,
    };
    (name, valve)
}

fn create_graph(input: &str) -> HashMap<String, Valve> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let (name, valve) = from_str(line);
        graph.insert(name, valve);
    }
    graph
}

fn part1(mut graph: HashMap<String, Valve>, starting: &str, mut remaining_time: u32) -> u32 {
    let current_valve = graph.get_mut(starting).unwrap();

    if remaining_time == 0 {
        // println!("Ran out of time");
        return 0;
    }

    // dbg!(starting);
    // dbg!(remaining_time);

    let mut released_pressure = 0;

    // open valve
    if current_valve.flow_rate != 0 {
        remaining_time -= 1;
        released_pressure = remaining_time * current_valve.flow_rate;
        current_valve.flow_rate = 0;
    }

    let mut closed_valves: Vec<_> = graph
        .iter()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(k, _)| k)
        .collect();
    if closed_valves.len() == 0 {
        // println!("No more open valves");
        return released_pressure;
    }

    // find distances to all valves
    let mut distances = HashMap::new();
    let mut next_valves = VecDeque::<(String, u32)>::new();
    next_valves.push_back((starting.to_string(), 0));

    while let Some((valve, time_to_open)) = next_valves.pop_front() {
        if time_to_open == remaining_time {
            break;
        }
        if !distances.contains_key(&valve) {
            distances.insert(valve.clone(), time_to_open);
            let time_to_open = time_to_open + 1;

            for neighbor in &graph.get(&valve).unwrap().leads_to {
                next_valves.push_back((neighbor.to_owned(), time_to_open));
            }
        }
    }

    // dbg!(&distances);

    // visit closed valves, check which one returns the highest flow rate
    // depth first search with recursive call
    // dbg!(&closed_valves);
    let more_pressure = closed_valves
        .into_iter()
        .map(|valve| {
            // only visit next valve if there is enough time
            if let Some(distance) = distances.get(valve) {
                part1(graph.clone(), valve, remaining_time - distance)
            } else {
                0
            }
        })
        .max()
        .unwrap();

    released_pressure + more_pressure
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day16.txt").expect("Failed to read file");

    let graph = create_graph(&input);
    // dbg!(graph);

    dbg!(part1(graph, "AA", 30));
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn can_parse() {
        let graph = create_graph(TEST_DATA);

        assert_eq!(1651, part1(graph, "AA", 30));
    }
}
