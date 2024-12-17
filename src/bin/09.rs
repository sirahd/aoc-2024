use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::iter::zip;

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i128> {
        let num: Vec<u32> = reader
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let mut id = 0;
        let mut layout: Vec<i32> = vec![];
        for (i, d) in num.iter().enumerate() {
            if i % 2 == 0 {
                layout.append(&mut vec![id; *d as usize]);
                id += 1;
            } else {
                layout.append(&mut vec![-1; *d as usize]);
            }
        }
        let mut end = layout.len() - 1;
        for i in 0..layout.len() {
            let n = layout[i];
            if n == -1 && i < end {
                while layout[end] == -1 {
                    end -= 1;
                }
                layout.swap(i, end);
            }
        }

        let checksum: i128 = layout
            .iter()
            .filter(|n| **n != -1)
            .enumerate()
            .map(|(i, n)| (i as i128) * (*n as i128))
            .sum();

        Ok(checksum)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i128> {
        let num: Vec<u32> = reader
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        // vec((num, starting, len))
        let mut disk_space: Vec<(i32, i32, i32)> = vec![];
        // vec((starting, len))
        let mut free_space: Vec<(i32, i32)> = vec![];
        let mut id = 0;
        let mut len = 0;
        for (i, d) in num.iter().enumerate() {
            if i % 2 == 0 {
                disk_space.push((id, len, *d as i32));
                id += 1;
            } else {
                free_space.push((len, *d as i32));
            }
            len += *d as i32;
        }
        for (_, start, len) in disk_space.iter_mut().rev() {
            'inner: for (free_start, free_len) in free_space.iter_mut() {
                if *free_start >= *start {
                    break 'inner;
                }
                if *free_len >= *len {
                    *start = *free_start;
                    *free_len -= *len;
                    *free_start += *len;
                    break 'inner;
                }
            }
        }
        let checksum: i128 = disk_space
            .into_iter()
            .flat_map(|(num, start, len)| zip(start..start + len, iter::repeat(num)))
            .map(|(i, num)| (num as i128) * (i as i128))
            .sum();
        Ok(checksum)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
