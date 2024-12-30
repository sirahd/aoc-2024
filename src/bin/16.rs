use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;

const DAY: &str = "16";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

const TEST2: &str = "\
#####
###E#
#...#
#S.##
#####";

const TEST3: &str = "\
######
###.E#
###.##
#...##
#.#.##
#...##
#.####
#S####
######";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let map: Vec<Vec<char>> = reader
            .lines()
            .map(|l| l.unwrap().chars().collect())
            .collect();

        let scores = calculate_scores(&map);
        // print_map(&map, &scores);
        Ok(scores[1][map[0].len() - 2])
    }

    fn calculate_scores(map: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
        let mut scores = vec![vec![i32::MAX; map[0].len()]; map.len()];

        let start = (1, map.len() - 2);
        let mut queue: VecDeque<((usize, usize), i32, i32)> = VecDeque::from([(start, 0, 0)]);
        while !queue.is_empty() {
            let ((px, py), score, dir) = queue.pop_front().unwrap();
            for (nx, ny, mut ndir) in [
                (px + 1, py, 0),
                (px, py.saturating_sub(1), 90),
                (px.saturating_sub(1), py, 180),
                (px, py + 1, 270),
            ] {
                if let Some(r) = map.get(ny) {
                    if let Some(c) = r.get(nx) {
                        if *c == '.' || *c == 'E' {
                            let mut new_score = score + 1;
                            if ndir != dir {
                                new_score += 1000;
                            }
                            queue.push_back(((nx, ny), new_score, dir));
                        }
                    }
                }
            }
        }
        scores
    }

    // fn dfs() {
    //     dp[i][j] = dp[i][j-1]
    // }

    #[allow(dead_code)]
    fn print_map(map: &Vec<Vec<char>>, scores: &Vec<Vec<i32>>) {
        for j in 0..map.len() {
            for i in 0..map[j].len() {
                if map[j][i] != '#' {
                    print!("{},", scores[j][i]);
                } else {
                    print!("{}", map[j][i]);
                }
            }
            println!();
        }
    }

    // assert_eq!(11048, part1(BufReader::new(TEST.as_bytes()))?);

    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file)?);
    // println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let map: Vec<Vec<char>> = reader
            .lines()
            .map(|l| l.unwrap().chars().collect())
            .collect();
        let mut scores = calculate_scores(&map);
        print_map(&map, &scores);

        let end = (map[0].len() - 2, 1);
        let mut queue: VecDeque<((usize, usize), HashSet<(usize, usize)>)> =
            VecDeque::from([(end, HashSet::from([end]))]);
        let mut best_paths: HashSet<(usize, usize)> = HashSet::new();
        while !queue.is_empty() {
            let ((px, py), paths) = queue.pop_front().unwrap();
            if map[py][px] == 'S' {
                best_paths.extend(paths.iter());
            }
            let cur_score = scores[py][px];

            for (nx, ny) in [
                (px + 1, py),
                (px, py.saturating_sub(1)),
                (px.saturating_sub(1), py),
                (px, py + 1),
            ] {
                if paths.contains(&(nx, ny)) {
                    continue;
                }
                if let Some(r) = scores.get(ny) {
                    if let Some(score) = r.get(nx) {
                        if *score == cur_score - 1
                            || *score == cur_score - 1001
                            || map[ny][nx] == 'S'
                        {
                            let mut new_paths = paths.clone();
                            new_paths.insert((nx, ny));
                            queue.push_front(((nx, ny), new_paths));
                        }
                    }
                }
            }
        }
        print_best_paths(&best_paths, (map[0].len(), map.len()));

        Ok(best_paths.len() as i32)
    }

    fn print_best_paths(best_paths: &HashSet<(usize, usize)>, (len_x, len_y): (usize, usize)) {
        for j in 0..len_y {
            for i in 0..len_x {
                if best_paths.contains(&(i, j)) {
                    print!("O")
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }

    assert_eq!(64, part2(BufReader::new(TEST3.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
