#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use parse_display::FromStr;

/*
    Advent of Code 2022: Day 07
        part1 answer: 1581595
        part2 answer: 1544176

 */

const TEST_ANSWER: (u32, u32) = (95437, 24933642);
const INPUT_ANSWER: (u32, u32) = (1581595, 1544176);

const PART1_TEST_FILENAME: &str = "data/day07/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day07/part1_input.txt";
const PART2_TEST_FILENAME: &str = "data/day07/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day07/part2_input.txt";


const PART1_DIR_SIZE_CAP: u32 = 100_000;
const PART2_TOTAL_SPACE: u32 = 70_000_000;
const PART2_TARGET_UNUSED_SPACE: u32 = 30_000_000;


const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("07");

    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();
    let assert;
    if TEST {
        assert_eq!(answer1, TEST_ANSWER.0.to_string());
        assert = '*';
    } else {
        assert_eq!(answer1, INPUT_ANSWER.0.to_string());
        assert = '*';
    }
    println!("\t Part 1: {answer1} , {assert} \t time: {:?}", duration1);


    let start2 = Instant::now();
    let answer2 = part2();
    let duration2 = start2.elapsed();
    let assert;
    if TEST {
        assert_eq!(answer2, TEST_ANSWER.1.to_string());
        assert = '*';
    } else {
        assert_eq!(answer2, INPUT_ANSWER.1.to_string());
        assert = '*';
    }
    println!("\t Part 2: {answer2} , {assert} \t time: {:?}", duration2);
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
    for i in 0..lines.len() { lines[i] = lines[i].trim(); }

    let (dir_size, directory_set): (HashMap<String, u32>, HashSet<String>)
        = parse_into_directories(&mut lines);
    let cum_dir_sizes: HashMap<String, u32> =
        get_cumulative_directory_sizes(&directory_set, &dir_size);
    let mut total_size: u32 = 0;

    for d in &directory_set {
        let size = cum_dir_sizes.get(d);
        match size {
            None => {
                println!("\t{d:<20}  \t size: (unknown)");
                panic!("directory {d} should have known size");
            }
            Some(&s) => {
                if s < PART1_DIR_SIZE_CAP {
                    total_size += s;
                }
            }
        }
    }
    let answer1 = total_size.to_string();
    return answer1;
}




fn parse_into_directories(lines: &mut Vec<&str>) -> (HashMap<String, u32>, HashSet<String>) {
    let mut dir_size: HashMap<String, u32> = HashMap::new();
    let mut directory_set: HashSet<String> = HashSet::new();

    let mut cwd = String::from("/"); // start at root directory
    directory_set.insert(cwd.clone());
    dir_size.insert(cwd.clone(), 0);

    for line in lines {
        if line.starts_with("$") {
            if line.starts_with("$ ls") {
                // Don't care about this command ignore it, and just pay attention to output
            } else {  // change directory command
                let (_, dir) = line.split_once("cd ").unwrap();
                match dir {
                    "/" => {
                        cwd = String::from("/");
                    }
                    ".." => {
                        if cwd.ne("/") {
                            cwd = dir_up(&cwd);
                        }
                    }
                    d_name => {
                        cwd.push_str(d_name);
                        cwd.push('/');
                        directory_set.insert(cwd.clone());
                        dir_size.insert(cwd.clone(), 0);
                    }
                }
            }
        } else {
            //listing
            if !line.starts_with("dir") {
                let mut c_size = match dir_size.get(&cwd) {
                    None => { 0 }
                    Some(&x) => { x }
                };
                let f: File = line.parse().unwrap();
                c_size = c_size + f.size;
                dir_size.insert(cwd.clone(), c_size);
            }
        }
    }
    return (dir_size, directory_set);
}


fn get_cumulative_directory_sizes(dirs_set: &HashSet<String>, dir_size: &HashMap<String, u32>) -> HashMap<String, u32> {
    let mut cum_size_map: HashMap<String, u32> = HashMap::new();
    for dir in dirs_set {
        let cwd = dir.clone();
        let mut size: u32 = 0;
        for sub_dir in dirs_set {
            let sd_size = match dir_size.get(sub_dir) {
                None => { panic!("can't find size for {sub_dir}"); }
                Some(x) => { x }
            };
            if sub_dir.starts_with(&cwd) {
                let new_size = sd_size + size;
                size = new_size;
            }
        }
        cum_size_map.insert(cwd, size);
    }
    return cum_size_map;
}


fn dir_up(cwd: &String) -> String {
    let mut p_cwd = cwd.clone();
    p_cwd.pop();
    let (left, _) = p_cwd.rsplit_once("/").unwrap();
    if left.is_empty() {
        p_cwd = String::from("/");
    } else {
        p_cwd = String::from(left);
    }
    p_cwd.push('/');
    return p_cwd;
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

    for i in 0..lines.len() { lines[i] = lines[i].trim(); }

    let (dir_size, directory_set): (HashMap<String, u32>, HashSet<String>)
        = parse_into_directories(&mut lines);
    let cum_dir_sizes: HashMap<String, u32> =
        get_cumulative_directory_sizes(&directory_set, &dir_size);

    let root_used = cum_dir_sizes.get("/").unwrap();
    let free_space = PART2_TOTAL_SPACE - root_used;
    let need_to_free = PART2_TARGET_UNUSED_SPACE - free_space;

    let dir_size_totals: Vec<&u32> = cum_dir_sizes.values().collect();
    let mut size_values: Vec<u32> = dir_size_totals.iter().map(|x| **x).collect();
    size_values.sort();

    let mut to_free = 0;
    for i in 0..size_values.len() {
        let v = size_values[i];
        if v >= need_to_free {
            to_free = v;
            break;
        }
    }

    let answer2 = to_free.to_string();
    return answer2;
}
