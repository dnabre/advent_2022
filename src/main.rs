#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::collections::{HashMap, VecDeque};
use std::collections::HashSet;
use std::{cmp, fmt};
use std::fmt::{Display, Formatter};
use std::fs;
use std::time::{Instant,Duration};





/*
    Advent of Code 2022: Day 14
        part1 answer:
        part2 answer:


 */

const TEST_ANSWER: (i64, i64) = (64, 58);
const INPUT_ANSWER: (i64, i64) = (4282, 2452);

const PART1_TEST_FILENAME: &str = "data/day14/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day14/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day14/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day14/part2_input.txt";

const TEST: bool = true;

fn main() {
    // print!("Advent of Code 2022, Day ");
    // println!("14");                           // insert Day


    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();

    // println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);
    // //
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

    // println!("----------\ndone");
}

#[cfg(windows)]
const D_LINE_ENDING: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const D_LINE_ENDING: &'static str = "\n\n";

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";


fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };
    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let mut lines: Vec<&str> = data1_s.trim().split("\n").collect();
    let l_num = lines.len();
    // if TEST {
    //     println!("\t read {} lines from {}", l_num, p1_file);
    //     if l_num == 1 {
    //         println!("\t\t line read has length: {}", lines[0].len());
    //     }
    // }
    // let data1_ss = data1_s.trim();
    // let split_lines:Vec<&str> = data1_ss.split(LINE_ENDING).collect();
    // for l in split_lines {
    //     println!("{l}");
    // }

    let p1_file = "in.info";
    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let mut lines: Vec<&str> = data1_s.trim().split("\n").collect();
    let l_num = lines.len();
    //println!("lines = {}", l_num);


    for whole in lines {
        if whole.starts_with("BookmarkPageNumber")  ||
            whole.starts_with("NumberOfPages") ||
            whole.starts_with("PageMediaNumber")
            {
            let ps = whole.split_once(":").unwrap();
            let (l,r) = ps;
            let n:i32 = r.trim().parse().unwrap();
            let n2 = n-1;


        //    println!("{whole}");
            println!("{l}: {n2}");


         //   println!("{:?} n={n}",ps );
        }
        else {
            println!("{whole}");
        }
    }



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
    let split_lines:Vec<&str> = data2_ss.split(LINE_ENDING).collect();

    for l in split_lines {
        println!("{l}");
    }

    let answer2 = String::new();


    return answer2;
}
