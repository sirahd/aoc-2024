use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let map: Vec<Vec<u32>> = reader
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap_or(10))
                    .collect()
            })
            .collect();
        let len_x = map[0].len();
        let len_y = map.len();
        let mut nodes: VecDeque<(usize, usize, u32)> = VecDeque::new();
        for j in 0..map.len() {
            for i in 0..map[j].len() {
                if map[j][i] == 0 {
                    nodes.push_back((i, j, 0));
                }
            }
        }
        let mut count = 0;
        for node in nodes {
            let mut visited = VecDeque::new();
            visited.push_back(node);
            let mut peak: HashSet<(usize, usize)> = HashSet::new();
            while !visited.is_empty() {
                let (i, j, n) = visited.pop_front().unwrap();
                if n == 9 && !peak.contains(&(i, j)) {
                    count += 1;
                    peak.insert((i, j));
                    continue;
                }
                // (i-1, j), (i+1, j), (i, j-1), (i, j+1)
                for (ni, nj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    if i.checked_add_signed(ni).is_some_and(|n| n < len_x)
                        && j.checked_add_signed(nj).is_some_and(|n| n < len_y)
                        && map[j.saturating_add_signed(nj)][i.saturating_add_signed(ni)] == n + 1
                    {
                        visited.push_back((
                            i.saturating_add_signed(ni),
                            j.saturating_add_signed(nj),
                            n + 1,
                        ));
                    }
                }
            }
        }
        Ok(count)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let map: Vec<Vec<u32>> = reader
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap_or(10))
                    .collect()
            })
            .collect();
        let len_x = map[0].len();
        let len_y = map.len();
        let mut nodes: VecDeque<(usize, usize, u32)> = VecDeque::new();
        for j in 0..map.len() {
            for i in 0..map[j].len() {
                if map[j][i] == 0 {
                    nodes.push_back((i, j, 0));
                }
            }
        }
        let mut count = 0;
        while !nodes.is_empty() {
            let (i, j, n) = nodes.pop_front().unwrap();
            if n == 9 {
                count += 1;
                continue;
            }
            // (i-1, j), (i+1, j), (i, j-1), (i, j+1)
            for (ni, nj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if i.checked_add_signed(ni).is_some_and(|n| n < len_x)
                    && j.checked_add_signed(nj).is_some_and(|n| n < len_y)
                    && map[j.saturating_add_signed(nj)][i.saturating_add_signed(ni)] == n + 1
                {
                    nodes.push_back((
                        i.saturating_add_signed(ni),
                        j.saturating_add_signed(nj),
                        n + 1,
                    ));
                }
            }
        }
        Ok(count)
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
