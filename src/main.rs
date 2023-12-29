#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::fmt::{Display, Formatter};
use std::time::Instant;

use advent_2022::{Direction, LeftOrRight, print_grid, print_grid_window};

use crate::Tile::OffMap;

/*
    Advent of Code 2022: Day 22
        part1 answer:   155060
        part2 answer:   3479

*/
const ANSWER: (&str, &str) = ("155060", "3479");

fn main() {
    let _filename_test = "data/day22/test_input_01.txt";
    let _filename_test2 = "data/day22/test_input_02.txt";

    let filename_part1 = "data/day22/part1_input.txt";
    let filename_part2 = "data/day22/part2_input.txt";


    println!("Advent of Code, Day 22");
    println!("    ---------------------------------------------");
    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!(
            "\t\t ERROR: Answer is WRONG. Got: {}, Expected {}",
            answer1, ANSWER.0
        );
    }

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    if ANSWER.1 != answer2 {
        println!(
            "\t\t ERROR: Answer is WRONG. Got: {}, Expected {}",
            answer2, ANSWER.1
        );
    }
    println!("    ---------------------------------------------");
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Tile {
    Open,
    Block,
    OffMap,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tile::Open => { '.' }
            Tile::Block => { '#' }
            Tile::OffMap => { ' ' }
        })
    }
}



#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Code {
    Turn(LeftOrRight),
    Forward(usize),
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            Code::Turn(a) => { ("Turn", (*a).to_string()) }
            Code::Forward(n) => { ("Forword", (*n).to_string()) }
        };
        write!(f, "{}:{}", t.0, t.1)
    }
}


fn parse_flatgrid_and_code(mut lines: &mut Vec<String>) -> (Vec<Vec<char>>, Vec<Vec<Tile>>, Vec<Code>) {
    let mut grid_lines: Vec<String> = Vec::new();
    let mut instruction_lines = None;
    let mut max_grid_line_length: usize = usize::MIN;
    for i in 0..lines.len() - 2 {
        let len = lines[i].len();
        max_grid_line_length = max_grid_line_length.max(len);
    }
    let mut index = 0;
    while index < lines.len() {
        let lin = &mut lines[index];
        if lin.is_empty() {
            instruction_lines = Some(lines[index + 1].clone());
            break;
        }
        if lin.len() < max_grid_line_length {
            let diff = max_grid_line_length - lin.len();
            let blanks_to_add = std::iter::repeat(" ").take(diff).collect::<String>();
            lin.push_str(&blanks_to_add);
        }
        grid_lines.push(lines[index].clone());
        index += 1;
    }
    let grid = advent_2022::parse_grid(&grid_lines);
    let mut c_grid = grid.clone();
    let grid = advent_2022::convert_grid_using(&grid, |ch| match ch {
        '.' => { Tile::Open }
        '#' => { Tile::Block }
        ' ' => { Tile::OffMap }
        _ => { panic!("character for map tile unknown: {}", ch) }
    });
    let instructions;
    if let Some(s_instructions) = instruction_lines {
        instructions = s_instructions;
    } else {
        panic!("no instruction line;")
    }
    let codes: Vec<Code> = parse_codes(instructions);
    (c_grid, grid, codes)
}

fn parse_codes(input: String) -> Vec<Code> {
    let mut codes: Vec<Code> = Vec::new();
    let array = advent_2022::str_to_char_vec(input.as_str());
    let mut digits: Vec<usize> = Vec::new();
    for c in array {
        if c.is_numeric() {
            let digit: usize = c.to_digit(10).unwrap() as usize;
            digits.push(digit);
        } else {
            let num: usize = digits.iter().fold(0, |num, digit| num * 10usize + digit);
            digits.clear();
            codes.push(Code::Forward(num));
            let l_of_r = match c {
                'L' => LeftOrRight::Left,
                'R' => LeftOrRight::Right,
                x => panic!("not left or right: {}", x)
            };
            codes.push(Code::Turn(l_of_r));
        }
    }
    if !digits.is_empty() {
        let num = digits.iter().fold(0, |num, digit| num * 10 + digit);
        digits.clear();
        codes.push(Code::Forward(num));
    }
    return codes;
}



fn part1(input_file: &str) -> String {
    let mut lines = advent_2022::file_to_lines(input_file);


    let (_,grid, codes) = parse_flatgrid_and_code(&mut lines);

    let max_row = grid.len();
    let max_col = grid[0].len();

    let mut pos: (usize, usize) = (0, 0);
    let mut dir = Direction::Right;
    loop {
        let ch = grid[pos.0][pos.1];
        if ch != Tile::Open {
            pos.1 += 1;
        } else {
            break;
        }
    }

     for c in codes {
        match c {
            Code::Turn(l_or_r) => {
                dir = dir.turn_to(l_or_r);
            }
            Code::Forward(much) => {
                let mut steps = much;
                let mut n_pos = dir.grid_go_in_dir_rc(pos, max_row, max_col);
                let mut wrapped_n_pos = None;
                while steps > 0 {
                    if n_pos.is_none() || (grid[n_pos.unwrap().0][n_pos.unwrap().1] == OffMap) {
                        // Either ran off grid or fit a place on the grid that is explicitly Off the Map.
                        // Either way, we wrap:
                        match dir {
                            Direction::Up => {
                                let mut wrap_r = max_row - 1;
                                let c = pos.1;
                                while grid[wrap_r][c] == Tile::OffMap {
                                    wrap_r -= 1;
                                }
                                n_pos = Some((wrap_r, c));
                            }
                            Direction::Down => {
                                let mut wrap_r = 0;
                                let c = pos.1;
                                while grid[wrap_r][c] == Tile::OffMap {
                                    wrap_r += 1;
                                }
                                n_pos = Some((wrap_r, c))
                            }
                            Direction::Left => {
                                let mut wrap_c = max_col - 1;
                                let r = pos.0;
                                while grid[r][wrap_c] == Tile::OffMap {
                                    wrap_c -= 1;
                                }
                                n_pos = Some((r, wrap_c))
                            }
                            Direction::Right => {
                                let mut wrap_c = 0;
                                let r = pos.0;
                                while grid[r][wrap_c] == Tile::OffMap {
                                    wrap_c += 1;
                                }
                                n_pos = Some((r, wrap_c))
                            }
                        }
                        wrapped_n_pos = n_pos;
                    };
                    n_pos = match wrapped_n_pos {
                        None => { n_pos }
                        Some(nw_pos) => {
                            wrapped_n_pos = None;
                            Some(nw_pos)
                        }
                    };
                    if let Some((n_r,n_c)) = n_pos {
                        let ch = grid[n_r][n_c];
                        match ch {
                            Tile::Open => {
                                pos = (n_r,n_c);
                                n_pos = dir.grid_go_in_dir_rc(pos, max_row, max_col);
                                steps -= 1;
                            }
                            Tile::Block => {
                                // We are blocked. Abort any steps left over
                                break;
                            }
                            OffMap => {panic!("should have fixed this position to be on map")}
                        }
                    } else {
                        panic!("n_pos should have a value at this point");
                    }
                }
            }
        }
    }
    let answer = (pos.0 + 1) * 1000 + 4 * (pos.1 + 1) + get_facing(dir);
    return answer.to_string();
}

const PART2_SIDE_SIZE:(usize,usize)=(4,50);

fn part2(input_file: &str) -> String {
    let mut lines = advent_2022::file_to_lines(input_file);
    println!("read {} lines", lines.len());
    let (mut c_grid, grid, codes) = parse_flatgrid_and_code(&mut lines);

    let mut start_pos = (0,0);
    while grid[start_pos.0][start_pos.1] !=  Tile::Open {
        start_pos.1 += 1;
    }
    println!("start position: {:?}", start_pos);
    let mut pos = start_pos;
    let mut dir = Direction::Right;

    let max_row = PART2_SIDE_SIZE.1;
    let max_col = PART2_SIDE_SIZE.1;

    for c in codes {
        match c {
            Code::Turn(l_or_r) => {
                dir = dir.turn_to(l_or_r);
            }
            Code::Forward(much) => {
                c_grid[pos.0][pos.1] = dir.to_arrow();
                let mut steps = much;
                let mut n_pos = grid_go_in_dir_mrc(dir,pos);
                while steps > 0 {
                    c_grid[pos.0][pos.1] = dir.to_arrow();
                    let mut n_pos = grid_go_in_dir_mrc(dir,pos);
                 //   println!("n_pos: {:?}", n_pos);
                    if let (Some((n_r, n_c)), _ ) = n_pos {
                        let ch = grid.get(n_r).and_then(|row| row.get(n_c)).unwrap_or(&Tile::OffMap);
                        match ch {
                            Tile::Open => {
                                pos =(n_r,n_c);

                            }
                            Tile::Block => {
                                break;
                            }
                            OffMap => {
                                let (new_pos, new_dir) = wrap2(&pos, &dir);

                                if grid[new_pos.0][new_pos.1] == Tile::Block {
                                    break;
                                }
                                pos = new_pos;
                                dir = new_dir;

                            }
                        }
                    } else {
                        let off = n_pos.1;

                        let (new_pos, new_dir) = wrap2(&pos, &dir);
                        if grid[new_pos.0][new_pos.1] == Tile::Block {
                            break;
                        }
                        pos = new_pos;
                        dir = new_dir;

                    }
                    steps = steps - 1;




                }
            }
        }
    }

    println!("final pos: {}@{:?}", dir.to_arrow(), pos);
    let answer = (pos.0 + 1) * 1000 + 4 * (pos.1 + 1) + get_facing(dir);
    return answer.to_string();

    //correct answer is 3479 with final pos (2,118)
}
fn wrap2(pos: &(usize,usize), dir: &Direction) -> ((usize,usize), Direction) {
    // find idxes of entire cube
    // this huge match statement only covers cases in the real input, but can be expanded to cover everything. It's just tedious
    let (cube_row, cube_col, new_dir) = match (pos.0 / 50, pos.1 / 50, dir) {
        (0, 1, Direction::Up) => (3, 0, Direction::Right),
        (0, 1, Direction::Left) => (2, 0, Direction::Right),
        (0, 2, Direction::Up) => (3, 0, Direction::Up),
        (0, 2, Direction::Right) => (2, 1, Direction::Left),
        (0, 2, Direction::Down) => (1, 1, Direction::Left),
        (1, 1, Direction::Right) => (0, 2, Direction::Up),
        (1, 1, Direction::Left) => (2, 0, Direction::Down),
        (2, 0, Direction::Up) => (1, 1, Direction::Right),
        (2, 0, Direction::Left) => (0, 1, Direction::Right),
        (2, 1, Direction::Right) => (0, 2, Direction::Left),
        (2, 1, Direction::Down) => (3, 0, Direction::Left),
        (3, 0, Direction::Right) => (2, 1, Direction::Up),
        (3, 0, Direction::Down) => (0, 2, Direction::Down),
        (3, 0, Direction::Left) => (0, 1, Direction::Down),
        _ => unreachable!(),
    };
    // find idxes within the cube
    let (row_idx, col_idx) = (pos.0 % 50, pos.1 % 50);

    let i = match dir {
        Direction::Left => 49 - row_idx,
        Direction::Right => row_idx,
        Direction::Up => col_idx,
        Direction::Down => 49 - col_idx,
    };

    // find new idxes within the cube
    let new_row = match new_dir {
        Direction::Left => 49 - i,
        Direction::Right => i,
        Direction::Up => 49,
        Direction::Down => 0,
    };
    let new_col = match new_dir {
        Direction::Left => 49,
        Direction::Right => 0,
        Direction::Up => i,
        Direction::Down => 49 - i,
    };

    let new_pos = (
        cube_row * 50 + new_row,
        cube_col * 50 + new_col,
    );

    (new_pos, new_dir)
}

pub fn grid_go_in_dir_mrc(dir:Direction, (r, c):(usize, usize)) -> (Option<(usize, usize)>, (i32,i32)) {
    match dir {
        Direction::Up => {
            if r > 0 {
(Some((r - 1, c)), (0,0))
            } else {


            (None,(-1, 0))

            }
        }
        Direction::Down => {

(Some((r + 1, c)), (0,0))


        }

        Direction::Left => {
            if c > 0 {
                (Some((r, c - 1)),(0,0))
            } else {
(None, (0,-1))
            }
        }
        Direction::Right => {

(Some((r, c + 1)), (0,0))


        }
    }
}

fn get_facing(dir:Direction) -> usize {
    match dir {
        Direction::Up => {3}
        Direction::Down => {1}
        Direction::Left => {2}
        Direction::Right => {0}
    }
}