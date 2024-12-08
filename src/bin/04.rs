use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let chars: Vec<Vec<char>> = reader
            .lines()
            .into_iter()
            .map(|l| l.unwrap().chars().collect())
            .collect();
        let mut total = 0;
        for (i, row) in chars.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                let cur = find_xmas(&chars, (j, i));
                total += cur;
            }
        }
        return Ok(total / 2);
    }

    fn find_xmas(chars: &Vec<Vec<char>>, (x, y): (usize, usize)) -> i32 {
        let len_y = chars.len();
        let len_x = chars[0].len();
        let mut total: Vec<String> = vec!["".to_string(); 8];
        for i in 0..4 {
            let x_new = x.saturating_add(i);
            let y_new = y.saturating_add(i);
            let x_neg_new = x.checked_add_signed(-(i as isize));
            let y_neg_new = y.checked_add_signed(-(i as isize));
            if x_new < len_x {
                total[0].push(chars[y][x_new])
            }
            if x_new < len_x && y_neg_new.is_some_and(|y| y < len_y) {
                total[1].push(chars[y_neg_new.unwrap()][x_new])
            }
            if y_neg_new.is_some_and(|y| y < len_y) {
                total[2].push(chars[y_neg_new.unwrap()][x])
            }
            if x_neg_new.is_some_and(|x| x < len_x) && y_neg_new.is_some_and(|y| y < len_y) {
                total[3].push(chars[y_neg_new.unwrap()][x_neg_new.unwrap()])
            }
            if x_neg_new.is_some_and(|x| x < len_x) {
                total[4].push(chars[y][x_neg_new.unwrap()])
            }
            if x_neg_new.is_some_and(|x| x < len_x) && y_new < len_y {
                total[5].push(chars[y_new][x_neg_new.unwrap()])
            }
            if y_new < len_y {
                total[6].push(chars[y_new][x])
            }
            if x_new < len_x && y_new < len_y {
                total[7].push(chars[y_new][x_new])
            }
        }
        total
            .iter()
            .filter(|s| **s == "XMAS" || **s == "SAMX")
            .count() as i32
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let chars: Vec<Vec<char>> = reader
            .lines()
            .into_iter()
            .map(|l| l.unwrap().chars().collect())
            .collect();
        let mut total = 0;
        let len_x = chars[0].len();
        let len_y = chars.len();
        for j in 1..len_y - 1 {
            for i in 1..len_x - 1 {
                if chars[j][i] == 'A' && is_xmas(&chars, (i, j)) {
                    total += 1
                }
            }
        }
        return Ok(total);
    }

    fn is_xmas(chars: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
        let tl = chars[(y as isize - 1) as usize][(x as isize - 1) as usize];
        let tr = chars[(y as isize - 1) as usize][x + 1];
        let br = chars[y + 1][x + 1];
        let bl = chars[y + 1][(x as isize - 1) as usize];
        let strd = format!("{tl}A{br}");
        let stru = format!("{tr}A{bl}");
        return (strd == "SAM" || strd == "MAS") && (stru == "SAM" || stru == "MAS");
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
