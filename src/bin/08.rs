use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        let mut map: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
        let len_x: i32 = lines[0].len() as i32;
        let len_y: i32 = lines.len() as i32;
        for (j, line) in lines.iter().enumerate() {
            for (i, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                map.entry(c)
                    .and_modify(|s| {
                        s.insert((i as i32, j as i32));
                    })
                    .or_insert(HashSet::from([(i as i32, j as i32)]));
            }
        }
        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
        for (_, set) in &map {
            for (i, c1) in set.iter().enumerate() {
                for c2 in set.iter().skip(i + 1) {
                    let dx = c1.0 - c2.0;
                    let dy = c1.1 - c2.1;
                    for c in [c1, c2] {
                        for f in [i32::add, i32::sub] {
                            let nc = (f(c.0, dx), f(c.1, dy));
                            if !set.contains(&nc)
                                && nc.0 >= 0
                                && nc.0 < len_x
                                && nc.1 >= 0
                                && nc.1 < len_y
                            {
                                antinodes.insert(nc);
                            }
                        }
                    }
                }
            }
        }
        Ok(antinodes.len() as i32)
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        let mut map: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
        let len_x: i32 = lines[0].len() as i32;
        let len_y: i32 = lines.len() as i32;
        for (j, line) in lines.iter().enumerate() {
            for (i, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                map.entry(c)
                    .and_modify(|s| {
                        s.insert((i as i32, j as i32));
                    })
                    .or_insert(HashSet::from([(i as i32, j as i32)]));
            }
        }
        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
        for (_, set) in &map {
            for (i, c1) in set.iter().enumerate() {
                for c2 in set.iter().skip(i + 1) {
                    let dx = c1.0 - c2.0;
                    let dy = c1.1 - c2.1;
                    for c in [c1, c2] {
                        for f in [i32::add, i32::sub] {
                            let mut m = 1;
                            loop {
                                let nc = (f(c.0, m * dx), f(c.1, m * dy));
                                if nc.0 < 0 || nc.0 >= len_x || nc.1 < 0 || nc.1 >= len_y {
                                    break;
                                }
                                antinodes.insert(nc);
                                m += 1;
                            }
                        }
                    }
                }
            }
        }
        Ok(antinodes.len() as i32)
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
