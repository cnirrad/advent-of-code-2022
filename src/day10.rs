use std::fmt::Display;

use anyhow::Result;

use crate::utils::read_file;

pub fn run(part: u8) -> Result<()> {
    if part == 1 {
        part1()?;
    } else {
        part2()?;
    }

    Ok(())
}

fn part1() -> Result<i32> {
    println!("Running day10::part1");

    let lines = read_file("./resources/day10.txt")?;
    let strength = determine_signal_strength(&lines);

    println!("Signal strength is {}", strength);

    Ok(strength)
}

fn part2() -> Result<String> {
    println!("Running day10::part2");

    let lines = read_file("./resources/day10.txt")?;

    let mut screen = Screen::new();
    screen.process(&lines)?;

    let display = format!("{}", screen);

    println!("{}", display);

    Ok(display)
}

fn determine_signal_strength(instructions: &str) -> i32 {
    let v: Vec<&str> = instructions.split('\n').collect();
    let mut signal_strength = 0;
    let mut cycle = 1;
    let mut current_v = 1;

    for inst in v {
        cycle += 1;

        if is_interesting_cycle(cycle) {
            signal_strength += current_v * cycle;

            println!(
                "Cycle {}, increased strength to {}. (current_v={})",
                cycle, signal_strength, current_v
            );
        }

        if !inst.starts_with("noop") {
            let (_, param) = inst.split_once(' ').unwrap();
            let count: i32 = param.parse().unwrap();

            cycle += 1;
            current_v += count;

            if is_interesting_cycle(cycle) {
                signal_strength += current_v * cycle;

                println!(
                    "Cycle {}, increased strength to {}. (current_v={})",
                    cycle, signal_strength, current_v
                );
            }
        }
    }

    signal_strength
}

fn is_interesting_cycle(cycle: i32) -> bool {
    (cycle - 20) % 40 == 0
}

struct Screen {
    pixels: Vec<Vec<char>>,

    sprite_pos: i32,
}

impl Screen {
    fn new() -> Self {
        let mut pixels: Vec<Vec<char>> = Vec::new();

        for _ in 0..6 {
            let row = vec!['.'; 40];

            pixels.push(row);
        }

        Screen {
            pixels,
            sprite_pos: 1,
        }
    }

    fn process(&mut self, instructions: &str) -> Result<()> {
        let instructions: Vec<&str> = instructions.split('\n').collect();

        let mut cycle = 0;

        for inst in instructions {
            cycle += 1;

            self.draw_cycle(cycle);

            if !inst.starts_with("noop") {
                cycle += 1;

                self.draw_cycle(cycle);
                self.move_sprite(inst);
            }
        }

        Ok(())
    }

    fn move_sprite(&mut self, instruction: &str) {
        let (_, param) = instruction.split_once(' ').unwrap();
        let count: i32 = param.parse().unwrap();

        self.sprite_pos += count;
        println!("Moving sprite {} spots to {}", count, self.sprite_pos);
    }

    fn draw_cycle(&mut self, cycle: usize) {
        let cycle0 = cycle - 1;
        let col_idx = cycle0 % 40;

        if (col_idx as i32 - self.sprite_pos).abs() <= 1 {
            // set pixel for this cycle to #
            let row_idx = cycle0 / 40;

            println!(
                "Drawing pixel ({},{}) in cycle {} because sprite pos = {}",
                row_idx, col_idx, cycle, self.sprite_pos
            );
            self.pixels[row_idx][col_idx] = '#';
        } else {
            println!(
                "Not drawing pixel for cycle {}, sprite_pos = {}",
                cycle, self.sprite_pos
            );
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.pixels {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::*;

    const EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop   ";

    #[test]
    fn test_part1() {
        assert_eq!(14060, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        let expected = "###...##..###..#..#.####.#..#.####...##.
#..#.#..#.#..#.#.#..#....#.#..#.......#.
#..#.#..#.#..#.##...###..##...###.....#.
###..####.###..#.#..#....#.#..#.......#.
#....#..#.#....#.#..#....#.#..#....#..#.
#....#..#.#....#..#.#....#..#.####..##..";
        assert_eq!(expected, part2().unwrap().trim());
    }

    #[test]
    fn test_read_file() {
        read_file("./resources/day10.txt").unwrap();
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(13140, determine_signal_strength(EXAMPLE));
    }

    #[test]
    fn test_example_part2() {
        let mut screen = Screen::new();
        screen.process(EXAMPLE).unwrap();

        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

        let display = format!("{}", screen);
        println!("{}", display);

        assert_eq!(expected, display.trim());
    }

    #[test]
    fn test_is_interesting_cycle() {
        assert!(is_interesting_cycle(20));
        assert!(is_interesting_cycle(60));
        assert!(is_interesting_cycle(100));
        assert!(is_interesting_cycle(140));
        assert!(is_interesting_cycle(180));
        assert!(is_interesting_cycle(220));

        assert!(!is_interesting_cycle(10));
        assert!(!is_interesting_cycle(40));
        assert!(!is_interesting_cycle(120));
    }
}
