use anyhow::{Context, Result};
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
    println!("Running day2::part2");

    let file = File::open("./resources/day2.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let mut iter = line.split_whitespace();

        let them = iter.next();
        let desired = iter.next();

        // lose = 0
        // draw = 3
        // win = 6
        // rock = 1
        // paper = 2
        // scissors = 3
        let score = match (them, desired) {
            (Some("A"), Some("X")) => 3, // lose + rock
            (Some("B"), Some("X")) => 1, // lose + paper
            (Some("C"), Some("X")) => 2, // lose + scissors
            (Some("A"), Some("Y")) => 4, // draw + rock
            (Some("B"), Some("Y")) => 5, // draw + paper
            (Some("C"), Some("Y")) => 6, // draw + scissors
            (Some("A"), Some("Z")) => 8, // win + rock
            (Some("B"), Some("Z")) => 9, // win + paper
            (Some("C"), Some("Z")) => 7, // win + scissors
            _ => unreachable!(),
        };
        total += score;
    }

    println!("Total points = {}", total);
    Ok(())
}

fn part1() -> Result<()> {
    println!("Running day2::part1");

    let file = File::open("./resources/day2.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let mut iter = line.split_whitespace();
        let them = iter.next();
        let me = iter.next();

        let score = match (them, me) {
            (Some("A"), Some("X")) => 4, // draw + rock
            (Some("B"), Some("X")) => 1, // lose + rock
            (Some("C"), Some("X")) => 7, // win + rock
            (Some("A"), Some("Y")) => 8, // win + paper
            (Some("B"), Some("Y")) => 5, // draw + paper
            (Some("C"), Some("Y")) => 2, // lose + paper
            (Some("A"), Some("Z")) => 3, // lose + scissors
            (Some("B"), Some("Z")) => 9, // win + scissors
            (Some("C"), Some("Z")) => 6, // draw + scissors
            _ => unreachable!(),
        };

        total += score;
    }

    println!("Total points = {}", total);
    Ok(())
}
