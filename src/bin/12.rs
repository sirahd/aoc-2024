use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let plot: Vec<Vec<char>> = reader
            .lines()
            .map(|l| l.unwrap().chars().collect())
            .collect();
        let len_x = plot[0].len() as i32;
        let len_y = plot.len() as i32;
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut total = 0;
        for j in 0..len_y {
            for i in 0..len_x {
                if visited.contains(&(i, j)) {
                    continue;
                }
                let plant = plot[j as usize][i as usize];
                let mut same_plant: VecDeque<(i32, i32)> = VecDeque::from([(i, j)]);
                visited.insert((i, j));
                let mut area = 0;
                let mut perimeter = 0;
                while !same_plant.is_empty() {
                    let (ci, cj) = same_plant.pop_front().unwrap();
                    area += 1;
                    perimeter += 4;
                    for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let (ni, nj) = (ci + di, cj + dj);
                        if let Some(r) = plot.get(nj as usize) {
                            if let Some(p) = r.get(ni as usize) {
                                if *p == plant {
                                    perimeter -= 1;
                                    if !visited.contains(&(ni, nj)) {
                                        same_plant.push_back((ni, nj));
                                        visited.insert((ni, nj));
                                    }
                                }
                            }
                        }
                    }
                }
                total += area * perimeter;
            }
        }
        Ok(total)
    }
    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let plot: Vec<Vec<char>> = reader
            .lines()
            .map(|l| l.unwrap().chars().collect())
            .collect();
        let len_x = plot[0].len() as i32;
        let len_y = plot.len() as i32;
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut total = 0;
        for j in 0..len_y {
            for i in 0..len_x {
                if visited.contains(&(i, j)) {
                    continue;
                }
                let plant = plot[j as usize][i as usize];
                let mut same_plant: VecDeque<(i32, i32)> = VecDeque::from([(i, j)]);
                let mut region = HashSet::from([(i, j)]);
                visited.insert((i, j));
                let mut area = 0;
                while !same_plant.is_empty() {
                    let (ci, cj) = same_plant.pop_front().unwrap();
                    area += 1;
                    for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let (ni, nj) = (ci + di, cj + dj);
                        if let Some(r) = plot.get(nj as usize) {
                            if let Some(p) = r.get(ni as usize) {
                                if *p == plant {
                                    if !visited.contains(&(ni, nj)) {
                                        same_plant.push_back((ni, nj));
                                        visited.insert((ni, nj));
                                        region.insert((ni, nj));
                                    }
                                }
                            }
                        }
                    }
                }
                let mut edge = 0;
                for (ni, nj) in &region {
                    let n = (*ni, *nj - 1);
                    let w = (*ni - 1, *nj);
                    let s = (*ni, *nj + 1);
                    let e = (*ni + 1, *nj);
                    let nw = (*ni - 1, *nj - 1);
                    let sw = (*ni - 1, *nj + 1);
                    let se = (*ni + 1, *nj + 1);
                    let ne = (*ni + 1, *nj - 1);

                    // north
                    // same edge if north is the same plant AND northwest is not in the region
                    // corner if north is not in the region OR northwest is in the region
                    if !region.contains(&n) && (!region.contains(&w) || region.contains(&nw)) {
                        edge += 1;
                    }
                    // west
                    if !region.contains(&w) && (!region.contains(&s) || region.contains(&sw)) {
                        edge += 1;
                    }
                    // south
                    if !region.contains(&s) && (!region.contains(&e) || region.contains(&se)) {
                        edge += 1;
                    }
                    // east
                    if !region.contains(&e) && (!region.contains(&n) || region.contains(&ne)) {
                        edge += 1;
                    }
                }
                total += area * edge;
            }
        }
        Ok(total)
    }

    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
