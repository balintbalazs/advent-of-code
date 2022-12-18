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

    let closed_valves: Vec<_> = graph
        .iter()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(k, _)| k)
        .collect();
    if closed_valves.len() == 0 {
        println!("No more open valves");
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

fn part2(mut graph: HashMap<String, Valve>, me: (&str, u32), elephant: (&str, u32)) -> u32 {
    // dbg!(me);
    // dbg!(elephant);
    let mut released_pressure = 0;

    let mut remaining_time_me = me.1;

    let current_valve_me = graph.get_mut(me.0).unwrap();
    if current_valve_me.flow_rate != 0 && remaining_time_me != 0 {
        remaining_time_me -= 1;
        released_pressure += remaining_time_me * current_valve_me.flow_rate;
        current_valve_me.flow_rate = 0;
    }

    let mut remaining_time_el = elephant.1;

    let current_valve_el = graph.get_mut(elephant.0).unwrap();
    if current_valve_el.flow_rate != 0 && remaining_time_el != 0 {
        remaining_time_el -= 1;
        released_pressure += remaining_time_el * current_valve_el.flow_rate;
        current_valve_el.flow_rate = 0;
    }

    let closed_valves: Vec<_> = graph
        .iter()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(k, _)| k)
        .collect();
    if closed_valves.len() == 0 {
        // println!("No more open valves");
        return released_pressure;
    }

    let mut distances_me = HashMap::new();
    let mut next_valves = VecDeque::<(String, u32)>::new();
    next_valves.push_back((me.0.to_string(), 0));

    while let Some((valve, time_to_open)) = next_valves.pop_front() {
        if time_to_open >= remaining_time_me {
            break;
        }
        if !distances_me.contains_key(&valve) {
            distances_me.insert(valve.clone(), time_to_open);
            let time_to_open = time_to_open + 1;

            for neighbor in &graph.get(&valve).unwrap().leads_to {
                next_valves.push_back((neighbor.to_owned(), time_to_open));
            }
        }
    }

    let mut distances_el = HashMap::new();
    let mut next_valves = VecDeque::<(String, u32)>::new();
    next_valves.push_back((elephant.0.to_string(), 0));

    while let Some((valve, time_to_open)) = next_valves.pop_front() {
        if time_to_open >= remaining_time_el {
            break;
        }
        if !distances_el.contains_key(&valve) {
            distances_el.insert(valve.clone(), time_to_open);
            let time_to_open = time_to_open + 1;

            for neighbor in &graph.get(&valve).unwrap().leads_to {
                next_valves.push_back((neighbor.to_owned(), time_to_open));
            }
        }
    }

    let mut max_additional_pressure = 0;
    for valve_me in &closed_valves {
        if me.0 == "AA" {
            dbg!(valve_me);
        }
        for valve_el in &closed_valves {
            if valve_el == valve_me {
                continue;
            }
            let p = match (distances_me.get(*valve_me), distances_el.get(*valve_el)) {
                (Some(distance_me), Some(distance_el)) => {
                    if let Some(swap_distance) = distances_me.get(*valve_el) {
                        if swap_distance < distance_me {
                            continue;
                        }
                    }
                    part2(
                        graph.clone(),
                        (valve_me, remaining_time_me - distance_me),
                        (valve_el, remaining_time_el - distance_el),
                    )
                }
                (None, Some(distance_el)) => part2(
                    graph.clone(),
                    me,
                    (valve_el, remaining_time_el - distance_el),
                ),
                (Some(distance_me), None) => part2(
                    graph.clone(),
                    (valve_me, remaining_time_me - distance_me),
                    elephant,
                ),
                (None, None) => 0,
            };
            if p > max_additional_pressure {
                max_additional_pressure = p;
            }
        }
    }
    // dbg!(max_additional_pressure);

    released_pressure + max_additional_pressure
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day16.txt").expect("Failed to read file");

    let graph = create_graph(&input);
    // dbg!(graph);

    dbg!(part1(graph.clone(), "AA", 30));
    dbg!(part2(graph, ("AA", 26), ("AA", 26)));
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

    #[test]
    fn test_part2() {
        let graph = create_graph(TEST_DATA);

        dbg!(part2(graph, ("AA", 26), ("AA", 26)));
    }
}
