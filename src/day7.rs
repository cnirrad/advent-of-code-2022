use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::HashMap;
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
    println!("Running day7::part2");

    let mut term_output = TerminalOutput::new();
    term_output.parse("./resources/day7.txt")?;

    let total = 70000000;
    let need = 30000000;
    let used: u64 = term_output.total;

    let target = need - (total - used);
    println!("{} bytes used, need to free {}", used, target);

    let to_delete = term_output
        .dir_sizes
        .into_values()
        .sorted()
        .find(|&x| x > target);

    println!("Deleteing a directory the size of {}", to_delete.unwrap());

    Ok(())
}

fn part1() -> Result<()> {
    println!("Running day7::part1");

    let mut term_output = TerminalOutput::new();
    term_output.parse("./resources/day7.txt")?;

    //println!("{:#?}", term_output);

    let total: u64 = term_output
        .dir_sizes
        .into_values()
        .filter(|v| *v <= 100000)
        .sum();

    println!("The total of all directories (under 100000) is {}", total);

    println!("The total of all directories (under 100000) is {}", total);

    Ok(())
}

#[derive(Debug)]
struct TerminalOutput {
    dir_sizes: HashMap<String, u64>,
    current_path: Vec<String>,

    current_size: u64,

    total: u64,
}

impl TerminalOutput {
    fn new() -> Self {
        TerminalOutput {
            dir_sizes: HashMap::new(),
            current_path: Vec::new(),
            current_size: 0,
            total: 0,
        }
    }

    // Reads the file and returns the lines
    fn parse(&mut self, path: &str) -> Result<()> {
        let file = File::open(path).context("Could not read file")?;

        let reader = BufReader::new(file);

        for (idx, line) in reader.lines().enumerate() {
            let line = line.with_context(|| format!("Could not read line {}", idx))?;

            if line.starts_with("$ cd") {
                self.change_dir(&line[5..]);
            } else if !line.starts_with("$ ls") && !line.starts_with("dir") {
                let size: u64 = line.split_ascii_whitespace().next().unwrap().parse()?;

                //println!("Adding {} to current_size {}", size, self.current_size);
                self.current_size += size;
                self.total += size;
            } else {
                //println!("Ignore line {}", line);
            }
        }

        // Change dir one more time to make sure we add in the current_size
        self.change_dir("..");
        Ok(())
    }

    fn change_dir(&mut self, dir: &str) {
        // add current directory size to all parents
        for i in 0..self.current_path.len() {
            //dir in &self.current_path {
            let path = self.current_path[0..=i].join("/");
            if let Some(s) = self.dir_sizes.get_mut(&path) {
                //println!("Setting {} = {} + {}", path, self.current_size, *s);
                *s += self.current_size;
            }
        }
        self.current_size = 0;

        if dir == ".." {
            self.current_path.pop();
        } else {
            self.current_path.push(dir.to_string());
            let path = self.current_path.join("/");
            if let Some(_) = self.dir_sizes.insert(path, 0) {
                panic!("Changing dir to {} and found a duplicate.", dir)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::*;

    #[test]
    fn test_change_dir() {
        let mut m = HashMap::new();
        m.insert("/".to_string(), 0);
        m.insert("//test".to_string(), 0);
        let mut t = TerminalOutput {
            current_path: vec!["/".to_string(), "test".to_string()],
            dir_sizes: m,

            current_size: 100,
            total: 1000,
        };

        t.change_dir("foo");

        assert_eq!(&100, t.dir_sizes.get("/").unwrap());
        assert_eq!(&100, t.dir_sizes.get("//test").unwrap());
        assert_eq!(&0, t.dir_sizes.get("//test/foo").unwrap());

        assert_eq!("//test/foo", t.current_path.join("/"));

        t.current_size = 11;
        t.change_dir("..");

        assert_eq!("//test", t.current_path.join("/"));
        assert_eq!(&111, t.dir_sizes.get("/").unwrap());
        assert_eq!(&111, t.dir_sizes.get("//test").unwrap());
        assert_eq!(&11, t.dir_sizes.get("//test/foo").unwrap());
    }
}
