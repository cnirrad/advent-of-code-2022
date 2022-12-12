use anyhow::Result;

use crate::utils::read_file;

pub fn run(part: u8) -> Result<()> {
    let lines = read_file("./resources/dayX.txt")?;

    if part >= 1 {
        part1(&lines)?;
    } else {
        part2(&lines)?;
    }

    Ok(())
}

fn part1(lines: &str) -> Result<usize> {
    println!("Running dayX::part1");

    println!("Report");

    Ok(1)
}

fn part2(lines: &str) -> Result<usize> {
    println!("Running dayX::part2");

    println!("Report");

    Ok(1)
}

#[cfg(test)]
mod tests {
    use crate::dayX::*;

    const EXAMPLE: &str = "FILL ME";

    #[test]
    fn test_part1_example() {
        assert_eq!(0, part1(EXAMPLE).unwrap());
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(-1, part2().unwrap());
    // }
}
