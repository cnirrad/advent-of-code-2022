use anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(part: u8) -> Result<()> {
    if part == 1 {
        part1()
    } else {
        part2()
    }
}

fn part2() -> Result<()> {
    println!("Running day3::part2");

    let file = File::open("./resources/day3.txt").context("Could not find day3.txt")?;
    let reader = BufReader::new(file);

    let total: u16 = reader
        .lines()
        .into_iter()
        .batching(|it| Some((it.next()?, it.next()?, it.next()?)))
        .map(|g| find_badge_in_group(&g.0.unwrap(), &g.1.unwrap(), &g.2.unwrap()))
        .map(|c| item_priority(c.unwrap()))
        .sum();

    println!("Sum of priorities = {}", total);
    Ok(())
}

fn part1() -> Result<()> {
    println!("Running day3::part1");

    let file = File::open("./resources/day3.txt").context("Could not find day3.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line?;

        let half = line.len() / 2;
        let (first, second) = line.split_at(half);
        let dup = find_dup(first, second)?;

        total += item_priority(dup);
    }

    println!("Sum of priorities = {}", total);
    Ok(())
}

fn find_badge_in_group(s1: &str, s2: &str, s3: &str) -> Result<char> {
    let set1: HashSet<char> = s1.chars().collect();
    let set2: HashSet<char> = s2.chars().collect();

    let dups1_2: String = set1.intersection(&set2).collect();
    //println!("dups between {} and {} = {}", s1, s2, dups1_2);

    let dup = find_dup(&dups1_2, s3)?;

    Ok(dup)
}

fn find_dup(s1: &str, s2: &str) -> Result<char> {
    let set1: HashSet<char> = s1.chars().collect();

    for ch in s2.chars() {
        if set1.contains(&ch) {
            return Ok(ch);
        }
    }

    bail!("Duplicate not found in strings {} and {}", s1, s2);
}

fn item_priority(ch: char) -> u16 {
    if ch.is_uppercase() {
        let p = ch as u16 - 'A' as u16;
        p + 27
    } else {
        let p = ch as u16 - 'a' as u16;
        p + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::{find_badge_in_group, find_dup, item_priority};

    #[test]
    fn test_item_priority() {
        assert_eq!(1, item_priority('a'));
        assert_eq!(26, item_priority('z'));
        assert_eq!(27, item_priority('A'));
        assert_eq!(52, item_priority('Z'));
    }

    #[test]
    fn test_find_dup() {
        assert_eq!('a', find_dup("abcdefg", "xyzarty").unwrap());
    }

    #[test]
    fn test_find_badge_in_group() {
        let (s1, s2, s3) = (
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        );

        let result = find_badge_in_group(s1, s2, s3).unwrap();
        assert_eq!('r', result);
    }
}
