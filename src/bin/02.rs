use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut num_safe = 0;
        for line in reader.lines() {
            let levels: Vec<i32> = line
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            if is_safe(levels) {
                num_safe += 1;
            }
        }
        return Ok(num_safe);
    }

    fn is_safe(levels: Vec<i32>) -> bool {
        let diff: Vec<i32> = levels.windows(2).map(|w| w[1] - w[0]).collect();
        let first_sign = diff.first().unwrap().signum();
        return diff
            .iter()
            .all(|e| e.abs() <= 3 && e.signum() == first_sign);
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut num_safe = 0;
        for line in reader.lines() {
            let levels: Vec<i32> = line
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            let mut safe = false;
            for i in 0..levels.len() {
                let mut new_level = levels.clone();
                new_level.remove(i);
                safe |= is_safe(new_level);
            }
            if safe {
                num_safe += 1;
            }
        }
        return Ok(num_safe);
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
