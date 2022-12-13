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
use aho_corasick::AhoCorasick;

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

const TEST: bool = true;

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


fn parse_line(lline: &mut String) -> Vec<i32> {
    let mut l_vec: Vec<i32> = Vec::new();
    let mut sb: String = String::from("0");

    let mut ch: char;
    for ch in lline.chars() {
        if ch == '[' { continue; }
        if ch == ']' { continue; }
        if ch.is_digit(10) {
            sb.push(ch);
        }
        if ch == ',' {
            if sb.len() > 1 {
                let n: i32 = sb.parse().unwrap();
                l_vec.push(n);
                sb = String::from("0");
            }
        }
    }
    if sb.len() > 1 {
        let n: i32 = sb.parse().unwrap();
        l_vec.push(n);
        sb = String::from("0");
    }
    return l_vec;
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

    let mut left: Vec<String> = Vec::new();
    let mut right: Vec<String> = Vec::new();

    println!("{}", lines[0]);
    println!("{}", lines[1]);

    for i in 0..lines.len() {
        let mut ln = lines[i].to_string();
        if left.len() == right.len() {
            if ln.len() == 0 {
                continue;
            } else {
                left.push(ln.trim().parse().unwrap());
                continue;
            }
        }
        if left.len() > right.len() {
            right.push(ln.trim().parse().unwrap());
            continue;
        }
        panic!("more right lists than left lists, input {}", ln);
    }
    assert_eq!(left.len(), right.len());
    let left = left;
    let right = right;
    let pair_count = left.len();
    println!("number of pairs: {pair_count}");

    let mut lline = left[0].clone();
    let mut rline = right[0].clone();

    for i in 0..pair_count {
        println!("== Pair {:2>} ==", i+1);
        let mut s_left = left[i].clone();
        let mut s_right = right[i].clone();
        let mut l_vec = parse_line(&mut s_left);
        let mut r_vec = parse_line(&mut s_right);

	let check = true;
        //let check:bool = check_vecs(l_vec.clone(), r_vec.clone());


        // println!("\tCompare {} vs {}", lline, rline);
        // println!("\t\"{}\" -> {:?}", s_left, l_vec);
        // println!("\t\"{}\" -> {:?}", s_right, r_vec);
         println!("\t  check: {}", match check {
            true => {"Ok"}
            false => {"bad"}
        });
        println!("--------------");
    }







    let mut indices: Vec<i32> = Vec::new();
    let sum: i32 = indices.iter().sum();

    let mut answer1 = sum.to_string();
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
