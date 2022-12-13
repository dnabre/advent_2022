#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fs;
use std::time::Instant;
use parse_display::FromStr;

/*
    Advent of Code 2022: Day 13
        part1 answer:
        part2 answer:

 */


const TEST_ANSWER: (i32, i32) = (0, 0);
const INPUT_ANSWER: (i32, i32) = (0, 0);


const PART1_TEST_FILENAME: &str = "data/day13/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day13/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day13/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day13/part2_input.txt";

const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("13");

    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();

    // println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);
    // if TEST {
    //     assert_eq!(answer1, TEST_ANSWER.0.to_string());
    // } else {
    //     assert_eq!(answer1, INPUT_ANSWER.0.to_string());
    // }

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
    //
    // println!("----------\ndone");
}



fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };

    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let mut lines: Vec<&str> = data1_s.trim().split("\n").map(|t| t.trim()).collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
    }

    let mut answer1 = String::new();
    return answer1;
}


fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));

    let mut lines: Vec<&str> = data2_s.trim().split("\n").map(|t| t.trim()).collect();
    let l_num = lines.len();

    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
    }
    let mut answer2 = String::new();
    return answer2;
}
