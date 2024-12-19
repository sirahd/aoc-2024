use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut nums: Vec<u128> = reader
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u128>().unwrap())
            .collect();
        for _ in 0..25 {
            let mut new_nums = vec![];
            for n in nums {
                if n == 0 {
                    new_nums.push(1);
                } else if n.ilog10() % 2 == 1 {
                    // even digit
                    let s = n.to_string();
                    let (l, r) = s.split_at(s.len() / 2);
                    let l = l.parse::<u128>().unwrap();
                    let r = r
                        .trim_start_matches("0")
                        .parse::<u128>()
                        .unwrap_or_default();
                    new_nums.push(l);
                    new_nums.push(r);
                } else {
                    new_nums.push(n * 2024);
                }
            }
            nums = new_nums;
            // println!("{nums:?}");
        }
        Ok(nums.len() as i32)
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u128> {
        let mut nums: HashMap<u128, u128> = reader
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| (s.parse::<u128>().unwrap(), 1))
            .collect();
        for _ in 0..75 {
            let mut new_nums = HashMap::new();
            for (n, count) in &nums {
                if *n == 0 {
                    *new_nums.entry(1).or_default() += *count;
                } else if n.ilog10() % 2 == 1 {
                    // even digit
                    let s = n.to_string();
                    let (l, r) = s.split_at(s.len() / 2);
                    let l = l.parse::<u128>().unwrap();
                    let r = r
                        .trim_start_matches("0")
                        .parse::<u128>()
                        .unwrap_or_default();
                    *new_nums.entry(l).or_default() += *count;
                    *new_nums.entry(r).or_default() += *count;
                } else {
                    *new_nums.entry(n * 2024).or_default() += *count;
                }
            }
            nums = new_nums;
        }
        Ok(nums.values().sum())
    }

    assert_eq!(65601038650482, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
