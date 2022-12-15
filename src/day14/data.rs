use anyhow::Result;
use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructureType {
    Air,
    Rock,
    Sand,
}

impl Display for StructureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructureType::Air => write!(f, "."),
            StructureType::Rock => write!(f, "#"),
            StructureType::Sand => write!(f, "o"),
        }
    }
}

pub type Coord = (usize, usize);

#[derive(Debug)]
pub struct Scan {
    grid: HashMap<Coord, StructureType>,

    source: Coord,

    min_x: usize,
    max_x: usize,
    max_y: usize,

    with_floor: bool,
}

impl Scan {
    pub fn new(input: Vec<Vec<Coord>>) -> Self {
        let mut grid = HashMap::new();
        let mut min_x = 500;
        let mut max_x = 0;
        let mut max_y = 0;

        // For each line of input
        for line in input {
            // Add the rock to the grid, while keeping track of min/max values
            for (from, to) in line.iter().tuple_windows() {
                let start = from.0.min(to.0);
                let end = from.0.max(to.0);
                for x in start..=end {
                    if x > max_x {
                        max_x = x;
                    }
                    if x < min_x {
                        min_x = x;
                    }

                    let starty = from.1.min(to.1);
                    let endy = from.1.max(to.1);
                    for y in starty..=endy {
                        if y > max_y {
                            max_y = y;
                        }

                        println!("\tRock at ({x},{y})");
                        grid.insert((x, y), StructureType::Rock);
                    }
                }
            }
        }

        Scan {
            grid,
            source: (500, 0),
            min_x,
            max_x,
            max_y,
            with_floor: false,
        }
    }

    pub fn add_floor(&mut self) {
        self.with_floor = true;
        self.max_y += 2;
    }

    fn is_blocked(&self, coord: &Coord) -> bool {
        if self.with_floor && (coord.1 >= self.max_y) {
            true
        } else {
            let tile = self.grid.get(coord).unwrap_or(&StructureType::Air);

            tile != &StructureType::Air
        }
    }

    pub fn simulate_grain_of_sand(&mut self, start: &Coord) -> Result<(bool, Coord)> {
        let (x, mut y) = start;

        // while falling straight down...
        y += 1;
        while !self.is_blocked(&(*x, y)) {
            y += 1;
        }

        // If there is no floor, check to see if it goes to the abyss
        if !self.with_floor {
            if y - 1 == self.max_y {
                // Abyss!
                println!("sand went down into abyss at ({x},{y})");
                return Ok((true, (*x, y)));
            }

            // Check bounds before we go left
            if *x == 0 || *x == self.min_x {
                // Abyss!
                println!("sand went over into abyss at ({x},{y})");
                return Ok((true, (*x, y)));
            }
        }

        if !self.is_blocked(&(x - 1, y)) {
            return self.simulate_grain_of_sand(&(x - 1, y));
        } else if !self.is_blocked(&(x + 1, y)) {
            return self.simulate_grain_of_sand(&(x + 1, y));
        }

        // We have come to a rest. Need to back y up by one so that it sits atop of the rock that we ran into.
        y -= 1;
        self.grid.insert((*x, y), StructureType::Sand);

        Ok((false, (*x, y)))
    }
}

impl Display for Scan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Show one extra column on each side
        let start_x = self.min_x.saturating_sub(1);
        for y in 0..=self.max_y {
            for x in start_x..=self.max_x + 1 {
                if x == self.source.0 && y == self.source.1 {
                    write!(f, "+")?;
                } else {
                    let d = self.grid.get(&(x, y)).unwrap_or(&StructureType::Air);
                    write!(f, "{}", d)?;
                }
            }
            writeln!(f)?;
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
        character::complete::{multispace1, newline, one_of, space1, u32},
        combinator::{map, value},
        multi::{separated_list0, separated_list1},
        sequence::{delimited, preceded, separated_pair, terminated, tuple},
        Finish, IResult,
    };

    fn coord(s: &str) -> IResult<&str, Coord> {
        map(separated_pair(u32, tag(","), u32), |(x, y)| {
            (x as usize, y as usize)
        })(s)
    }

    fn paths(s: &str) -> IResult<&str, Vec<Coord>> {
        map(separated_list0(tag(" -> "), coord), Vec::from)(s)
    }

    pub fn parse(s: &str) -> Result<Vec<Vec<Coord>>> {
        let result = separated_list1(tag("\n"), paths)(s);
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
            let line = "497,29 -> 497,32 -> 493,32 -> 493,38\n507,38 -> 507,32 -> 501,32 -> 501,29";

            let result = &parse(line).unwrap()[0];

            assert_eq!((497, 29), result[0]);
            assert_eq!((497, 32), result[1]);

            let result = &parse(line).unwrap()[1];

            assert_eq!((507, 38), result[0]);
            assert_eq!((507, 32), result[1]);
        }
    }
}
