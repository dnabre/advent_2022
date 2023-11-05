#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]




use std::collections::{HashMap, VecDeque};
use std::collections::HashSet;
use std::fmt::{Display,Formatter};
use std::{cmp, fmt};
use std::fs;
use std::time::{Instant, Duration};

use enum_display_derive::Display as Derived_Display;

/*
    Advent of Code 2022: Day 23
        part1 answer:
        part2 answer:


 */


const TEST_ANSWER: (i64, i64) = (110, 93);
const INPUT_ANSWER: (i64, i64) = (843, 27625);

const PART1_TEST_FILENAME: &str = "data/day23/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day23/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day23/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day23/part2_input.txt";

const TEST: bool = true;




fn main() {
    print!("Advent of Code 2022, Day ");
    println!("23");                           // insert Day


    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();

    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);
    if TEST {
        if answer1 != TEST_ANSWER.0.to_string() {
            println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
                     answer1, TEST_ANSWER.0.to_string())
        }
    } else {
        if answer1 != INPUT_ANSWER.0.to_string() {
            println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
                     answer1, INPUT_ANSWER.0.to_string())
        }
    }

    let start2 = Instant::now();
    let answer2 = part2();
    let duration2 = start2.elapsed();

    // println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);
    //
    //
    // if TEST {
    //     if answer2 != TEST_ANSWER.1.to_string() {
    //         println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
    //                  answer2,TEST_ANSWER.1.to_string() )
    //     }
    // } else {
    //     if answer2 != INPUT_ANSWER.1.to_string() {
    //         println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
    //                  answer2,TEST_ANSWER.1.to_string() )
    //     }
    // }


    println!("----------\ndone");
}

#[cfg(windows)]
const D_LINE_ENDING: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const D_LINE_ENDING: &'static str = "\n\n";

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash )]
struct Coord {
    row: i32,
    col: i32,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.col, self.row)
    }
}

#[derive(Hash, PartialEq, Eq, Derived_Display)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn print_set<T:Display>(set:HashSet<T>) {
    let mut c =0;
    let l = set.len();

    let v:Vec<T> = set.into_iter().collect();

    let Some((last, elements)) = v.split_last()
    else {
        panic!("split_last on vector of hashset wonky");
    };
    print!("[");
    for elem in elements {
        print!("{}, ", elem);
    }
    println!("{}]", last);

}


fn rotate_elf_rules(mut rules:VecDeque<Direction>) {
    let top = rules.pop_front().unwrap();
    rules.push_back(top);

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
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }
    let data1_ss = data1_s.trim();
    let split_lines: Vec<&str> = data1_ss.split(LINE_ENDING).collect();

    let mut c_col;
    let mut c_row;

    let mut coords:HashSet<Coord> = HashSet::new();

    c_row = 1;
    for l in split_lines {
        c_col = 1;
        for c in l.chars() {
            if c == '#' {
                coords.insert(Coord{ row: c_row, col: c_col});
            } c_col += 1;
        } c_row += 1;
    }

//    println!("coords: {:?}",coords);
print_set(coords);
    let mut elf_rules:VecDeque<Direction> = VecDeque::with_capacity(4);
    elf_rules.push_back(Direction::North);
    elf_rules.push_back(Direction::South);
    elf_rules.push_back(Direction::West);
    elf_rules.push_back(Direction::East);

    rotate_elf_rules(elf_rules);


    println!("\n\n\n done");
    let answer1 = String::new();
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
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }
    let data2_ss = data2_s.trim();
    let split_lines: Vec<&str> = data2_ss.split(LINE_ENDING).collect();


    let answer2 = String::new();
    return answer2.to_string();
}
