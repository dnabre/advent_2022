#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::fs;
use std::collections::HashMap;
use std::time::Instant;
use std::iter;
use std::ops::{Add, Index};
use std::fmt;
use std::path::PathBuf;
use std::thread::current;
use parse_display::FromStr;

/*
    Advent of Code 2022: Day 07
        part1 answer:
        part2 answer:
 */


#[cfg(windows)]
const D_LINE_ENDING: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const D_LINE_ENDING: &'static str = "\n\n";


const TEST: bool = true;

const PART1_TEST_FILENAME: &str = "data/day07/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day07/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day07/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day07/part2_input.txt";

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("07");                           // insert Day

    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();
    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);

    // let start2 = Instant::now();
    // let answer2 = part2();
    // let duration2 = start2.elapsed();
    // println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);

    println!("----------\ndone");
}


#[derive(FromStr, Debug)]
#[display("{size} {name}")]
struct File {
    name: String,
    size: usize,
}


#[derive(FromStr, Debug)]
#[display("dir {name}")]
struct Dir {
    name: String,
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
    }

    for i in 0..lines.len() {
        lines[i] = lines[i].trim();
    }


    let mut file_size:HashMap<PathBuf,u32> = HashMap::new();

    let mut path = PathBuf::from("/");



    println!("path: {:?}", path);
    path.push("subdir");
    println!("path: {:?}", path);


    let pwd = "<path>";

    // Assume first command will be "cd /"
    for line in lines {
        if line.starts_with("$") {
            //command
            if line.starts_with("$ ls") {
                println!("listing  : {pwd}");
            } else {  // change directory command
                let (_, dir) = line.split_once("cd ").unwrap();
                print!("commmand : cd from {pwd} to ");
                match dir {
                    "/" => {
                        println!("-:\t cd /");
                    }
                    ".." => {
                        println!("-:\t cd ..");
                    }
                    d_name => {
                        println!("-:\t cd {d_name}");
                    }
                }
            }
            //   println!("\t\t post: {:?}", path);

            //end command
        } else {
            //listing
            if line.starts_with("dir") {
                let d: Dir = line.parse().unwrap();
                println!("list  dir: {pwd}/{}",d.name );
            } else {
                let f: File = line.parse().unwrap();
                println!("list file: {pwd}/{} size: {}",f.name, f.size);

            }
        }
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


    let lines: Vec<&str> = data2_s.trim().split("\n").collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
    }

    let mut answer2 = String::new();
    return answer2;
}
