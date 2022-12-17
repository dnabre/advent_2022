#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::collections::{HashMap, VecDeque};
use std::collections::HashSet;
use std::{cmp, fmt};
use std::fmt::Formatter;
use std::fs;
use std::time::Instant;

use parse_display::FromStr;


/*
    Advent of Code 2022: Day 17
        part1 answer:
        part2 answer:

        part1 -> 3158 is too high

 */

const TEST_ANSWER: (i64, i64) = (3068, 3059);
const INPUT_ANSWER: (i64, i64) = (3158, 1500874635587);


const PART1_TEST_FILENAME: &str = "data/day17/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day17/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day17/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day17/part2_input.txt";

const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("17");                           // insert Day


    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();

    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);
    //
    // if TEST {
    //     assert_eq!(answer1, TEST_ANSWER.0.to_string());
    // } else {
    //     assert_eq!(answer1, INPUT_ANSWER.0.to_string());
    // }
    //
    // let start2 = Instant::now();
    // let answer2 = part2();
    // let duration2 = start2.elapsed();
    //
    // println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);

    // if TEST {
    //     assert_eq!(answer2, TEST_ANSWER.1.to_string());
    // } else {
    //     assert_eq!(answer2, INPUT_ANSWER.1.to_string());
    // }

    println!("----------\ndone");
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Block {
    HLine,
    Cross,
    Corner,
    VLine,
    Square,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Block::HLine => { write!(f, "hort-bar") }
            Block::Cross => { write!(f, "cross") }
            Block::Corner => { write!(f, "corner") }
            Block::VLine => { write!(f, "vert-bar") }
            Block::Square => { write!(f, "square") }
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::Up => write!(f, "🠕"),
            Direction::Down => write!(f, "🠗"),
            Direction::Left => write!(f, "🠔"),
            Direction::Right => write!(f, "➝"),
        }
    }
}


fn can_move(field: &[u8], r: Block, dir: Direction, row: usize, col: usize) -> bool {
    match dir {
        Direction::Up => {
            unreachable!()
        }

        Direction::Down => {
            0 < row
                && match r {
                Block::HLine => {
                    field[row - 1] & (0xF << (7 - col - 3)) == 0
                }
                Block::Cross => {
                    field[row - 1] & (1 << (7 - col - 1)) | field[row] &
                        (0x5 << (7 - col - 2)) == 0
                }
                Block::Corner => {
                    field[row - 1] & (0x7 << (7 - col - 2)) == 0
                }
                Block::VLine => {
                    field[row - 1] & (1 << (7 - col)) == 0
                }
                Block::Square => {
                    field[row - 1] & (0x3 << (7 - col - 1)) == 0
                }
            }
        }
        Direction::Left => {
            0 < col &&
                match r {
                    Block::HLine => {
                        field[row] & (1 << (7 - col + 1)) == 0
                    }
                    Block::Cross => {
                        (field[row] | field[row + 2])
                            & (1 << (7 - col + 1))
                            | field[row + 1]
                            & (1 << (7 - col + 1))
                            == 0
                    }
                    Block::Corner => {
                        field[row] & (1 << (7 - col + 1)) |
                            (field[row + 1] | field[row + 2]) &
                                (1 << (7 - col - 1)) == 0
                    }
                    Block::VLine => {
                        (field[row] | field[row + 1] | field[row + 2] | field[row + 3])
                            & (1 << (7 - col + 1))
                            == 0
                    }
                    Block::Square => {
                        (field[row] | field[row + 1]) & (1 << (7 - col + 1)) == 0
                    }
                }
        }
        Direction::Right => {
            match r {
                Block::HLine => col + 4 < 7 && field[row] & (1 << (7 - col - 4)) == 0,
                Block::Cross => {
                    col + 3 < 7
                        && (field[row] | field[row + 2]) & (1 << (7 - col - 2))
                        | field[row + 1] & (1 << (7 - col - 3))
                        == 0
                }
                Block::Corner => {
                    col + 3 < 7
                        && (field[row] | field[row + 1] | field[row + 2]) & (1 << (7 - col - 3)) == 0
                }
                Block::VLine => {
                    col + 1 < 7
                        && (field[row] | field[row + 1] | field[row + 2] | field[row + 3])
                        & (1 << (7 - col - 1))
                        == 0
                }
                Block::Square => {
                    col + 2 < 7 && (field[row] | field[row + 1]) & (1 << (7 - col - 2)) == 0
                }
            }
        }
    }
}


fn freeze(field:&mut [u8], r: Block, row: usize, col: usize) -> usize {
    match r {
        Block::HLine => {
            field[row] |= 0xF << (7 - col - 3);
            row
        }
        Block::Cross => {
            field[row] |= 1 << (7 - col - 1);
            field[row + 1] |= 0x7 << (7 - col - 2);
            field[row + 2] |= 1 << (7 - col - 1);
            row + 2
        }
        Block::Corner => {
            field[row] |= 0x7 << (7 - col - 2);
            field[row + 1] |= 1 << (7 - col - 2);
            field[row + 2] |= 1 << (7 - col - 2);
            row + 2
        }
        Block::VLine => {
            field[row] |= 1 << (7 - col);
            field[row + 1] |= 1 << (7 - col);
            field[row + 2] |= 1 << (7 - col);
            field[row + 3] |= 1 << (7 - col);
            row + 3
        }
        Block::Square => {
            field[row] |= 3 << (7 - col - 1);
            field[row + 1] |= 3 << (7 - col - 1);
            row + 1
        }
    }
}





fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };
    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let mut lines: Vec<&str> = data1_s.trim().split("\n").collect();
    let l_num = lines.len();
    if !TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }

    let mut field = vec![0u8; 10];
    let mut highest_row = 0;
    let jets = lines[0].trim_end().as_bytes();
    let mut jet_index = 0;


    let last_turn: usize = 50;

    let mut more_debug;
    'block: for i in 0..last_turn {
        println!("top of block loop, turn {i}, {highest_row}");
        if i ==  27 {
            more_debug = true;
        } else {
            more_debug = false;
        }

        let mut bottom = highest_row + 3;
        for _ in field.len()..bottom + 4 {
            field.push(0);
        }
        let mut left = 2;
        let r = match i % 5 {
            0 => Block::HLine,
            1 => Block::Cross,
            2 => Block::Corner,
            3 => Block::VLine,
            4 => Block::Square,
            _ => unreachable!(),
        };
        if more_debug {
            println!("\tblock, r: {:?}", r);
            println!("\tfield: {:?}", field);
            println!("\tbottom: {bottom}");
            println!("\thigh_row: {highest_row}");
            println!("\tjet_index: {jet_index}");
        }
        bottom -= 3;
        for _ in 0..3 {
            match jets[jet_index % jets.len()] {
                60 => { //left
                    if can_move(&field, r, Direction::Left, bottom, left) {
                        left -= 1;
                    }
                }
                62 => { //right
                    if can_move(&field, r, Direction::Right, bottom, left) {
                        left += 1;
                    }
                }
                _ => unreachable!(),
            }
            jet_index += 1;
        }

        loop {
            match jets[jet_index % jets.len()] {
                60 => { //left
                    if can_move(&field, r, Direction::Left, bottom, left) {
                        left -= 1
                    }
                }
                62 => { //right
                    if can_move(&field, r, Direction::Right, bottom, left) {
                        left += 1
                    }
                }
                _ => unreachable!(),
            }
            jet_index += 1;

            let dir = Direction::Down;
            if !can_move(&field, r, dir, bottom, left) {
                let row = freeze(&mut field, r, bottom, left);
           //     highest_row = cmp::max(row + 1, highest_row);
                if row +1 > highest_row {
                    highest_row = row + 1
                }

                continue 'block;
            } else {
                bottom -= 1;
            }
        }
    }

    let answer1 = highest_row as u32;
    return answer1.to_string();
}

fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));

    let mut lines: Vec<&str> = data2_s.trim().split("\n").collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
    }

    let mut answer2 = String::new();
    return answer2;
}
