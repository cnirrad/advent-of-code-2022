use anyhow::Result;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PacketData {
    Integer(u32),
    List(Vec<PacketData>),
}

pub fn parse_input(lines: &str) -> Result<Vec<(PacketData, PacketData)>> {
    parser::parse_pairs(lines)
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PacketData::Integer(l), PacketData::Integer(r)) => l.cmp(r),
            (PacketData::List(l), PacketData::List(r)) => {
                let shortest = l.len().min(r.len());
                for idx in 0..shortest {
                    match l[idx].cmp(&r[idx]) {
                        std::cmp::Ordering::Equal => continue,
                        o => return o,
                    };
                }
                // Shorter of the lists is the Lesser
                l.len().cmp(&r.len())
            } // TODO: This will need custom code
            (l @ PacketData::List(_), PacketData::Integer(i)) => {
                let right_list = PacketData::List(vec![PacketData::Integer(*i)]);
                l.cmp(&right_list)
            }
            (PacketData::Integer(i), r @ PacketData::List(_)) => {
                let left_list = PacketData::List(vec![PacketData::Integer(*i)]);
                left_list.cmp(r)
            }
        }
    }
}

mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{newline, u32},
        combinator::map,
        multi::{separated_list0, separated_list1},
        sequence::{delimited, terminated},
        Finish, IResult,
    };

    fn integer(s: &str) -> IResult<&str, PacketData> {
        map(u32, PacketData::Integer)(s)
    }

    fn list(s: &str) -> IResult<&str, PacketData> {
        let list_items = separated_list0(tag(","), packet_data);
        let to_vec = map(delimited(tag("["), list_items, tag("]")), Vec::from);

        map(to_vec, PacketData::List)(s)
    }

    /// Will parse either a PacketData Integer or List
    fn packet_data(s: &str) -> IResult<&str, PacketData> {
        alt((integer, list))(s)
    }

    fn packet_data_pair(s: &str) -> IResult<&str, (PacketData, PacketData)> {
        let (s, left) = terminated(packet_data, newline)(s)?;

        // Make sure not to capture the line break here, otherwise our parse will not pick up the double line break
        let (s, right) = packet_data(s)?;

        Ok((s, (left, right)))
    }

    /// Splits the input file into chunks based on empty lines and parses
    /// each chunk into a PacketData pair.
    pub fn parse_pairs(s: &str) -> Result<Vec<(PacketData, PacketData)>> {
        let result = separated_list1(tag("\n\n"), packet_data_pair)(s);
        let (_, data) = result
            .finish()
            .map_err(|e| anyhow!("Failed to parse packet data with error {e}"))?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::{parser, *};

    const EXAMPLE: &str = "[0,1,3,1,1]
[0,1,5,1,1]

[[0],[2,3,4]]
[[0],4]

[[[]]]
[[]]

[0,[2,[3,[4,[5,6,7]]]],8,9]
[0,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_parse_pairs() {
        let result = parser::parse_pairs(EXAMPLE).unwrap();

        println!("Result: {:?}", result);

        assert_eq!(4, result.len());
    }

    #[test]
    fn test_packet_data_compare_integer() {
        let left = PacketData::Integer(5);
        let right = PacketData::Integer(8);

        assert_eq!(std::cmp::Ordering::Less, left.cmp(&right));
        assert_eq!(std::cmp::Ordering::Greater, right.cmp(&left));
        assert_eq!(std::cmp::Ordering::Equal, left.cmp(&left));
    }

    #[test]
    fn test_packet_data_compare_list() {
        let left = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(1)])]);
        let right = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(2)])]);

        assert_eq!(std::cmp::Ordering::Less, left.cmp(&right));
        assert_eq!(std::cmp::Ordering::Greater, right.cmp(&left));
        assert_eq!(std::cmp::Ordering::Equal, left.cmp(&left));
    }
}
