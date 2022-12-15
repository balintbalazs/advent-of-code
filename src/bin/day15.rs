use std::{collections::HashSet, fs};

use scan_fmt::scan_fmt;

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    distance_to_closest_beacon: i64,
}

#[derive(Debug)]
struct Beacon {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn from_str(line: &str) -> (Sensor, Beacon) {
    let (sx, sy, bx, by) = scan_fmt!(
        line,
        "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}}",
        i64,
        i64,
        i64,
        i64
    )
    .unwrap();

    let sensor = Sensor {
        x: sx,
        y: sy,
        distance_to_closest_beacon: (sx - bx).abs() + (sy - by).abs(),
    };
    let beacon = Beacon { x: bx, y: by };

    (sensor, beacon)
}

fn part1(input: &Vec<(Sensor, Beacon)>, row: i64) -> usize {
    let mut points = HashSet::new();
    let mut occupied = HashSet::new();

    for (sensor, beacon) in input {
        if sensor.y == row {
            occupied.insert(sensor.x);
        }
        if beacon.y == row {
            occupied.insert(beacon.x);
        }
        let distance_from_row = (sensor.y - row).abs();
        if distance_from_row <= sensor.distance_to_closest_beacon {
            for dx in 0..=(sensor.distance_to_closest_beacon - distance_from_row) {
                points.insert(sensor.x + dx);
                points.insert(sensor.x - dx);
            }
        }
    }

    points.len() - occupied.len()
}

fn is_inside(sensor: &Sensor, point: &Point) -> bool {
    (sensor.x - point.x).abs() + (sensor.y - point.y).abs() <= sensor.distance_to_closest_beacon
}

fn just_outside(sensor: &Sensor) -> Vec<Point> {
    let mut points = vec![];
    let mut x = sensor.x - sensor.distance_to_closest_beacon - 1;
    let mut y = sensor.y;

    let dirs = vec![(1, 1), (1, -1), (-1, -1), (-1, 1)];

    for (dx, dy) in dirs {
        for _ in 0..sensor.distance_to_closest_beacon + 1 {
            points.push(Point { x, y });
            x += dx;
            y += dy;
        }
    }

    points
}

fn part2(sensors: &Vec<Sensor>, limits: Point) -> i64 {
    let outside_points: Vec<Point> = sensors
        .iter()
        .flat_map(just_outside)
        .filter(|point| point.x >= 0 && point.x <= limits.x && point.y >= 0 && point.y <= limits.y)
        .collect();

    for point in outside_points {
        let inside = sensors.iter().any(|sensor| is_inside(sensor, &point));
        if !inside {
            return point.x * 4_000_000 + point.y;
        }
    }

    unreachable!("should return before")
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day15.txt").expect("Failed to read file");
    let input: Vec<_> = input.lines().map(from_str).collect();
    dbg!(part1(&input, 2_000_000));
    let sensors: Vec<_> = input.into_iter().map(|(sensor, _)| sensor).collect();
    dbg!(part2(
        &sensors,
        Point {
            x: 4_000_000,
            y: 4_000_000
        }
    ));
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn can_parse_input() {
        let input: Vec<_> = TEST_DATA.lines().map(from_str).collect();
        assert_eq!(26, part1(&input, 10));

        let sensors: Vec<_> = input.into_iter().map(|(sensor, _)| sensor).collect();
        assert_eq!(56000011, part2(&sensors, Point { x: 20, y: 20 }));
    }
}
