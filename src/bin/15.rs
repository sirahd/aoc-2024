use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

use adv_code_2024::*;

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<i32> {
        let mut lines = String::new();
        reader.read_to_string(&mut lines).unwrap();
        let (map, moves) = lines.split_once("\n\n").unwrap();
        let map: Vec<&str> = map.split("\n").collect();
        let moves: Vec<&str> = moves.split("\n").collect();
        let (len_x, len_y) = (map[0].len(), map.len());
        let mut wall = HashSet::new();
        let mut boxes = HashSet::new();
        let mut robot = (0, 0);
        for (j, line) in map.iter().enumerate() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    wall.insert((i, j));
                } else if c == 'O' {
                    boxes.insert((i, j));
                } else if c == '@' {
                    robot = (i, j);
                }
            }
        }
        for line in moves {
            for m in line.chars() {
                if m == '>' {
                    // figure out all boxes that need to be moved
                    let mut adjacent_boxes = vec![];
                    let (robot_x, robot_y) = robot;
                    'inner: for i in robot_x + 1..len_x {
                        if wall.contains(&(i, robot_y)) {
                            break;
                        }
                        if boxes.remove(&(i, robot_y)) {
                            adjacent_boxes.push((i, robot_y));
                        } else {
                            break 'inner;
                        }
                    }
                    // move them from right to left
                    for (b_x, b_y) in adjacent_boxes.into_iter().rev() {
                        if wall.contains(&(b_x + 1, b_y)) || boxes.contains(&(b_x + 1, b_y)) {
                            boxes.insert((b_x, b_y));
                        } else {
                            boxes.insert((b_x + 1, b_y));
                        }
                    }
                    if !wall.contains(&(robot_x + 1, robot_y))
                        && !boxes.contains(&(robot_x + 1, robot_y))
                    {
                        robot.0 += 1;
                    }
                } else if m == '<' {
                    // figure out all boxes that need to be moved
                    let mut adjacent_boxes = vec![];
                    let (robot_x, robot_y) = robot;
                    'inner: for i in (0..robot_x).rev() {
                        if wall.contains(&(i, robot_y)) {
                            break;
                        }
                        if boxes.remove(&(i, robot_y)) {
                            adjacent_boxes.push((i, robot_y));
                        } else {
                            break 'inner;
                        }
                    }
                    // move them from left to right
                    for (b_x, b_y) in adjacent_boxes.into_iter().rev() {
                        if wall.contains(&(b_x - 1, b_y)) || boxes.contains(&(b_x - 1, b_y)) {
                            boxes.insert((b_x, b_y));
                        } else {
                            boxes.insert((b_x - 1, b_y));
                        }
                    }
                    if !wall.contains(&(robot_x - 1, robot_y))
                        && !boxes.contains(&(robot_x - 1, robot_y))
                    {
                        robot.0 -= 1;
                    }
                } else if m == '^' {
                    // figure out all boxes that need to be moved
                    let mut adjacent_boxes = vec![];
                    let (robot_x, robot_y) = robot;
                    'inner: for j in (0..robot_y).rev() {
                        if wall.contains(&(robot_x, j)) {
                            break;
                        }
                        if boxes.remove(&(robot_x, j)) {
                            adjacent_boxes.push((robot_x, j));
                        } else {
                            break 'inner;
                        }
                    }
                    // move them from left to right
                    for (b_x, b_y) in adjacent_boxes.into_iter().rev() {
                        if wall.contains(&(b_x, b_y - 1)) || boxes.contains(&(b_x, b_y - 1)) {
                            boxes.insert((b_x, b_y));
                        } else {
                            boxes.insert((b_x, b_y - 1));
                        }
                    }
                    if !wall.contains(&(robot_x, robot_y - 1))
                        && !boxes.contains(&(robot_x, robot_y - 1))
                    {
                        robot.1 -= 1;
                    }
                } else {
                    // 'v'
                    let mut adjacent_boxes = vec![];
                    let (robot_x, robot_y) = robot;
                    'inner: for j in robot_y + 1..len_y {
                        if wall.contains(&(robot_x, j)) {
                            break;
                        }
                        if boxes.remove(&(robot_x, j)) {
                            adjacent_boxes.push((robot_x, j));
                        } else {
                            break 'inner;
                        }
                    }
                    // move them from left to right
                    for (b_x, b_y) in adjacent_boxes.into_iter().rev() {
                        if wall.contains(&(b_x, b_y + 1)) || boxes.contains(&(b_x, b_y + 1)) {
                            boxes.insert((b_x, b_y));
                        } else {
                            boxes.insert((b_x, b_y + 1));
                        }
                    }
                    if !wall.contains(&(robot_x, robot_y + 1))
                        && !boxes.contains(&(robot_x, robot_y + 1))
                    {
                        robot.1 += 1;
                    }
                }
            }
        }
        // print_map(&wall, &boxes, robot, (len_x, len_y));
        let mut sum = 0;
        for (bx, by) in boxes {
            sum += (by * 100 + bx) as i32;
        }
        Ok(sum)
    }

    #[allow(dead_code)]
    fn print_map(
        wall: &HashSet<(usize, usize)>,
        boxes: &HashSet<(usize, usize)>,
        robot: (usize, usize),
        (len_x, len_y): (usize, usize),
    ) {
        for j in 0..len_y {
            for i in 0..len_x {
                if robot == (i, j) {
                    print!("@");
                } else if wall.contains(&(i, j)) {
                    print!("#");
                } else if boxes.contains(&(i, j)) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    assert_eq!(10092, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(mut reader: R) -> Result<i32> {
        let mut lines = String::new();
        reader.read_to_string(&mut lines).unwrap();
        let (map, moves) = lines.split_once("\n\n").unwrap();
        let map: Vec<&str> = map.split("\n").collect();
        let moves: Vec<&str> = moves.split("\n").collect();
        let (len_x, len_y) = (map[0].len() * 2, map.len());
        let mut wall = HashSet::new();
        let mut boxes = HashSet::new();
        let mut robot = (0, 0);
        for (j, line) in map.iter().enumerate() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    wall.insert((2 * i, j));
                    wall.insert((2 * i + 1, j));
                } else if c == 'O' {
                    boxes.insert((2 * i, j));
                } else if c == '@' {
                    robot = (2 * i, j);
                }
            }
        }
        for line in moves {
            for m in line.chars() {
                if m == '>' {
                    // figure out all boxes that need to be moved
                    let mut adjacent_boxes = vec![];
                    let (robot_x, robot_y) = robot;
                    'inner: for i in (robot_x + 1..len_x).step_by(2) {
                        if wall.contains(&(i, robot_y)) {
                            break 'inner;
                        }
                        if boxes.remove(&(i, robot_y)) {
                            adjacent_boxes.push((i, robot_y));
                        } else {
                            break 'inner;
                        }
                    }
                    // move them from right to left
                    for (b_x, b_y) in adjacent_boxes.into_iter().rev() {
                        if wall.contains(&(b_x + 2, b_y)) || boxes.contains(&(b_x + 2, b_y)) {
                            boxes.insert((b_x, b_y));
                        } else {
                            boxes.insert((b_x + 1, b_y));
                        }
                    }
                    if !wall.contains(&(robot_x + 1, robot_y))
                        && !boxes.contains(&(robot_x + 1, robot_y))
                    {
                        robot.0 += 1;
                    }
                } else if m == '<' {
                    // figure out all boxes that need to be moved
                    let mut adjacent_boxes = vec![];
                    let (robot_x, robot_y) = robot;
                    'inner: for i in (0..robot_x - 1).rev().step_by(2) {
                        if wall.contains(&(i, robot_y)) {
                            break;
                        }
                        if boxes.remove(&(i, robot_y)) {
                            adjacent_boxes.push((i, robot_y));
                        } else {
                            break 'inner;
                        }
                    }
                    // move them from left to right
                    for (b_x, b_y) in adjacent_boxes.into_iter().rev() {
                        if wall.contains(&(b_x - 1, b_y)) || boxes.contains(&(b_x - 2, b_y)) {
                            boxes.insert((b_x, b_y));
                        } else {
                            boxes.insert((b_x - 1, b_y));
                        }
                    }
                    if !wall.contains(&(robot_x - 1, robot_y))
                        && !boxes.contains(&(robot_x - 2, robot_y))
                    {
                        robot.0 -= 1;
                    }
                } else if m == '^' {
                    // figure out all boxes that need to be moved
                    let (robot_x, robot_y) = robot;
                    let mut adjacent_boxes = vec![];
                    let mut frontier_boxes = vec![];
                    if boxes.remove(&(robot_x, robot_y - 1)) {
                        adjacent_boxes.push((robot_x, robot_y - 1));
                        frontier_boxes.push((robot_x, robot_y - 1));
                    } else if boxes.remove(&(robot_x - 1, robot_y - 1)) {
                        adjacent_boxes.push((robot_x - 1, robot_y - 1));
                        frontier_boxes.push((robot_x - 1, robot_y - 1));
                    }
                    // TODO: BFS on boxes
                    for j in (0..robot_y - 1).rev() {
                        let mut new_frontier_boxes = vec![];
                        for (bx, _) in frontier_boxes {
                            for offset in -1..=1 {
                                let b = (bx.wrapping_add_signed(offset), j);
                                if boxes.remove(&b) {
                                    adjacent_boxes.push(b);
                                    new_frontier_boxes.push(b);
                                }
                            }
                        }
                        frontier_boxes = new_frontier_boxes;
                    }
                    // TODO: move them up but only if all of them can be moved without running into walls
                    let adjacent_boxes_clone = adjacent_boxes.clone();
                    let mut can_move_boxes = true;
                    for (b_x, b_y) in adjacent_boxes_clone.into_iter().rev() {
                        can_move_boxes &= !wall.contains(&(b_x, b_y - 1))
                            && !wall.contains(&(b_x + 1, b_y - 1))
                            && !boxes.contains(&(b_x, b_y - 1));
                    }
                    for (b_x, b_y) in adjacent_boxes.into_iter().rev() {
                        if can_move_boxes {
                            boxes.insert((b_x, b_y - 1));
                        } else {
                            boxes.insert((b_x, b_y));
                        }
                    }
                    if !wall.contains(&(robot_x, robot_y - 1))
                        && !boxes.contains(&(robot_x, robot_y - 1))
                        && !boxes.contains(&(robot_x - 1, robot_y - 1))
                    {
                        robot.1 -= 1;
                    }
                } else {
                    // 'v'
                    // figure out all boxes that need to be moved
                    let (robot_x, robot_y) = robot;
                    let mut adjacent_boxes = vec![];
                    let mut frontier_boxes = vec![];
                    if boxes.remove(&(robot_x, robot_y + 1)) {
                        adjacent_boxes.push((robot_x, robot_y + 1));
                        frontier_boxes.push((robot_x, robot_y + 1));
                    } else if boxes.remove(&(robot_x - 1, robot_y + 1)) {
                        adjacent_boxes.push((robot_x - 1, robot_y + 1));
                        frontier_boxes.push((robot_x - 1, robot_y + 1));
                    }
                    // TODO: BFS on boxes
                    for j in robot_y + 2..len_y {
                        let mut new_frontier_boxes = vec![];
                        for (bx, _) in frontier_boxes {
                            for offset in -1..=1 {
                                let b = (bx.wrapping_add_signed(offset), j);
                                if boxes.remove(&b) {
                                    adjacent_boxes.push(b);
                                    new_frontier_boxes.push(b);
                                }
                            }
                        }
                        frontier_boxes = new_frontier_boxes;
                    }
                    // TODO: move them up but only if all of them can be moved without running into walls
                    let adjacent_boxes_clone = adjacent_boxes.clone();
                    let mut can_move_boxes = true;
                    for (b_x, b_y) in adjacent_boxes_clone.into_iter().rev() {
                        can_move_boxes &= !wall.contains(&(b_x, b_y + 1))
                            && !wall.contains(&(b_x + 1, b_y + 1))
                            && !boxes.contains(&(b_x, b_y + 1));
                    }
                    for (b_x, b_y) in adjacent_boxes.into_iter().rev() {
                        if can_move_boxes {
                            boxes.insert((b_x, b_y + 1));
                        } else {
                            boxes.insert((b_x, b_y));
                        }
                    }
                    if !wall.contains(&(robot_x, robot_y + 1))
                        && !boxes.contains(&(robot_x, robot_y + 1))
                        && !boxes.contains(&(robot_x - 1, robot_y + 1))
                    {
                        robot.1 += 1;
                    }
                }
            }
        }
        // print_map_2(&wall, &boxes, robot, (len_x, len_y));
        let mut sum = 0;
        for (bx, by) in boxes {
            sum += (by * 100 + bx) as i32;
        }
        Ok(sum)
    }

    #[allow(dead_code)]
    fn print_map_2(
        wall: &HashSet<(usize, usize)>,
        boxes: &HashSet<(usize, usize)>,
        robot: (usize, usize),
        (len_x, len_y): (usize, usize),
    ) {
        let mut sum_box = 0;
        for j in 0..len_y {
            for i in 0..len_x {
                if robot == (i, j) {
                    print!("@");
                } else if wall.contains(&(i, j)) {
                    print!("#");
                } else if boxes.contains(&(i, j)) {
                    print!("[");
                    sum_box += 1;
                } else if boxes.contains(&(i.wrapping_sub(1), j)) {
                    print!("]");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!("total box: {sum_box}");
    }

    assert_eq!(9021, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
