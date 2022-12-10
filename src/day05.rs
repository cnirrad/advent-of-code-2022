use anyhow::{Context, Result};
use regex::Regex;
use std::fs::File;
use std::io::Read;

pub fn run(part: u8) -> Result<()> {
    if part == 1 {
        part1()
    } else {
        part2()
    }
}

struct CrateStacks {
    stacks: Vec<Vec<char>>,
}

impl CrateStacks {
    fn new(num_stacks: usize) -> Self {
        let mut stacks = Vec::new();
        for _ in 0..num_stacks {
            stacks.push(Vec::new());
        }

        CrateStacks { stacks }
    }

    fn reverse_all(&mut self) {
        for s in self.stacks.iter_mut() {
            s.reverse();
        }
    }

    fn push(&mut self, idx: usize, val: char) {
        if idx > self.stacks.len() {
            unimplemented!("Currently hard coding the number of stacks and this push goes beyond the pre-allocated size");
        }

        self.stacks.get_mut(idx).unwrap().push(val);
    }

    fn get_top_as_string(self) -> String {
        let mut s = String::new();
        for stack in self.stacks {
            let ch = if stack.is_empty() {
                ' '
            } else {
                stack[stack.len() - 1]
            };
            s.push(ch);
        }
        s
    }

    fn move_crates_with_crate_mover_9000(
        &mut self,
        count: usize,
        from_stack: usize,
        to_stack: usize,
    ) {
        for _ in 0..count {
            let item = self.stacks[from_stack - 1].pop().unwrap();
            self.stacks[to_stack - 1].push(item);
        }
    }

    fn move_crates_with_crate_mover_9001(
        &mut self,
        count: usize,
        from_stack: usize,
        to_stack: usize,
    ) {
        let mut tmp = Vec::new();
        for _ in 0..count {
            let item = self.stacks[from_stack - 1].pop().unwrap();
            tmp.push(item);
        }
        tmp.reverse();
        for item in tmp {
            self.stacks[to_stack - 1].push(item);
        }
    }
}

struct MoveInstruction {
    count: usize,
    from_stack: usize,
    to_stack: usize,
}

fn part2() -> Result<()> {
    println!("Running day5::part2");
    let (mut stacks, mut instructions) = read_file()?;

    for instruction in instructions.iter_mut() {
        stacks.move_crates_with_crate_mover_9001(
            instruction.count,
            instruction.from_stack,
            instruction.to_stack,
        );
    }

    println!("The top of the stack: {}", stacks.get_top_as_string());

    Ok(())
}

fn part1() -> Result<()> {
    println!("Running day5::part1");

    let (mut stacks, mut instructions) = read_file()?;

    for instruction in instructions.iter_mut() {
        stacks.move_crates_with_crate_mover_9000(
            instruction.count,
            instruction.from_stack,
            instruction.to_stack,
        );
    }

    println!("The top of the stack: {}", stacks.get_top_as_string());

    Ok(())
}

// Reads the file and returns the CrateStacks along with a list of move instructions
fn read_file() -> Result<(CrateStacks, Vec<MoveInstruction>)> {
    let mut file = File::open("./resources/day5.txt").context("Could not find day5.txt")?;
    let mut lines = String::new();
    file.read_to_string(&mut lines)?;

    let (stack_strs, instruction_strs) = lines.split_once("\n\n").unwrap();

    let stacks = parse_stacks(stack_strs)?;
    let inst = parse_instructions(instruction_strs)?;

    Ok((stacks, inst))
}

fn parse_stacks(stack_strs: &str) -> Result<CrateStacks> {
    // Hard coding the number of stacks as a shortcut
    let mut stacks = CrateStacks::new(9);

    for line in stack_strs.split('\n') {
        let mut pos = 1; // position in line
        let mut idx = 0; // zero based index of stack
        while pos < line.len() {
            let val = line
                .chars()
                .nth(pos)
                .with_context(|| format!("Could not get {}th character from {}", pos, line))
                .unwrap();
            if val != ' ' {
                stacks.push(idx, val);
            }
            idx += 1;
            pos += 4;
        }
    }

    stacks.reverse_all();

    Ok(stacks)
}

fn parse_instructions(stack_strs: &str) -> Result<Vec<MoveInstruction>> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let instructions = stack_strs
        .split('\n')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let nums = re
                .captures(s)
                .with_context(|| format!("Regular expression failed on {}", s))
                .unwrap();
            MoveInstruction {
                count: nums.get(1).unwrap().as_str().parse().unwrap(),
                from_stack: nums.get(2).unwrap().as_str().parse().unwrap(),
                to_stack: nums.get(3).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect();

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use crate::day05::*;

    #[test]
    fn test_can_read_file_without_err() {
        read_file().unwrap();
    }

    #[test]
    fn test_parse_instructions() {
        let input = "move 7 from 3 to 9\nmove 5 from 1 to 2";
        let result = parse_instructions(input).unwrap();

        assert_eq!(7, result[0].count);
        assert_eq!(3, result[0].from_stack);
        assert_eq!(9, result[0].to_stack);

        assert_eq!(5, result[1].count);
        assert_eq!(1, result[1].from_stack);
        assert_eq!(2, result[1].to_stack);
    }

    #[test]
    fn test_parse_stacks() {
        let input = "[R]     [L] [Q] [B] [B]     [D] [F]\n\
                           [H] [B] [G] [D] [Q] [Z]     [T] [J]";
        let result = parse_stacks(input).unwrap();

        assert_eq!("RBLQBB DF", result.get_top_as_string());
    }
}
