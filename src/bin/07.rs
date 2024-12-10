use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let lines = reader.lines();
        let mut total = 0;
        for line in lines {
            let line = line.unwrap();
            let split: Vec<&str> = line.split(":").collect();
            let sum = split[0].parse::<u64>().unwrap();
            let nums: Vec<u64> = split[1]
                .trim()
                .split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            if is_valid(sum, 0, nums) {
                total += sum;
            }
        }
        Ok(total)
    }

    fn is_valid(sum: u64, total: u64, nums: Vec<u64>) -> bool {
        if nums.is_empty() {
            return sum == total;
        }
        if nums.len() == 1 {
            return (total + nums[0] == sum) || (total * nums[0] == sum);
        }
        return is_valid(sum, total + nums[0], nums[1..].to_vec())
            || is_valid(sum, total * nums[0], nums[1..].to_vec());
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let lines = reader.lines();
        let mut total = 0;
        for line in lines {
            let line = line.unwrap();
            let split: Vec<&str> = line.split(":").collect();
            let sum = split[0].parse::<u64>().unwrap();
            let nums: Vec<u64> = split[1]
                .trim()
                .split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            if is_valid_concat(sum, 0, nums) {
                total += sum;
            }
        }
        Ok(total)
    }

    fn is_valid_concat(sum: u64, total: u64, nums: Vec<u64>) -> bool {
        if nums.is_empty() {
            return sum == total;
        }
        if nums.len() == 1 {
            return (total + nums[0] == sum)
                || (total * nums[0] == sum)
                || (total * 10u64.pow(nums[0].ilog10() + 1) + nums[0] == sum);
        }
        return is_valid_concat(sum, total + nums[0], nums[1..].to_vec())
            || is_valid_concat(sum, total * nums[0], nums[1..].to_vec())
            || is_valid_concat(
                sum,
                total * 10u64.pow(nums[0].ilog10() + 1) + nums[0],
                nums[1..].to_vec(),
            );
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
