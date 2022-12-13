use std::collections::HashSet;
use std::fmt::Display;

use crate::utils::read_file;
use anyhow::{Context, Result};
use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

pub fn run(part: u8) -> Result<()> {
    let lines = read_file("./resources/day12.txt")?;

    if part == 1 {
        part1(&lines)?;
    } else {
        part2(&lines)?;
    }

    Ok(())
}

fn part1(lines: &str) -> Result<u32> {
    println!("Running day12::part1");

    let mut hm = HeightMap::parse(lines);

    let shortest_path = hm.find_shortest_path()?;

    println!("width={}, height={}\n{}", hm.width(), hm.height(), hm);

    println!("Found shortest path: {}", shortest_path);

    Ok(shortest_path)
}

fn part2(lines: &str) -> Result<u32> {
    println!("Running day12::part2");

    let mut hm = HeightMap::parse(lines);

    let shortest_path = hm.find_hiking_trail()?;

    println!("width={}, height={}\n{}", hm.width(), hm.height(), hm);

    println!("Found shortest path: {}", shortest_path);

    Ok(shortest_path)
}

/// get a numeric elevation from a char
fn char_to_elevation(ch: char) -> i32 {
    match ch {
        'S' => 'a' as i32,
        'E' => 'z' as i32,
        _ => ch as i32,
    }
}

struct HeightMap {
    map: Vec<Vec<char>>,

    path: Option<HashSet<i32>>,
}

impl HeightMap {
    fn parse(lines: &str) -> Self {
        let map: Vec<Vec<char>> = lines
            .split('\n')
            .map(|line| line.chars().collect_vec())
            .collect();

        HeightMap { map, path: None }
    }

    fn width(&self) -> usize {
        self.map.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn get(&self, (x, y): (i32, i32)) -> Option<char> {
        if x < 0 || y < 0 {
            None
        } else {
            let row = self.map.get(y as usize);
            if let Some(r) = row {
                r.get(x as usize).map(|c| c.to_owned())
            } else {
                None
            }
        }
    }

    fn get_by_idx(&self, idx: usize) -> Option<char> {
        let coord = (idx % self.width(), idx / self.width());

        self.get((coord.0 as i32, coord.1 as i32))
    }

    fn coord_to_idx(&self, (x, y): (usize, usize)) -> usize {
        y * self.width() + x
    }

    /// Finds adjacent nodes that are accessible from the target node
    fn get_accessible_nodes(
        &self,
        (x, y): (i32, i32),
        is_accessible: fn(i32, i32) -> bool,
    ) -> Vec<(i32, i32)> {
        let mut result: Vec<(i32, i32)> = Vec::new();

        let current = self
            .get((x, y))
            .with_context(|| format!("Trying to get ({},{}) and expecting it to be there.", x, y))
            .unwrap();

        let current = char_to_elevation(current);

        for node in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if let Some(val) = self.get(node) {
                let height = char_to_elevation(val);

                if is_accessible(current, height) {
                    // Can reach this node from here
                    // println!(
                    //     "Can reach from {}({}, {}) =>{} ({}, {})",
                    //     current, x, y, height as i32, node.0, node.1
                    // );
                    result.push(node);
                }
            }
        }

        result
    }

    fn find_start_and_end_idx(&self) -> (usize, usize) {
        let mut start = 0;
        let mut end = 0;
        for col_idx in 0..self.width() as i32 {
            for row_idx in 0..self.height() as i32 {
                if let Some(val) = self.get((col_idx, row_idx)) {
                    if val == 'S' {
                        start = self.coord_to_idx((col_idx as usize, row_idx as usize));
                    } else if val == 'E' {
                        end = self.coord_to_idx((col_idx as usize, row_idx as usize));
                    }
                }
            }
        }

        (start, end)
    }

    /// Find the shortest path from 'S' to 'E'
    fn find_shortest_path(&mut self) -> Result<u32> {
        let graph = self.build_graph(|current, next| (next - current) <= 1);
        let (start, end) = self.find_start_and_end_idx();

        println!("Searching from {} to {}", start, end);
        let start = NodeIndex::new(start);
        let end = NodeIndex::new(end);

        let path = astar(&graph, start, |finish| finish == end, |_| 1, |_| 0)
            .context("Could not find a path.")
            .unwrap();

        // println!("{:?}", path);
        self.path = Some(path.1.iter().map(|n| n.index() as i32).collect());

        Ok(path.0)
    }

    /// Finds the shortest path from position 'E' to any 'a'
    fn find_hiking_trail(&mut self) -> Result<u32> {
        let graph = self.build_graph(|current, next| (current - next) <= 1);
        let (_, end) = self.find_start_and_end_idx();

        let end = NodeIndex::new(end);

        let path = astar(
            &graph,
            end,
            |finish| {
                let val = self.get_by_idx(finish.index()).unwrap();

                val == 'a'
            },
            |_| 1,
            |_| 0,
        )
        .context("Could not find a path.")
        .unwrap();

        // println!("{:?}", path);
        self.path = Some(path.1.iter().map(|n| n.index() as i32).collect());

        Ok(path.0)
    }

    fn build_graph(&mut self, f: fn(i32, i32) -> bool) -> Graph<(usize, usize), i32> {
        let mut graph = Graph::<(usize, usize), i32>::new();

        // add all nodes
        for row_idx in 0..self.height() {
            for col_idx in 0..self.width() {
                let id = graph.add_node((col_idx, row_idx));
                let idx = self.coord_to_idx((col_idx, row_idx));
                assert_eq!(NodeIndex::new(idx), id);
            }
        }

        // add edges
        for col_idx in 0..self.width() {
            for row_idx in 0..self.height() {
                let accessible = self.get_accessible_nodes((col_idx as i32, row_idx as i32), f);
                // println!(
                //     "Accessible from ({}, {}) = {:?}",
                //     col_idx, row_idx, accessible,
                // );
                let current_idx = self.coord_to_idx((col_idx, row_idx));

                accessible.iter().for_each(|coord| {
                    let to_idx = self.coord_to_idx((coord.0 as usize, coord.1 as usize));

                    let from = NodeIndex::new(current_idx);
                    let to = NodeIndex::new(to_idx);
                    //println!("Adding {} -> {}", current_idx, to_idx);

                    graph.add_edge(from, to, 1);
                });
            }
        }
        graph
    }
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = match &self.path {
            Some(p) => p.clone(),
            None => HashSet::new(),
        };

        let mut idx = 0;

        for row in &self.map {
            for col in row {
                if path.contains(&idx) {
                    write!(f, "#")?;
                } else {
                    write!(f, "{}", col)?;
                }
                idx += 1;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::*;

    const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1_example() {
        assert_eq!(31, part1(EXAMPLE).unwrap());
    }

    #[test]
    fn test_part1() {
        let lines = read_file("./resources/day12.txt").unwrap();
        assert_eq!(437, part1(&lines).unwrap());
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(29, part2(EXAMPLE).unwrap());
    }

    #[test]
    fn test_part2() {
        let lines = read_file("./resources/day12.txt").unwrap();
        assert_eq!(430, part2(&lines).unwrap());
    }
}
