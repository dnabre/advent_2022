#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
use std::iter;
use std::ops::{Add, Index};
use std::fmt;

use std::thread::current;
use parse_display::FromStr;

/*
    Advent of Code 2022: Day 07
        part1 answer: 1581595
            1835142 is too high
        part2 answer:


        part1, other guy:
            194
            1581595
            Total sum of directories < 100000: 1581595
Directory to free: ////bcfwbq/gpbswq/tnsjg/jrflhz 1544176

 */


#[cfg(windows)]
const D_LINE_ENDING: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const D_LINE_ENDING: &'static str = "\n\n";

#[cfg(windows)]
const P_DELIMIT: &'static str = "\\";
#[cfg(not(windows))]
const P_DELIMIT: &'static str = "/";


const TEST: bool = true;

const PART1_TEST_FILENAME: &str = "data/day07/part1_test.txt";
const PART1_TEST2_FILENAME: &str = "data/day07/part1_test_2.txt";
const PART1_INPUT_FILENAME: &str = "data/day07/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day07/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day07/part2_input.txt";

const PART1_DIR_SIZE_CAP: u32 = 100_000;

const PART2_TOTAL_SPACE: u32 = 70_000_000;
const PART2_TARGET_UNUSED_SPACE: u32 = 30_000_000;


fn main() {
    print!("Advent of Code 2022, Day ");
    println!("07");                           // insert Day


   // test();
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
    size: u32,
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

    let mut dir_size: HashMap<String, u32> = HashMap::new();

    let mut directory_set:HashSet<String> = HashSet::new();



    let mut cwd = String::from("/");
    directory_set.insert(cwd.clone());
    dir_size.insert(cwd.clone(), 0);

    let mut line_count = 0;
    // Assume first command will be "cd /"
    for line in lines {
        line_count += 1;
 //       println!("cwd: {cwd} \t line: {line} \t\t l_num: {line_count}");
        if line.starts_with("$") {
            //command
            if line.starts_with("$ ls") {
   //             println!("listing  : {cwd}");
            } else {  // change directory command
                let mut debug = false;
                let (_, dir) = line.split_once("cd ").unwrap();
            //    println!("cd from {cwd}, {dir}");
                match dir {
                    "/" => {
                        cwd = String::from("/");

                    }
                    ".." => {
                        if cwd.ne("/") {
                           // let p_cwd = dir_up(&mut cwd);
                           // cwd = p_cwd;
                          //
                            cwd = dir_up(&cwd);
                          }
                    }
                    d_name => {
                        if debug {println!("\t cd __ branch, cwd: {cwd}, d_name: {d_name}")};
                    //    if cwd.ne("/") { cwd.push('/'); }
                        cwd.push_str(d_name);
                        if debug {println!("push_str d_name ({d_name}) onto cwd, resulting in {cwd}")};
                        cwd.push('/');
                        if debug {println!("pushed slash onto cwd, resulting {cwd}")};
                        let rr = directory_set.insert(cwd.clone());
                        if debug {println!("inserted clone of cwd into directory_set, result {}", rr)};
                        let rr2 = dir_size.insert(cwd.clone(), 0);
                        if debug {println!("insert cwd = 0 , {cwd} into dir_size, result {:?}", rr2)};
                    }

                }
      //          println!("moved to {cwd}");
            }
            //end command
        } else {
            //listing



            if line.starts_with("dir") {
                let d: Dir = line.parse().unwrap();
        //        println!("list  dir: {} \t -:- {cwd} ", d.name);

            } else {
                let mut c_size =  match dir_size.get(&cwd) {
                    None => {0}
                    Some(&x) => {x}
                };
                let f: File = line.parse().unwrap();
          //      println!("\t File: {cwd} <|> {} \t{}", f.name,f.size);
                c_size = c_size +  f.size;
                let r = dir_size.insert(cwd.clone(),c_size);

            }
        }
    }

  //  println!("\n directory list:");
    for d in &directory_set {
        let size = dir_size.get(d);
        match size {
            None => {
                println!("\t{d:<20}  \t size: (unknown)");
                panic!("directory {d} should have known size");
            }
            Some(s) => {
   //             println!("\t{d:<20}  \t size: {s}");
            }
        }

    }


    let cum_dir_sizes:HashMap<String,u32> =
        get_cumulative_directory_sizes(&directory_set, &dir_size);

    let mut total_size:u32 = 0;

    //println!("\n directory list:");
    for d in &directory_set {
        let size = cum_dir_sizes.get(d);
        match size {
            None => {
                println!("\t{d:<20}  \t size: (unknown)");
                panic!("directory {d} should have known size");
            }
            Some(&s) => {
                if s < PART1_DIR_SIZE_CAP{
                    total_size += s;

                }
       //        println!("\t{d:<20}  \t size: {s}");
            }
        }

    }


    println!("directory entries: {}", directory_set.len());
    let mut answer1 = total_size.to_string();
    return answer1;
}


fn get_cumulative_directory_sizes(dirs_set: &HashSet<String>, dir_size: &HashMap<String, u32>) -> HashMap<String, u32> {
    let mut cum_size_map:HashMap<String,u32> = HashMap::new();

    for dir in dirs_set {
        let cwd = dir.clone();
        let mut size:u32 = 0;
        // println!("scanning for subdirs of {cwd}");
        for sub_dir in dirs_set {
            let sd_size = match dir_size.get(sub_dir) {
                None => {
                    panic!("can't find size for {sub_dir}");
                }
                Some(x) => {x}
            };
            // println!("\t subdir: {sub_dir:<20} \t size: {sd_size} ");
            if sub_dir.starts_with(&cwd) {
                let new_size = sd_size + size;
                // println!("\t\t {sub_dir} is inside {dir}, adding it's size: {} + {} => {}", sd_size, size, new_size);
                size = new_size;
            }
        }
        cum_size_map.insert(cwd, size);

    }

    return cum_size_map;
}


fn last_char_is(s:&String, ch: char)->bool {
    let c = s.chars().last().unwrap();
    if c==ch {
        return true;
    } else {
        return false;
    }
}

fn dir_up(cwd: & String) -> String {
    let mut p_cwd = cwd.clone();
    p_cwd.pop();
    let (left, _) = p_cwd.rsplit_once("/").unwrap();
    if left.is_empty() {
        p_cwd = String::from("/");
    } else {
        p_cwd = String::from(left);
    }
    return p_cwd
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
