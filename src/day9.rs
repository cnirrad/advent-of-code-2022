use std::collections::HashSet;

use anyhow::Result;

use crate::utils::read_file_as_string_vec;

pub fn run(part: u8) -> Result<()> {
    if part == 1 {
        part1()?;
    } else {
        part2()?;
    }

    Ok(())
}

struct RopeSimulation {
    knots: Vec<(i32, i32)>,

    visited: HashSet<(i32, i32)>,
}

impl RopeSimulation {
    fn new(num_knots: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        let mut knots = Vec::new();
        for _ in 0..num_knots {
            knots.push((0, 0));
        }

        RopeSimulation { knots, visited }
    }

    fn simulate(&mut self, instructions: &Vec<String>) -> Result<()> {
        let instructions: Vec<(&str, &str)> = instructions
            .iter()
            .map(|i| i.split_once(" ").unwrap())
            .collect();

        for (dir, steps) in instructions {
            //
            let steps: usize = steps.parse()?;
            self.move_head(dir, steps);
        }

        Ok(())
    }

    fn move_head(&mut self, dir: &str, steps: usize) {
        // get a unit vector of movement
        let (x, y) = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!("Found invalid direction: {}", dir),
        };

        println!("Executing {} {}...", dir, steps);
        for _ in 0..steps {
            let (cx, cy) = self.knots[0];
            self.move_knot(0, cx + x, cy + y);

            let tail = self.knots[self.knots.len() - 1];
            let inserted = self.visited.insert(tail);

            println!(
                "Tail is now at ({},{}). new location = {}",
                tail.0, tail.1, inserted
            );
        }
    }

    fn move_knot(&mut self, knot_idx: usize, x: i32, y: i32) {
        println!("Knot {} moved to  ({}, {}).", knot_idx, x, y);
        self.knots[knot_idx] = (x, y);

        if knot_idx != self.knots.len() - 1 {
            let next = self.knots[knot_idx + 1];
            if !adjacent(&self.knots[knot_idx], &next) {
                // determine appropriate movement
                let mut new_x = next.0;
                let mut new_y = next.1;

                if next.0 - x > 0 {
                    new_x -= 1;
                } else if next.0 - x < 0 {
                    new_x += 1;
                }

                if next.1 - y > 0 {
                    new_y -= 1;
                } else if next.1 - y < 0 {
                    new_y += 1;
                }

                self.move_knot(knot_idx + 1, new_x, new_y);
            }
        }
    }

    fn num_visited(&self) -> usize {
        self.visited.len()
    }
}

fn part1() -> Result<usize> {
    println!("Running day9::part1");

    let lines = read_file_as_string_vec("./resources/day9.txt")?;

    let mut sim = RopeSimulation::new(2);

    sim.simulate(&lines)?;

    println!(
        "The tail visited a total of {} locations",
        sim.num_visited()
    );

    Ok(sim.num_visited())
}

fn part2() -> Result<usize> {
    println!("Running day9::part2");

    let lines = read_file_as_string_vec("./resources/day9.txt")?;

    let mut sim = RopeSimulation::new(10);

    sim.simulate(&lines)?;

    println!(
        "The tail visited a total of {} locations",
        sim.num_visited()
    );

    Ok(sim.num_visited())
}

/// Tests that the 2d coords are adjacent to each other (within 1, vertically, horizontally, or diagnolly)
fn adjacent((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> bool {
    let adj_x = (x1 - x2).abs() <= 1;
    let adj_y = (y1 - y2).abs() <= 1;

    adj_x && adj_y
}

#[cfg(test)]
mod tests {
    use crate::day9::*;

    const EXAMPLE_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const EXAMPLE_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        assert_eq!(5902, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(2445, part2().unwrap());
    }

    #[test]
    fn test_example1_part1() {
        let v: Vec<String> = EXAMPLE_1.split("\n").map(|s| s.to_owned()).collect();

        let mut sim = RopeSimulation::new(2);
        sim.simulate(&v).unwrap();

        assert_eq!(13, sim.num_visited());
    }

    #[test]
    fn test_example1_part2() {
        let v: Vec<String> = EXAMPLE_1.split("\n").map(|s| s.to_owned()).collect();

        let mut sim = RopeSimulation::new(10);
        sim.simulate(&v).unwrap();

        assert_eq!(1, sim.num_visited());
    }

    #[test]
    fn test_example2_part2() {
        let v: Vec<String> = EXAMPLE_2.split("\n").map(|s| s.to_owned()).collect();

        let mut sim = RopeSimulation::new(10);
        sim.simulate(&v).unwrap();

        println!("visited = {:?}", sim.visited);

        assert_eq!(36, sim.num_visited());
    }

    #[test]
    fn test_adjacent() {
        assert!(adjacent(&(5, 5), &(5, 5)));
        assert!(adjacent(&(4, 5), &(5, 5)));
        assert!(adjacent(&(4, 5), &(5, 6)));
        assert!(adjacent(&(-1, -2), &(-2, -1)));

        assert!(!adjacent(&(1, -2), &(-2, -1)));
    }
}
