use anyhow::Result;

use crate::{
    day15::data::{scan_line, Coord, Sensor, TunnelMap},
    utils::read_file,
};

mod data;
use data::parser::parse;

pub fn run(part: u8) -> Result<()> {
    let lines = read_file("./resources/day15.txt")?;

    if part == 1 {
        part1(&lines, 2000000)?;
    } else {
        part2(&lines, 4000000)?;
    }

    Ok(())
}

fn part1(lines: &str, line: i32) -> Result<usize> {
    println!("Running day15::part1");

    let input = parse(lines)?;

    let mut cave_map = TunnelMap::new(&input);

    for (sensor, beacon) in input {
        let dist_to_beacon = sensor.manhattan(&beacon);
        let dist_to_line = sensor.manhattan(&Coord::new(sensor.x(), line));

        let dx = dist_to_beacon - dist_to_line;

        if dx <= 1 {
            continue;
        }

        let mut coord = Coord::new(sensor.x() - dx, line);

        // println!(
        //     "Sensor {} has a distance of {} to beacon {}. Will reach toward {}",
        //     sensor, dist_to_beacon, beacon, coord
        // );

        while sensor.manhattan(&coord) <= dist_to_beacon {
            cave_map.add_map_feature(coord, data::Node::Empty);

            coord.set_x(coord.x() + 1);
        }
    }
    let count = cave_map.count_not_beacon(line)?;

    println!("{}", cave_map);

    println!(
        "Number of positions where beacon CANNOT be deployed: {}",
        count
    );
    Ok(count)
}

fn part2(lines: &str, max_bound: i32) -> Result<i64> {
    println!("Running day15::part2");

    let input = parse(lines)?;

    let sensors: Vec<Sensor> = input.iter().map(|i| Sensor::new(i.0, i.1)).collect();

    for y in 0..max_bound {
        if y % 50000 == 0 {
            println!("Processed {}/{} lines", y, max_bound);
        }

        if let Some(c) = scan_line(&sensors, y, 0, max_bound) {
            let tuning_freq = c.x() as i64 * 4000000 + c.y() as i64;

            println!("Tuning freq: {}", tuning_freq);
            return Ok(tuning_freq);
        }
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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
    fn test_part1_example() {
        assert_eq!(26, part1(EXAMPLE, 10).unwrap());
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(56000011, part2(EXAMPLE, 20).unwrap());
    }

    // Commented out because it is too slow
    // #[test]
    // fn test_part1() {
    //     let lines = read_file("./resources/day15.txt").unwrap();
    //     assert_eq!(5144286, part1(&lines, 2000000).unwrap());
    // }

    // #[test]
    // fn test_part2() {
    //     let lines = read_file("./resources/day15.txt").unwrap();
    //     assert_eq!(10229191267339, part2(&lines, 4000000).unwrap());
    // }
}
