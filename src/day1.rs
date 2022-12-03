use anyhow::{Context, Result};
use std::collections::BinaryHeap;
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
    println!("Running Day1::part2");

    let file = File::open("./resources/day1_part1.txt")?;
    let reader = BufReader::new(file);

    let mut heap = BinaryHeap::new();
    let mut accum: u32 = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line.with_context(|| format!("Could not read line {}", index))?;
        if line.is_empty() {
            heap.push(accum);
            accum = 0;
        } else {
            let c: u32 = line.parse()?;
            accum += c;
        }
    }

    let total: u32 = heap.into_sorted_vec().iter().take(3).sum();

    println!("The top 3 Elves are carrying {} calories.", total);

    Ok(())
}

fn part1() -> Result<()> {
    println!("Running Day1::part1");

    let file = File::open("./resources/day1_part1.txt")?;
    let reader = BufReader::new(file);

    let mut max: u32 = 0;
    let mut accum: u32 = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line.with_context(|| format!("Could not read line {}", index))?;
        if line.is_empty() {
            if accum > max {
                max = accum;
            }
            accum = 0;
        } else {
            let c: u32 = line.parse()?;
            accum += c;
        }
    }

    println!(
        "The Elf carrying the most calories is carrying {} calories.",
        max
    );

    Ok(())
}
