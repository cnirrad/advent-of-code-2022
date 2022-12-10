use anyhow::bail;
use anyhow::{Context, Result};
use std::fmt::Display;
use std::fs::File;
use std::io::Read;

pub fn run(part: u8) -> Result<()> {
    if part == 1 {
        part1()
    } else {
        part2()
    }
}

fn part2() -> Result<()> {
    println!("Running day8::part2");

    let lines = read_file()?;
    let score = find_most_scenic(&lines)?;

    println!("The score of the most scenic tree is: {}", score);

    Ok(())
}

fn part1() -> Result<()> {
    println!("Running day8::part1");

    let lines = read_file()?;

    let count = count_visible(&lines)?;

    println!(
        "There are a total of {} trees visible from the outside.",
        count
    );

    Ok(())
}

struct ForestMap {
    forest: Vec<String>,

    width: usize,
}

impl ForestMap {
    fn new(lines: &str) -> Self {
        let lines: Vec<String> = lines.split("\n").map(|s| s.to_owned()).collect();
        let width = lines.get(0).unwrap().len();

        ForestMap {
            forest: lines,
            width: width,
        }
    }

    fn total_trees(&self) -> usize {
        self.width * self.forest.len()
    }

    fn get(&self, (row, col): (usize, usize)) -> u32 {
        self.forest
            .get(row)
            .with_context(|| format!("Error getting ({},{})", row, col))
            .unwrap()
            .chars()
            .nth(col)
            .with_context(|| format!("Error getting col ({},{})", row, col))
            .unwrap()
            .to_digit(10)
            .unwrap()
    }

    fn scenic_score(&self, idx: usize) -> usize {
        let (row, col) = self.idx_to_coord(idx);

        let current_height = self.get((row, col));

        let mut num_left = 0;
        if col > 0 {
            let mut c = col - 1;
            loop {
                let current = self.get((row, c));
                num_left += 1;
                if current >= current_height {
                    break;
                }
                if c > 0 {
                    c -= 1;
                } else {
                    break;
                }
            }
        }

        let mut num_right = 0;
        if col < self.width - 1 {
            let mut c = col + 1;
            loop {
                let current = self.get((row, c));
                num_right += 1;
                if current >= current_height {
                    break;
                }
                if c < self.width - 1 {
                    c += 1;
                } else {
                    break;
                }
            }
        }

        let mut num_up = 0;
        if row > 0 {
            let mut r = row - 1;
            loop {
                let current = self.get((r, col));
                num_up += 1;
                if current >= current_height {
                    break;
                }
                if r > 0 {
                    r -= 1;
                } else {
                    break;
                }
            }
        }

        let mut num_down = 0;
        if row < self.forest.len() - 1 {
            let mut r = row + 1;
            loop {
                let current = self.get((r, col));
                num_down += 1;
                if current >= current_height {
                    break;
                }
                if r < self.forest.len() - 1 {
                    r += 1;
                } else {
                    break;
                }
            }
        }

        println!(
            "Score for {} ({},{}) is ({}*{}*{}*{})",
            current_height, row, col, num_left, num_right, num_up, num_down
        );

        num_left * num_right * num_up * num_down
    }

    fn idx_to_coord(&self, idx: usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }
}

struct Visibility {
    m: Vec<bool>,

    width: usize,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, b) in self.m.iter().enumerate() {
            if i % self.width == 0 {
                writeln!(f, "")?;
            }

            write!(f, "{}", if *b { 1 } else { 0 })?;
        }

        Ok(())
    }
}

impl Visibility {
    fn new(num_rows: usize, num_cols: usize) -> Self {
        let mut v = Vec::new();
        let size = num_rows * num_cols;
        for i in 0..size {
            if i < num_cols
                || i >= size - num_cols
                || i % num_cols == 0
                || i % num_cols == num_cols - 1
            {
                // outside edge
                v.push(true);
            } else {
                v.push(false);
            }
        }
        Visibility {
            m: v,
            width: num_cols,
        }
    }

    fn set(&mut self, coords: (usize, usize), val: bool) -> Result<()> {
        let idx = self.translate_pair(coords);

        if idx > self.m.len() {
            bail!("Invalid coords: ({}, {})", coords.0, coords.1);
        }

        self.m[idx] = val;
        Ok(())
    }

    fn num_visible(&self) -> usize {
        let result = &self.m.iter().filter(|x| **x).count();

        result.to_owned()
    }

    fn translate_pair(&self, (row, col): (usize, usize)) -> usize {
        (self.width * row) + col
    }
}

fn read_file() -> Result<String> {
    let mut file = File::open("./resources/day8.txt").context("Could not find day6.txt")?;
    let mut lines = String::new();
    file.read_to_string(&mut lines)?;

    Ok(lines)
}

fn count_visible(lines: &str) -> Result<usize> {
    let lines: Vec<&str> = lines.split("\n").collect();
    let width = lines.get(0).unwrap().len();
    let height = lines.len();
    //let outside_edges = (width * 2) + (height * 2) - 4;

    let mut visibility: Visibility = Visibility::new(height, width);

    // by row
    for (row_idx, row) in lines[1..lines.len()].iter().enumerate() {
        let mut tallest = 0;

        // left to right
        for (col_idx, col) in row.chars().enumerate() {
            let size: u32 = col.to_digit(10).unwrap();
            if size > tallest {
                visibility.set((row_idx + 1, col_idx), true)?;
                tallest = size;
            }
            if size == 9 {
                // can't see past
                break;
            }
        }

        tallest = 0;
        // Right to Left
        for (col_idx, col) in row.chars().rev().enumerate() {
            let size: u32 = col.to_digit(10).unwrap();
            if size > tallest {
                visibility.set((row_idx + 1, width - col_idx - 1), true)?;
                tallest = size;
            }
            if size == 9 {
                // can't see past
                break;
            }
        }
    }

    // By Column
    for col_idx in 1..width {
        let mut tallest = 0;

        // Top Down
        for (row_idx, row) in lines.iter().enumerate() {
            let size = row.chars().nth(col_idx).unwrap().to_digit(10).unwrap();

            if size > tallest {
                visibility.set((row_idx, col_idx), true)?;
                tallest = size;
            }
            if size == 9 {
                // can't see past
                break;
            }
        }

        tallest = 0;

        // Bottom up
        for (row_idx, row) in lines[0..lines.len()].iter().rev().enumerate() {
            let size = row.chars().nth(col_idx).unwrap().to_digit(10).unwrap();

            if size > tallest {
                visibility.set((height - row_idx - 1, col_idx), true)?;
                tallest = size;
            }
            if size == 9 {
                // can't see past
                break;
            }
        }
    }

    println!("{}", visibility);
    Ok(visibility.num_visible())
}

fn find_most_scenic(lines: &str) -> Result<usize> {
    let forest = ForestMap::new(&lines);

    let best_score = (0..forest.total_trees())
        .map(|i| forest.scenic_score(i))
        .max();

    Ok(best_score.unwrap())
}

#[cfg(test)]
mod tests {
    use crate::day08::*;

    const EXAMPLE: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn test_with_example_input() {
        assert_eq!(21, count_visible(EXAMPLE).unwrap());
    }

    #[test]
    fn test_part1() {
        let lines = read_file().unwrap();
        let count = count_visible(&lines).unwrap();

        assert_eq!(1713, count);
    }

    #[test]
    fn test_find_most_scenic() {
        assert_eq!(8, find_most_scenic(EXAMPLE).unwrap());
    }

    #[test]
    fn test_scenic_score() {
        let forest = ForestMap::new(EXAMPLE);

        assert_eq!(4, forest.scenic_score(7));
        assert_eq!(8, forest.scenic_score(17));
    }

    #[test]
    fn test_idx_to_coord() {
        let forest = ForestMap::new(EXAMPLE);

        assert_eq!((1, 1), forest.idx_to_coord(6));
        assert_eq!((1, 2), forest.idx_to_coord(7));
        assert_eq!((2, 2), forest.idx_to_coord(12));
    }
}
