use anyhow::{bail, Context, Result};
use itertools::Itertools;

use crate::utils::read_file;

pub fn run(part: u8) -> Result<()> {
    let lines = read_file("./resources/day11.txt")?;

    if part == 1 {
        part1(&lines)?;
    } else {
        part2(&lines)?;
    }

    Ok(())
}

fn part1(lines: &str) -> Result<u64> {
    println!("Running day11::part1");

    let mut game = MonkeyGame::parse_input(lines)?;

    for _ in 0..20 {
        game.play_round()?;
    }

    let monkey_business = game.calc_monkey_business();
    println!("Monkey business = {}", monkey_business);

    Ok(monkey_business)
}

fn part2(lines: &str) -> Result<u64> {
    println!("Running day11::part2");

    let mut game = MonkeyGame::parse_input(lines)?;

    for _ in 0..10000 {
        game.play_round2()?;
    }

    let monkey_business = game.calc_monkey_business();
    println!("Monkey business = {}", monkey_business);

    Ok(monkey_business)
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add(String, String),
    Mul(String, String),
}

impl Operation {
    fn execute(&self, old: i64) -> Result<i64> {
        match self {
            Operation::Add(lhs, rhs) => {
                let lhs: i64 = if lhs == "old" { old } else { lhs.parse()? };
                let rhs: i64 = if rhs == "old" { old } else { rhs.parse()? };

                Ok(lhs + rhs)
            }
            Operation::Mul(lhs, rhs) => {
                let lhs: i64 = if lhs == "old" { old } else { lhs.parse()? };
                let rhs: i64 = if rhs == "old" { old } else { rhs.parse()? };

                Ok(lhs * rhs)
            }
        }
    }
}

struct Monkey {
    id: usize,
    items: Vec<i64>,

    operation: Operation,

    // the amount to test if the item is divisible by
    test: i64,

    on_true: usize,
    on_false: usize,

    inspected: usize,
}

impl Monkey {
    fn read<I>(it: &mut I) -> Result<Monkey>
    where
        I: Iterator,
        I::Item: ToString,
    {
        // Skip any blank lines
        let next_line = it.find(|line| !line.to_string().is_empty());

        if next_line.is_none() {
            bail!("Found end of input");
        }

        let line = next_line.unwrap().to_string();

        if !line.starts_with("Monkey ") {
            bail!(
                "Trying to parse Monkey, but found unexpected input: {}",
                line
            );
        }
        let (_, second) = line.split_once(' ').unwrap();
        let id: usize = second[0..second.len() - 1]
            .parse()
            .with_context(|| format!("Could not parse monkey ID from {}", line))?;

        let line = it.next().unwrap().to_string();

        let idx = line
            .find(':')
            .with_context(|| format!("Expected ':' while processing starting items in {}", line))
            .unwrap();
        let items: Vec<i64> = line[idx + 1..]
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let line = it.next().unwrap().to_string();

        let idx = line.find('=').unwrap();
        let line: &str = &line[idx + 1..];
        let elmts: Vec<&str> = line.split_ascii_whitespace().collect();
        let operation = match *elmts.get(1).unwrap() {
            "+" => Operation::Add(elmts[0].to_string(), elmts[2].to_string()),
            "*" => Operation::Mul(elmts[0].to_string(), elmts[2].to_string()),
            _ => panic!("Unknown operator in line {}", line),
        };

        let line = it.next().unwrap().to_string();

        let test = line.split_ascii_whitespace().last().unwrap().parse()?;

        let line = it.next().unwrap().to_string();

        let on_true = line.split_ascii_whitespace().last().unwrap().parse()?;

        let line = it.next().unwrap().to_string();

        let on_false = line.split_ascii_whitespace().last().unwrap().parse()?;

        Ok(Monkey {
            id,
            items,
            operation,
            test,
            on_true,
            on_false,
            inspected: 0,
        })
    }

    /// Returns a vector of items thrown - first element is which monkey it is to and the second is the item
    fn take_turn(&mut self) -> Result<Vec<(usize, i64)>> {
        let mut items_thrown = Vec::new();

        println!("Monkey {}", self.id);
        for item in &self.items {
            println!("  Monkey inspects an item with worry level {}", item);

            let new_level = self.operation.execute(*item)?;
            println!("    Worry level is {:?} to {}", self.operation, new_level);

            let new_level = new_level / 3;
            println!(
                "    Monkey gets bored with item. Worry level is divided by 3 to {}",
                new_level
            );

            if new_level % self.test == 0 {
                println!("    Current level is divisible by {}", self.test);
                println!(
                    "    Item with worry level {} is thrown to monkey {}",
                    new_level, self.on_true
                );
                items_thrown.push((self.on_true, new_level));
            } else {
                println!("    Current level is not divisible by {}", self.test);
                println!(
                    "    Item with worry level {} is thrown to monkey {}",
                    new_level, self.on_false
                );
                items_thrown.push((self.on_false, new_level));
            }
            self.inspected += 1;
        }
        self.items.clear();

        Ok(items_thrown)
    }

    /// Like take turn, but this is part 2
    fn take_turn_part2(&mut self, limit: i64) -> Result<Vec<(usize, i64)>> {
        let mut items_thrown = Vec::new();

        for item in &self.items {
            let mut new_level = self.operation.execute(*item)?;

            new_level %= limit;

            if new_level % self.test == 0 {
                items_thrown.push((self.on_true, new_level));
            } else {
                items_thrown.push((self.on_false, new_level));
            }
            self.inspected += 1;
        }
        self.items.clear();

        Ok(items_thrown)
    }

    fn catch(&mut self, item: i64) {
        self.items.push(item);
    }
}
struct MonkeyGame {
    monkeys: Vec<Monkey>,

    // max limit, used for part 2
    limit: i64,
}

impl MonkeyGame {
    fn parse_input(input: &str) -> Result<MonkeyGame> {
        let mut it = input.split('\n');

        let mut results = Vec::new();
        loop {
            let read_result = Monkey::read(&mut it);

            if let Ok(m) = read_result {
                results.push(m);
            } else {
                let limit = results.iter().map(|m| m.test).product();
                return Ok(MonkeyGame {
                    monkeys: results,
                    limit,
                });
            }
        }
    }

    fn play_round(&mut self) -> Result<()> {
        for id in 0..self.monkeys.len() {
            let items_thrown = self.monkeys[id].take_turn()?;

            for (monkey, item) in items_thrown {
                self.monkeys[monkey].catch(item);
            }
        }

        Ok(())
    }

    fn play_round2(&mut self) -> Result<()> {
        for id in 0..self.monkeys.len() {
            let items_thrown = self.monkeys[id].take_turn_part2(self.limit)?;

            for (monkey, item) in items_thrown {
                self.monkeys[monkey].catch(item);
            }
        }

        Ok(())
    }

    fn calc_monkey_business(&self) -> u64 {
        self.monkeys
            .iter()
            .map(|x| x.inspected as u64)
            .sorted()
            .rev()
            .take(2)
            .product()
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::*;

    const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1_example() {
        assert_eq!(10605, part1(EXAMPLE).unwrap());
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(2713310158, part2(EXAMPLE).unwrap());
    }
    #[test]
    fn test_part1() {
        let lines = read_file("./resources/day11.txt").unwrap();
        assert_eq!(113220, part1(&lines).unwrap());
    }

    #[test]
    fn test_part2() {
        let lines = read_file("./resources/day11.txt").unwrap();
        assert_eq!(30599555965, part2(&lines).unwrap());
    }

    #[test]
    fn test_read_monkey() {
        let monkey = Monkey::read(&mut EXAMPLE.split('\n')).unwrap();

        assert_eq!(0, monkey.id);
        let mut iter = monkey.items.iter();
        assert_eq!(Some(&79), iter.next());
        assert_eq!(Some(&98), iter.next());
        assert_eq!(
            Operation::Mul("old".to_string(), "19".to_string()),
            monkey.operation
        );
        assert_eq!(23, monkey.test);
        assert_eq!(2, monkey.on_true);
        assert_eq!(3, monkey.on_false);
    }

    #[test]
    fn test_read_input() {
        let results = MonkeyGame::parse_input(EXAMPLE).unwrap();

        assert_eq!(4, results.monkeys.len());
    }

    #[test]
    fn test_operation() {
        assert_eq!(
            14,
            Operation::Add("5".to_string(), "9".to_string())
                .execute(4)
                .unwrap()
        );

        assert_eq!(
            8,
            Operation::Add("old".to_string(), "old".to_string())
                .execute(4)
                .unwrap()
        );

        assert_eq!(
            45,
            Operation::Mul("5".to_string(), "9".to_string())
                .execute(4)
                .unwrap()
        );

        assert_eq!(
            16,
            Operation::Mul("old".to_string(), "old".to_string())
                .execute(4)
                .unwrap()
        );
    }
}
