use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[derive(Eq, PartialEq, Hash, Clone, Copy)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let input: Vec<String> = reader
            .lines()
            .map(Result::unwrap)
            .map(String::from)
            .collect();
        let len_y = input.len() as i32;
        let len_x = input[0].len() as i32;
        let (obstacles, mut pos) = find_obstacles_and_starting_pos(input);

        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut dir = Direction::Up;
        loop {
            let (x, y) = pos;
            if x < 0 || x >= len_x || y < 0 || y >= len_y {
                break;
            }
            visited.insert(pos);
            match dir {
                Direction::Up => {
                    if obstacles.contains(&(x, y - 1)) {
                        dir = Direction::Right;
                    } else {
                        pos = (x, y - 1);
                    }
                }
                Direction::Down => {
                    if obstacles.contains(&(x, y + 1)) {
                        dir = Direction::Left;
                    } else {
                        pos = (x, y + 1);
                    }
                }
                Direction::Left => {
                    if obstacles.contains(&(x - 1, y)) {
                        dir = Direction::Up;
                    } else {
                        pos = (x - 1, y);
                    }
                }
                Direction::Right => {
                    if obstacles.contains(&(x + 1, y)) {
                        dir = Direction::Down;
                    } else {
                        pos = (x + 1, y);
                    }
                }
            }
        }
        Ok(visited.len() as i32)
    }

    fn find_obstacles_and_starting_pos(input: Vec<String>) -> (HashSet<(i32, i32)>, (i32, i32)) {
        let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
        let mut pos = (0, 0);
        for (j, line) in input.iter().enumerate() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    obstacles.insert((i as i32, j as i32));
                }
                if c == '^' {
                    pos = (i as i32, j as i32);
                }
            }
        }
        (obstacles, pos)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let input: Vec<String> = reader
            .lines()
            .map(Result::unwrap)
            .map(String::from)
            .collect();
        let len_y = input.len() as i32;
        let len_x = input[0].len() as i32;
        let (obstacles, mut pos) = find_obstacles_and_starting_pos(input);
        // let mut visited = HashSet::new();
        // let mut obstruction = HashSet::new();
        // let mut dir = Direction::Up;
        let mut num_obstruction = 0;
        for i in 0..len_x {
            for j in 0..len_y {
                if !obstacles.contains(&(i, j)) && (i, j) != pos {
                    let mut new_obstacles = obstacles.clone();
                    new_obstacles.insert((i, j));
                    if has_cycle(
                        new_obstacles,
                        pos,
                        (len_x, len_y),
                        Direction::Up,
                        HashSet::new(),
                    ) {
                        num_obstruction += 1;
                    }
                }
            }
        }
        // loop {
        //     let (x, y) = pos;
        //     if x < 0 || x >= len_x || y < 0 || y >= len_y {
        //         break;
        //     }
        //     visited.insert((pos, dir));
        //     match dir {
        //         Direction::Up => {
        //             if obstacles.contains(&(x, y - 1)) {
        //                 dir = Direction::Right;
        //             } else {
        //                 if y - 1 >= 0 {
        //                     let mut new_obstacles = obstacles.clone();
        //                     new_obstacles.insert((x, y - 1));
        //                     if has_cycle(new_obstacles, pos, (len_x, len_y), dir, HashSet::new()) {
        //                         obstruction.insert((x, y - 1));
        //                     }
        //                 }
        //                 pos = (x, y - 1);
        //             }
        //         }
        //         Direction::Down => {
        //             if obstacles.contains(&(x, y + 1)) {
        //                 dir = Direction::Left;
        //             } else {
        //                 if y + 1 < len_y {
        //                     let mut new_obstacles = obstacles.clone();
        //                     new_obstacles.insert((x, y + 1));
        //                     if has_cycle(new_obstacles, pos, (len_x, len_y), dir, HashSet::new()) {
        //                         obstruction.insert((x, y + 1));
        //                     }
        //                 }
        //                 pos = (x, y + 1);
        //             }
        //         }
        //         Direction::Left => {
        //             if obstacles.contains(&(x - 1, y)) {
        //                 dir = Direction::Up;
        //             } else {
        //                 if x - 1 >= 0 {
        //                     let mut new_obstacles = obstacles.clone();
        //                     new_obstacles.insert((x - 1, y));
        //                     if has_cycle(new_obstacles, pos, (len_x, len_y), dir, HashSet::new()) {
        //                         obstruction.insert((x - 1, y));
        //                     }
        //                 }
        //                 pos = (x - 1, y);
        //             }
        //         }
        //         Direction::Right => {
        //             if obstacles.contains(&(x + 1, y)) {
        //                 dir = Direction::Down;
        //             } else {
        //                 if x + 1 < len_x {
        //                     let mut new_obstacles = obstacles.clone();
        //                     new_obstacles.insert((x + 1, y));
        //                     if has_cycle(new_obstacles, pos, (len_x, len_y), dir, HashSet::new()) {
        //                         obstruction.insert((x + 1, y));
        //                     }
        //                 }
        //                 pos = (x + 1, y);
        //             }
        //         }
        //     }
        // }
        // Ok(obstruction.len() as i32)
        Ok(num_obstruction)
    }
    fn has_cycle(
        obstacles: HashSet<(i32, i32)>,
        mut pos: (i32, i32),
        dim: (i32, i32),
        mut dir: Direction,
        mut visited: HashSet<((i32, i32), Direction)>,
    ) -> bool {
        let (len_x, len_y) = dim;
        loop {
            let (x, y) = pos;
            if x < 0 || x >= len_x || y < 0 || y >= len_y {
                return false;
            }
            if visited.contains(&(pos, dir)) {
                return true;
            }
            visited.insert((pos, dir));
            match dir {
                Direction::Up => {
                    if obstacles.contains(&(x, y - 1)) {
                        dir = Direction::Right;
                    } else {
                        pos = (x, y - 1);
                    }
                }
                Direction::Down => {
                    if obstacles.contains(&(x, y + 1)) {
                        dir = Direction::Left;
                    } else {
                        pos = (x, y + 1);
                    }
                }
                Direction::Left => {
                    if obstacles.contains(&(x - 1, y)) {
                        dir = Direction::Up;
                    } else {
                        pos = (x - 1, y);
                    }
                }
                Direction::Right => {
                    if obstacles.contains(&(x + 1, y)) {
                        dir = Direction::Down;
                    } else {
                        pos = (x + 1, y);
                    }
                }
            }
        }
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
