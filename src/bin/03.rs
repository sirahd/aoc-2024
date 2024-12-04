use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;

use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<i32> {
        let mut text: String = "".to_string();
        reader.read_to_string(&mut text).unwrap();
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let mut sum = 0;
        for (_, [first, second]) in re.captures_iter(text.as_str()).map(|c| c.extract()) {
            let l = first.parse::<i32>().unwrap_or_default();
            let r = second.parse::<i32>().unwrap_or_default();
            sum += l * r;
        }
        Ok(sum)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<i32> {
        let mut text: String = "".to_string();
        reader.read_to_string(&mut text).unwrap();
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(don't)(\(\))|(do)(\(\))").unwrap();
        let mut sum = 0;
        let mut active = true;
        for (_, [first, second]) in re.captures_iter(text.as_str()).map(|c| c.extract()) {
            if first == "don't" {
                active = false;
                continue;
            }
            if first == "do" {
                active = true;
                continue;
            }
            if active {
                let l = first.parse::<i32>().unwrap_or_default();
                let r = second.parse::<i32>().unwrap_or_default();
                sum += l * r;
            }
        }
        Ok(sum)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
