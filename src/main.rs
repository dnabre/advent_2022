#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]


use std::fmt::{Display, Formatter};
use std::fs;
use std::time::Instant;

use ndarray::{Array, Array2, ArrayBase, OwnedRepr, Dim};

/*
    Advent of Code 2022: Day 24`
        part1 answer:
        part2 answer:


 */


const TEST_ANSWER: (i64, i64) = (20, 110);
const INPUT_ANSWER: (i64, i64) = (4195, 1069);

const PART1_TEST_FILENAME: &str = "data/day24/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day24/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day24/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day24/part2_input.txt";

const TEST: bool = true;


fn main() {
    print!("Advent of Code 2022, Day ");
    println!("24");                           // insert Day


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

    // let start2 = Instant::now();
    // let answer2 = part2();
    // let duration2 = start2.elapsed();

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
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",
        match self {
            Direction::Up => {"^"}
            Direction::Down => {"v"}
            Direction::Left => {"<"}
            Direction::Right => {">"}
        })
    }
}

struct Blizzard {
    row:usize,
    col:usize,
    dir:Direction
}


fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };
    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let lines: Vec<&str> = data1_s.trim().split("\n").collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }
    let data1_ss = data1_s.trim();
    let split_lines: Vec<&str> = data1_ss.split(LINE_ENDING).collect();
    let dims@(rows,cols) = (split_lines.len(), split_lines[0].len());
    println!("dims: {:?}", dims);


   let mut char_buffer = String::new();
    for l in split_lines {
        char_buffer.push_str(l);
    }
    let  v:Vec<char> = char_buffer.chars().collect();
    let v2:Vec<char>  = v[rows..(v.len() - cols)].to_vec();

    let mut v3 = v2.clone();
    v3.retain_mut(|c| *c != '#');



   let init_grid =
       Array::from_shape_vec((rows-2,cols-2), v3.to_vec()).unwrap();
    println!("{:?}\n", init_grid);

    println!("init_grid.dim().0 = {}", init_grid.dim().0); // # rows
    println!("init_grid.dim().1 = {}", init_grid.dim().1); // # cols
println!();
    //let mut blizzards:Vec<Blizzard> = Vec::new();
    for row in 0..init_grid.dim().0 {
        for col in 0..init_grid.dim().1 {
            let c = init_grid[[row,col]];
            print!("{}", c);
        }
        println!();
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
    let lines: Vec<&str> = data2_s.trim().split("\n").collect();
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
