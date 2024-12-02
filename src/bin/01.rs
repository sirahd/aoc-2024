use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut a = vec![];
        let mut b = vec![];
        for line in reader.lines() {
            let mut entry = line.as_ref().unwrap().split_whitespace();
            a.push(entry.next().unwrap().parse::<i32>().unwrap());
            b.push(entry.next().unwrap().parse::<i32>().unwrap());
        }
        a.sort();
        b.sort();
        return Ok(zip(a, b).map(|(x, y)| (x - y).abs()).sum());
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut a = vec![];
        let mut b: HashMap<i32, i32> = HashMap::new();
        for line in reader.lines() {
            let mut entry = line.as_ref().unwrap().split_whitespace();
            a.push(entry.next().unwrap().parse::<i32>().unwrap());
            b.entry(entry.next().unwrap().parse::<i32>().unwrap())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        return Ok(a.iter().map(|e| e * b.get(e).unwrap_or(&0)).sum());
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
