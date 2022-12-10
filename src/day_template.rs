use anyhow::{Context, Result};

use crate::utils::read_file;

pub fn run(part: u8) -> Result<()> {
    if part >= 1 {
        part1()?;
    } else {
        part2()?;
    }

    Ok(())
}

fn part1() -> Result<usize> {
    println!("Running dayX::part1");

    let lines = read_file("../resources/dayX.txt")?;

    println!("Report");

    Ok(-1)
}

fn part2() -> Result<usize> {
    println!("Running dayX::part2");

    let lines = read_file("../resources/dayX.txt")?;

    println!("Report");

    Ok(-1)
}

#[cfg(test)]
mod tests {
    use crate::dayX::*;

    const EXAMPLE: &str = "FILL ME";

    // #[test]
    // fn test_part1() {
    //     assert_eq!(-1, part1());
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(-1, part2());
    // }

    #[test]
    fn test_read_file() {
        read_file("../resources/dayX.txt").unwrap();
    }
}
