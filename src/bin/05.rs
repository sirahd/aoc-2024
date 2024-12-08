use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut iter = reader.lines();

        let mut total = 0;
        let map = build_map(&mut iter);
        while let Some(line) = iter.next() {
            let line = line.unwrap();
            let books: Vec<_> = line.split(",").map(String::from).collect();

            let mut is_book_ordered = true;
            for (i, a) in books.iter().enumerate() {
                for b in books.iter().skip(i + 1) {
                    is_book_ordered &= map.get(a).is_some_and(|s| s.contains(b));
                }
            }
            if is_book_ordered {
                total += books[books.len() / 2].parse::<i32>().unwrap();
            }
        }
        Ok(total)
    }

    fn build_map<R: BufRead>(iter: &mut Lines<R>) -> HashMap<String, HashSet<String>> {
        let mut map: HashMap<String, HashSet<String>> = HashMap::new();
        while let Some(line) = iter.next() {
            let line = line.unwrap();
            if line.is_empty() {
                break;
            }
            let pages: Vec<_> = line.split("|").map(String::from).collect();
            map.entry(pages[0].to_string())
                .or_insert(HashSet::new())
                .insert(pages[1].to_string());
        }
        map
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut iter = reader.lines();

        let mut total = 0;
        let mut map = build_map(&mut iter);
        while let Some(line) = iter.next() {
            let line = line.unwrap();
            let mut books: Vec<_> = line.split(",").map(String::from).collect();

            let mut books_corrected = false;
            for i in 0..books.len() {
                for j in i + 1..books.len() {
                    if map.get(&books[i]).is_none()
                        || map.get(&books[i]).is_some_and(|s| !s.contains(&books[j]))
                    {
                        books_corrected = true;
                        books.swap(i, j);
                    }
                }
            }
            if books_corrected {
                total += books[books.len() / 2].parse::<i32>().unwrap();
            }
        }
        Ok(total)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
