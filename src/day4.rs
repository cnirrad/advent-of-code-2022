use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

pub fn run(part: u8) -> Result<()> {
    if part == 1 {
        part1()
    } else {
        part2()
    }
}

fn part2() -> Result<()> {
    println!("Running day4::part2");

    let file = File::open("./resources/day4.txt").context("Could not find day4.txt")?;
    let reader = BufReader::new(file);

    let total =  reader
        .lines()
        .into_iter()
        .map(|line| parse_line(&line.unwrap()).unwrap())
        .filter(|(a1,a2)| overlap(a1, a2) )
        .count();

    println!("Number of pairs that overlap the other: {}", total);
    Ok(())
}

fn part1() -> Result<()> {
    println!("Running day4::part1");

    let file = File::open("./resources/day4.txt").context("Could not find day4.txt")?;
    let reader = BufReader::new(file);

    let total =  reader
        .lines()
        .into_iter()
        .map(|line| parse_line(&line.unwrap()).unwrap())
        .filter(|(a1,a2)| contains(a1, a2) )
        .count();

    println!("Number of pairs that contain the other: {}", total);
    Ok(())
}


fn parse_line(line: &str) -> Result<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (a1, a2) = line.split_once(",").unwrap();
    let r1 = parse_range(a1)?;
    let r2 = parse_range(a2)?;

    Ok((r1,r2))
}

fn parse_range(range_str: &str) -> Result<RangeInclusive<u32>> {
    let (start_str, end_str) = range_str.split_once("-").unwrap();

    let start: u32 = start_str.parse()?;
    let end: u32 = end_str.parse()?;

    Ok(RangeInclusive::new(start, end))
}

fn contains(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool {

    if r1.contains(r2.start()) && r1.contains(r2.end()) {
        return true;
    }

    if r2.contains(r1.start()) && r2.contains(r1.end()) {
        return true;
    }

    false
}

fn overlap(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool {

    if r1.contains(r2.start()) || r1.contains(r2.end()) {
        return true;
    }
    if r2.contains(r1.start()) || r2.contains(r1.end()) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::day4::*;

    #[test]
    fn test_contains() {
        let r1 = RangeInclusive::new(1,10);
        let r2 = RangeInclusive::new(5,7);

        assert_eq!(true, contains(&r1, &r2));
        assert_eq!(true, contains(&r2, &r1));
    }

    #[test]
    fn test_not_contains() {
        let r1 = RangeInclusive::new(1,10);
        let r2 = RangeInclusive::new(15,27);

        assert_eq!(false, contains(&r1, &r2));
        assert_eq!(false, contains(&r2, &r1));
    }

    #[test]
    fn test_overlap() {
        let r1 = RangeInclusive::new(1,10);
        let r2 = RangeInclusive::new(5,17);

        assert_eq!(true, overlap(&r1, &r2));
        assert_eq!(true, overlap(&r2, &r1));
    }

    #[test]
    fn test_parse_line() {
        let (r1,r2) = parse_line("1-5,6-8").unwrap();

        assert_eq!(&1, r1.start());
        assert_eq!(&5, r1.end());
        assert_eq!(&6, r2.start());
        assert_eq!(&8, r2.end());
    }
}
