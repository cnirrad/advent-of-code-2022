use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(part: u8) -> Result<()> {
    // if part == 1 {
    part1()
    // } else {
    //     part2()
    // }
}

fn part1() -> Result<()> {
    println!("Running day2::part1");

    let file = File::open("./resources/day2_part1.txt")?;
    let reader = BufReader::new(file);

    Ok(())
}
