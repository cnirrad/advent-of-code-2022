use anyhow::Result;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Node {
    Beacon,
    Sensor { closest_beacon: Coord },
    Empty,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Beacon => write!(f, "B"),
            Node::Sensor { closest_beacon: _ } => write!(f, "S"),
            Node::Empty => write!(f, "#"),
        }
    }
}

pub struct Sensor {
    location: Coord,

    /// distance to beacon
    distance: i32,
}

impl Sensor {
    pub fn new(location: Coord, beacon: Coord) -> Self {
        Sensor {
            location,
            distance: location.manhattan(&beacon),
        }
    }

    pub fn in_range(&self, x: i32, y: i32) -> i32 {
        let dist = self.location.manhattan(&Coord::new(x, y));

        self.distance - dist
    }
}

pub fn scan_line(sensors: &[Sensor], line: i32, min: i32, max: i32) -> Option<Coord> {
    let mut x = min;
    'outer: loop {
        for sensor in sensors {
            let dx = sensor.in_range(x, line);

            if dx >= 0 {
                x += dx + 1;
                if x > max {
                    return None;
                }
                continue 'outer;
            }
        }
        return Some(Coord::new(x, line));
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Coord(i32, i32);

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord(x, y)
    }
    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
    }
    pub fn set_x(&mut self, x: i32) {
        self.0 = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.1 = y;
    }
    pub fn manhattan(&self, other: &Coord) -> i32 {
        let x = self.0 - other.0;
        let y = self.1 - other.1;

        x.abs() + y.abs()
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct TunnelMap {
    grid: HashMap<Coord, Node>,

    top_left: Coord,
    bottom_right: Coord,
}

impl TunnelMap {
    pub fn new(input: &[(Coord, Coord)]) -> Self {
        let mut grid = HashMap::new();

        let mut top_left = Coord::new(i32::MAX, i32::MAX);
        let mut bottom_right = Coord::new(0, 0);

        // For each line of input
        for (sensor, beacon) in input.iter() {
            // Add the beacon and sensor
            grid.insert(
                *sensor,
                Node::Sensor {
                    closest_beacon: *beacon,
                },
            );
            grid.insert(*beacon, Node::Beacon);

            // update boundries
            if top_left.x() > sensor.x().min(beacon.x()) {
                top_left.set_x(sensor.x().min(beacon.x()));
            } else if bottom_right.x() < sensor.x().max(beacon.x()) {
                bottom_right.set_x(sensor.x().max(beacon.x()));
            }
            if top_left.y() > sensor.y().min(beacon.y()) {
                top_left.set_y(sensor.y().min(beacon.y()));
            } else if bottom_right.y() < sensor.y().max(beacon.y()) {
                bottom_right.set_y(sensor.y().max(beacon.y()));
            }
        }

        TunnelMap {
            grid,
            top_left,
            bottom_right,
        }
    }

    pub fn count_not_beacon(&mut self, line: i32) -> Result<usize> {
        let mut count = 0;
        for x in self.top_left.x()..=self.bottom_right.x() {
            let c = Coord::new(x, line);
            if let Some(n) = self.grid.get(&c) {
                if *n != Node::Beacon {
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    pub fn add_map_feature(&mut self, coord: Coord, node: Node) {
        if self.grid.contains_key(&coord) {
            return;
        }
        self.grid.insert(coord, node);

        if coord.x() < self.top_left.x() {
            self.top_left.set_x(coord.x());
        } else if coord.x() > self.bottom_right.x() {
            self.bottom_right.set_x(coord.x());
        }
        if coord.y() < self.top_left.y() {
            self.top_left.set_y(coord.y());
        } else if coord.y() > self.bottom_right.y() {
            self.bottom_right.set_y(coord.y());
        }
    }
}

impl Display for TunnelMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.top_left.manhattan(&self.bottom_right) > 100 {
            writeln!(f, "Too large to show")?;
        } else {
            for y in self.top_left.y()..=self.bottom_right.y() {
                write!(f, "{:05} ", y)?;

                for x in self.top_left.x()..=self.bottom_right.x() {
                    match self.grid.get(&Coord::new(x, y)) {
                        Some(node) => write!(f, "{}", node)?,
                        None => write!(f, ".")?,
                    }
                }
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[allow(unused_imports)]
pub mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{i32, multispace1, newline, one_of, space1},
        combinator::{map, value},
        multi::{separated_list0, separated_list1},
        sequence::{delimited, preceded, separated_pair, terminated, tuple},
        Finish, IResult,
    };

    fn coord(s: &str) -> IResult<&str, Coord> {
        let x = preceded(tag("x="), i32);
        let y = preceded(tag("y="), i32);

        map(separated_pair(x, tag(", "), y), |(x, y)| Coord::new(x, y))(s)
    }

    fn sensor_beacon_pair(s: &str) -> IResult<&str, (Coord, Coord)> {
        let sensor = preceded(tag("Sensor at "), coord);
        let beacon = preceded(tag("closest beacon is at "), coord);

        separated_pair(sensor, tag(": "), beacon)(s)
    }

    pub fn parse(s: &str) -> Result<Vec<(Coord, Coord)>> {
        let result = separated_list1(tag("\n"), sensor_beacon_pair)(s);
        let (_, data) = result
            .finish()
            .map_err(|e| anyhow!("Failed to parse packet data with error {e}"))?;

        Ok(data)
    }

    #[cfg(test)]
    mod tests {
        use super::{parser, *};

        #[test]
        fn test_parse() {
            let line = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";

            let result = &parse(line).unwrap()[0];

            assert_eq!(Coord::new(2, 18), result.0);
            assert_eq!(Coord::new(-2, 15), result.1);
        }

        #[test]
        fn test_manhattan() {
            let c1 = Coord::new(1, 1);
            let c2 = Coord::new(10, 5);

            assert_eq!(13, c1.manhattan(&c2));
        }
    }
}
