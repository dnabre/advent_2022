#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::fs;
use std::collections::HashSet;
use std::time::Instant;
use std::iter;
use std::ops::{Add, Index};
use std::fmt;
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
#[from_str(default)]
struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Dir>,
    size: usize,
}

impl Default for Dir {
    fn default() -> Dir {
        Dir {
            name: String::from("undefined"),
            files: Vec::new(),
            dirs: Vec::new(),
            size: 0,
        }
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[dir: {} f#: {} d#: {}]", self.name, self.files.len(), self.dirs.len())
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
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
    }

    for i in 0..lines.len() {
        lines[i] = lines[i].trim();
    }

    let mut files: Vec<File> = Vec::new();
    let mut cwd: Vec<&str> = Vec::new();
    let mut root = Dir { name: String::from("/"), ..Default::default() };
    let mut path = Vec::new();
    path.push(root);


    // Assume first command will be "cd /"
    for line in lines {
        if line.starts_with("$") {
            //command
            if line.starts_with("$ ls") {
                println!("{:?}", path);
                println!("listing in {} ", path.last().unwrap());
            } else {  // change directory command
                //              print!("cd: {} to ", get_cwd(&cwd));
                let (_, dir) = line.split_once("cd ").unwrap();
                println!("change to directory name {dir}");
                //  println!("\t\t pre : {:?}", path);
                match dir {
                    "/" => {
                        println!("-:\t cd /");
                        while path.len() > 2 {
                            path.pop();
                        }
                    }
                    ".." => {
                        //  println!("{:?}", path);
                        println!("-:\t cd ..");
                        let top = path.pop();
                        match top {
                            None => {
                                println!("\t\t\t path pop: gave None");
                                panic!("!!!! 'cd ..' given at root directory");
                            }
                            Some(x) => {
                                //         println!("\t\t\t path pop:  {x} (path len = {})", path.len());
                            }
                        }
                        //   println!("{:?}", path);
                    }
                    d_name => {
                        println!("-:\t cd {d_name}");
                        let current_dir = path.last().unwrap();
                        let mut new_dir = None;
                        for sub_dir in &(current_dir.dirs) {
                            if sub_dir.name.eq(d_name) {
                                new_dir = Some(sub_dir);
                            }
                        }
                        match new_dir {
                            None => {
                                panic!("subdir {d_name} of {:?} not found", path);
                            }
                            Some(d) => {
                                println!("found subdir: {d}");
                                path.push(*d);
                            }
                        }
                    }
                }
            }
            //   println!("\t\t post: {:?}", path);

            //end command
        } else {
            //listing
            if line.starts_with("dir") {
                println!("list_dir: {}", line);
                let d: Dir = line.parse().unwrap();
                //     println!("list: {:?}", d);
                let de = path.last_mut().unwrap();

                de.dirs.push(d);
            } else {
                println!("list_file: {}", line);
                let f: File = line.parse().unwrap();
                path.last_mut().unwrap().files.push(f);
            }
        }
    }
    println!("{:?}", path);


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
