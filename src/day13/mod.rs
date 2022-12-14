use anyhow::Result;
use itertools::Itertools;

use crate::utils::read_file;

mod data;

use data::*;

pub fn run(part: u8) -> Result<()> {
    let lines = read_file("./resources/day13.txt")?;

    if part == 1 {
        part1(&lines)?;
    } else {
        part2(&lines)?;
    }

    Ok(())
}

fn part1(lines: &str) -> Result<usize> {
    println!("Running day13::part1");

    let packets = parse_input(lines)?;

    let mut sum_of_idx_in_correct_order = 0;

    for (idx, (left, right)) in packets.iter().enumerate() {
        if left.cmp(right) == std::cmp::Ordering::Less {
            println!("Packet Pair {} is in the correct order.", idx);
            sum_of_idx_in_correct_order += idx + 1;
        } else {
            println!("Packet Pair {} is NOT in the correct order.", idx);
        }
    }

    println!("Result: {}", sum_of_idx_in_correct_order);

    Ok(sum_of_idx_in_correct_order)
}

fn part2(lines: &str) -> Result<usize> {
    println!("Running day13::part2");

    let divider_packet2 = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(2)])]);
    let divider_packet6 = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(6)])]);

    let mut packets = parse_input(lines)?;

    packets.push((divider_packet2.clone(), divider_packet6.clone()));

    let list = packets
        .iter()
        .flat_map(|(l, r)| vec![l, r])
        .sorted()
        .collect_vec();

    let mut decoder_key = 1;

    for (idx, pd) in list.iter().enumerate() {
        if *pd == &divider_packet2 || *pd == &divider_packet6 {
            decoder_key *= idx + 1
        }
    }

    println!("Decoder Key: {}", decoder_key);

    Ok(decoder_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1_example() {
        assert_eq!(13, part1(EXAMPLE).unwrap());
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(140, part2(EXAMPLE).unwrap());
    }

    #[test]
    fn test_part1() {
        let lines = read_file("./resources/day13.txt").unwrap();
        assert_eq!(6415, part1(&lines).unwrap());
    }

    #[test]
    fn test_part2() {
        let lines = read_file("./resources/day13.txt").unwrap();
        assert_eq!(20056, part2(&lines).unwrap());
    }
}
