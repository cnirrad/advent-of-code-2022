use anyhow::Result;

use crate::utils::read_file;

mod data;
use data::parser::parse;

pub fn run(part: u8) -> Result<()> {
    let lines = read_file("./resources/day14.txt")?;

    if part == 1 {
        part1(&lines)?;
    } else {
        part2(&lines)?;
    }

    Ok(())
}

fn part1(lines: &str) -> Result<usize> {
    println!("Running day14::part1");

    let input = parse(lines)?;

    let mut scan = data::Scan::new(input);

    let mut count = 0;
    let mut end = false;

    while !end {
        (end, _) = scan.simulate_grain_of_sand(&(500, 0))?;
        count += 1;
    }
    // need to subtract 1 from our count
    count -= 1;

    println!("Scan: \n{}", scan);

    println!("Amount of sand before it went to the abyss: {}", count);
    Ok(count)
}

fn part2(lines: &str) -> Result<usize> {
    println!("Running day14::part2");

    let input = parse(lines)?;

    let mut scan = data::Scan::new(input);
    scan.add_floor();

    let mut count = 0;
    let mut end = false;
    let mut coord = (0, 0);

    while !end && coord != (500, 0) {
        (end, coord) = scan.simulate_grain_of_sand(&(500, 0))?;
        count += 1;

        // if count % 50 == 0 {
        //     println!("After {} iterations:\n{}\n\n", count, scan);
        // }
    }

    println!("Scan: \n{}", scan);

    println!("Amount of sand before it filled up: {}", count);
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1_example() {
        assert_eq!(24, part1(EXAMPLE).unwrap());
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(93, part2(EXAMPLE).unwrap());
    }

    #[test]
    fn test_part1() {
        let lines = read_file("./resources/day14.txt").unwrap();
        assert_eq!(672, part1(&lines).unwrap());
    }

    #[test]
    fn test_part2() {
        let lines = read_file("./resources/day14.txt").unwrap();
        assert_eq!(26831, part2(&lines).unwrap());
    }
}
