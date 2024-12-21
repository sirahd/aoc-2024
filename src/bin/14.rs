use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut robots: Vec<((i32, i32), (i32, i32))> = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            let (pos, speed) = line.split_once(" ").unwrap();
            let (px, py) = pos[2..].split_once(",").unwrap();
            let (sx, sy) = speed[2..].split_once(",").unwrap();
            robots.push((
                (px.parse().unwrap(), py.parse().unwrap()),
                (sx.parse().unwrap(), sy.parse().unwrap()),
            ))
        }
        let (len_x, len_y) = (101, 103);
        for _ in 0..100 {
            for ((px, py), (sx, sy)) in robots.iter_mut() {
                *px = (*px + *sx + len_x) % len_x;
                *py = (*py + *sy + len_y) % len_y;
            }
        }
        let mut quadrant = [0; 4];
        let (mid_x, mid_y) = (len_x / 2, len_y / 2);
        for ((px, py), _) in robots {
            // top left
            if px < mid_x && py < mid_y {
                quadrant[0] += 1;
            }
            // top right
            if px > mid_x && py < mid_y {
                quadrant[1] += 1;
            }
            // bottom left
            if px < mid_x && py > mid_y {
                quadrant[2] += 1;
            }
            // bottom right
            if px > mid_x && py > mid_y {
                quadrant[3] += 1;
            }
        }
        Ok(quadrant.iter().product())
    }

    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut robots: Vec<((i32, i32), (i32, i32))> = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            let (pos, speed) = line.split_once(" ").unwrap();
            let (px, py) = pos[2..].split_once(",").unwrap();
            let (sx, sy) = speed[2..].split_once(",").unwrap();
            robots.push((
                (px.parse().unwrap(), py.parse().unwrap()),
                (sx.parse().unwrap(), sy.parse().unwrap()),
            ))
        }
        let (len_x, len_y) = (101, 103);
        for i in 0..100000 {
            let mut unique_robots = HashSet::new();
            for ((px, py), (sx, sy)) in robots.iter_mut() {
                *px = (*px + *sx + len_x) % len_x;
                *py = (*py + *sy + len_y) % len_y;
                unique_robots.insert((*px, *py));
            }
            if unique_robots.len() == robots.len() {
                println!("{} ====================================================================================", i+1);
                display_tile(&robots, (len_x, len_y));
            }
        }
        Ok(0)
    }

    fn display_tile(robots: &Vec<((i32, i32), (i32, i32))>, (len_x, len_y): (i32, i32)) {
        let mut tiles = vec![vec![".".to_string(); len_x as usize]; len_y as usize];
        for ((px, py), _) in robots {
            tiles[*py as usize][*px as usize] = "O".to_string();
        }
        for row in tiles {
            for c in row {
                print!("{c}");
            }
            println!();
        }
    }

    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
