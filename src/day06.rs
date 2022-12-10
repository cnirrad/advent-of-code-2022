use anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

pub fn run(part: u8) -> Result<()> {
    if part == 1 {
        part1()
    } else {
        part2()
    }
}

#[derive(Debug)]
struct CharWindow<const WINDOW_SIZE: usize> {
    string: String,
    start: usize,
    current_values: [u8; 26], // a count for each letter of the alphabet (only dealing with lowercase)
}

fn char_to_idx(ch: &char) -> usize {
    *ch as usize - 'a' as usize
}

impl<const N: usize> CharWindow<N> {
    pub fn new(s: String) -> Self {
        let mut current_values = [0u8; 26];
        for ch in s[0..N].chars() {
            current_values[char_to_idx(&ch)] += 1;
        }
        CharWindow {
            string: s,
            start: 0,
            current_values,
        }
    }

    pub fn has_dups(&self) -> bool {
        self.current_values.iter().any(|i| *i > 1u8)
    }

    pub fn end(&self) -> usize {
        self.start + N - 1
    }

    pub fn advance(&mut self) -> bool {
        if self.start + 1 + N > self.string.len() {
            return false;
        }

        let ch_to_remove = &self.string.chars().nth(self.start).unwrap();
        self.current_values[char_to_idx(ch_to_remove)] -= 1;

        self.start += 1;

        let ch_to_add = &self.string.chars().nth(self.end()).unwrap();
        self.current_values[char_to_idx(ch_to_add)] += 1;

        // println!(
        //     "Advanced. Dropped {} (idx {}), added {} (char #{}, idx in string {})",
        //     ch_to_remove,
        //     char_to_idx(ch_to_remove),
        //     ch_to_add,
        //     char_to_idx(ch_to_add),
        //     self.end()
        // );
        true
    }
}

fn part2() -> Result<()> {
    println!("Running day6::part2");

    let signal = read_file()?;

    let mut window: CharWindow<14> = CharWindow::new(signal);

    while window.has_dups() {
        // println!(
        //     "Window {} - {} (string {}) has dups. (window={:?}",
        //     window.start,
        //     window.end(),
        //     &window.string.as_str()[window.start..window.end()],
        //     window.current_values
        // );
        if !window.advance() {
            bail!("advance() returned false. Did not find start of message");
        }
    }

    println!(
        "The start of message is {} at {}, the number of characters read is {}",
        &window.string.as_str()[window.start..window.end()],
        window.start,
        window.end() + 1
    );
    Ok(())
}

fn part1() -> Result<()> {
    println!("Running day6::part1");

    let stream = read_file()?;

    let iter = stream.chars().tuple_windows::<(_, _, _, _)>();

    if let Some((idx, item)) = iter.enumerate().find(|(_idx, tuple)| !contains_dup(tuple)) {
        println!(
            "First start of packet found at: {} for input {}{}{}{}",
            idx + 4,
            item.0,
            item.1,
            item.2,
            item.3
        );
    }

    /*
    let iter = stream.chars().tuple_windows::<(_, _, _, _)>();
    for (idx, item) in iter.enumerate() {
        if contains_dup(&item) {
            println!(
                "Start of packet found at: {} for input {}{}{}{}",
                idx, item.0, item.1, item.2, item.3
            );
        }
    }
    */

    Ok(())
}

fn contains_dup((c1, c2, c3, c4): &(char, char, char, char)) -> bool {
    let mut set = HashSet::new();
    set.insert(c1);
    set.insert(c2);
    set.insert(c3);
    set.insert(c4);

    set.len() < 4
}

// Reads the file and returns the CrateStacks along with a list of move instructions
fn read_file() -> Result<String> {
    let mut file = File::open("./resources/day6.txt").context("Could not find day6.txt")?;
    let mut lines = String::new();
    file.read_to_string(&mut lines)?;

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use crate::day06::*;

    #[test]
    fn test_can_read_file_without_err() {
        read_file().unwrap();
    }

    #[test]
    fn test_contains_dup() {
        assert!(contains_dup(&('a', 'b', 'c', 'a')));
        assert!(contains_dup(&('z', 'b', 'b', 'a')));
        assert!(contains_dup(&('a', 'b', 'd', 'd')));
    }

    #[test]
    fn test_does_not_contains_dup() {
        assert!(!contains_dup(&('a', 'b', 'c', 'd')));
    }

    #[test]
    fn test_has_dups() {
        let window: CharWindow<14> = CharWindow::new("abcdefghaijklmnopqrstuvwxyz".to_string());

        assert!(window.has_dups());
    }

    #[test]
    fn test_does_not_has_dups() {
        let window: CharWindow<14> = CharWindow::new("abcdefghijklmnopqrstuvwxyz".to_string());

        assert!(!window.has_dups());
    }
}
