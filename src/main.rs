#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::fmt::{Display, Formatter};
use std::time::Instant;

use advent_2022::{Direction, LeftOrRight, print_grid};

use crate::Tile1::OffMap;

/*
    Advent of Code 2022: Day 22
        part1 answer:   155060
        part2 answer:

*/
const ANSWER: (&str, &str) = ("155060", "Button Pressed");

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
enum Tile1 {
    Open,
    Block,
    OffMap,
}

impl Display for Tile1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tile1::Open => { '.' }
            Tile1::Block => { '#' }
            Tile1::OffMap => { ' ' }
        })
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Tile {
    Open,
    Block,
    Null
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tile::Open => { '.' }
            Tile::Block => { '#' }
            Tile::Null => { '?' }
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


fn parse_flatgrid_and_code(mut lines: &mut Vec<String>) -> (Vec<Vec<char>>, Vec<Vec<Tile1>>, Vec<Code>) {
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
        '.' => { Tile1::Open }
        '#' => { Tile1::Block }
        ' ' => { Tile1::OffMap }
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
        if ch != Tile1::Open {
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
                                while grid[wrap_r][c] == Tile1::OffMap {
                                    wrap_r -= 1;
                                }
                                n_pos = Some((wrap_r, c));
                            }
                            Direction::Down => {
                                let mut wrap_r = 0;
                                let c = pos.1;
                                while grid[wrap_r][c] == Tile1::OffMap {
                                    wrap_r += 1;
                                }
                                n_pos = Some((wrap_r, c))
                            }
                            Direction::Left => {
                                let mut wrap_c = max_col - 1;
                                let r = pos.0;
                                while grid[r][wrap_c] == Tile1::OffMap {
                                    wrap_c -= 1;
                                }
                                n_pos = Some((r, wrap_c))
                            }
                            Direction::Right => {
                                let mut wrap_c = 0;
                                let r = pos.0;
                                while grid[r][wrap_c] == Tile1::OffMap {
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
                            Tile1::Open => {
                                pos = (n_r,n_c);
                                n_pos = dir.grid_go_in_dir_rc(pos, max_row, max_col);
                                steps -= 1;
                            }
                            Tile1::Block => {
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
    let answer = (pos.0 + 1) * 1000 + 4 * (pos.1 + 1);
    return answer.to_string();
}

const PART2_SIDE_SIZE:(usize,usize)=(5,50);

fn part2(input_file: &str) -> String {
    let mut lines = advent_2022::file_to_lines(input_file);
    println!("read {} lines", lines.len());
    let (_, grid, codes) = parse_flatgrid_and_code(&mut lines);
    let grid = advent_2022::convert_grid_using(&grid, |t| match t {
        Tile1::Open => {Tile::Open }
        Tile1::Block => {Tile::Block}
        Tile1::OffMap => {Tile::Null}
    });
    //nulls shouldbe removed in the end



    let (piece_map, chunk_list):(Vec<Vec<Option<char>>>, Vec<Vec<Vec<Tile>>>) = split_grid_into_faces(grid, PART2_SIDE_SIZE.1);

    let sq = PART2_SIDE_SIZE.0;




    let answer = 0;
    return answer.to_string();
}

fn split_grid_into_faces(grid: Vec<Vec<Tile>>, sq_size: usize) -> (Vec<Vec<Option<char>>>, Vec<Vec<Vec<Tile>>>) {
    let mut map_vec:Vec<Vec<Option<char>>> =  Vec::new();
    let mut grid_list:Vec<Vec<Vec<Tile>>> = Vec::new();


    let sq_high = grid.len() / sq_size;
    let sq_wide = grid[0].len() /  sq_size;
    println!("grid {} high by {} wide", sq_high, sq_wide);
    for h in 0..sq_high {
            let mut line_vec = Vec::new();
            for w in 0..sq_wide {
                line_vec.push(None);
            }
        map_vec.push(line_vec);
    }

    //print_grid(&map_vec);
   advent_2022::print_grid_with_displayer(&map_vec, |o_ch| format!("{:?}", o_ch));

    let mut r_offset = 0;
    for h in 0..sq_high {
        let mut chunk_list = Vec::new();
        let mut c_offset = 0;
        for w in 0..sq_wide {



        }
        grid_list.push(chunk_list);
    }



    return (map_vec,grid_list);
}

