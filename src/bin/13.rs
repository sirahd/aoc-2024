use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;

use adv_code_2024::*;

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<u32> {
        let mut lines: String = String::new();
        reader.read_to_string(&mut lines).unwrap();
        let mut total = 0;
        for machine in lines.split("\n\n") {
            let ((max, may), (mbx, mby), (px, py)) = parse_machine::<u32>(machine);
            let mut token = u32::MAX;
            for a in 0..100 {
                for b in 0..100 {
                    if px == max * a + mbx * b && py == may * a + mby * b {
                        token = min(token, 3 * a + b)
                    }
                }
            }
            if token != u32::MAX {
                total += token
            }
        }
        Ok(total)
    }

    fn parse_machine<T: FromStr + Default>(lines: &str) -> ((T, T), (T, T), (T, T)) {
        let re_machine = Regex::new(r"Button \w: X\+(\d+), Y\+(\d+)").unwrap();
        let re_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        let split: Vec<&str> = lines.split("\n").collect();
        let (_, [max, may]) = re_machine.captures(split[0]).unwrap().extract();
        let (max, may) = (
            max.parse().unwrap_or_default(),
            may.parse().unwrap_or_default(),
        );

        let (_, [mbx, mby]) = re_machine.captures(split[1]).unwrap().extract();
        let (mbx, mby) = (
            mbx.parse().unwrap_or_default(),
            mby.parse().unwrap_or_default(),
        );

        let (_, [px, py]) = re_prize.captures(split[2]).unwrap().extract();
        let (px, py) = (
            px.parse().unwrap_or_default(),
            py.parse().unwrap_or_default(),
        );

        ((max, may), (mbx, mby), (px, py))
    }

    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<i128> {
        let mut lines: String = String::new();
        reader.read_to_string(&mut lines).unwrap();
        let mut total = 0;
        for machine in lines.split("\n\n") {
            let ((max, may), (mbx, mby), (mut px, mut py)) = parse_machine::<i128>(machine);
            px = px + 10000000000000;
            py = py + 10000000000000;
            let first_term_a = px * mby - py * mbx;
            let second_term_a = max * mby - may * mbx;
            let pa = first_term_a % second_term_a;
            if pa != 0 {
                continue;
            }
            let first_term_b = px * may - py * max;
            let second_term_b = mbx * may - mby * max;
            let pb = first_term_b % second_term_b;
            if pb != 0 {
                continue;
            }
            let na = first_term_a / second_term_a;
            let nb = first_term_b / second_term_b;
            total += na * 3 + nb;
        }
        Ok(total)
    }

    assert_eq!(875318608908, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
