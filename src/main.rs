
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
const PART1_TEST2_FILENAME: &str = "data/day07/part1_test_2.txt";
const PART1_INPUT_FILENAME: &str = "data/day07/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day07/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day07/part2_input.txt";

const PART1_DIR_SIZE_CAP: u32 = 100_000;

const PART2_TOTAL_SPACE: u32 = 70_000_000;
const PART2_TARGET_UNUSED_SPACE: u32 = 30_000_000;


const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("07");                           // insert Day


    // test();
    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();
    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);
    if TEST {
        assert_eq!(answer1, TEST_ANSWER.0.to_string());
    } else {
        assert_eq!(answer1, INPUT_ANSWER.0.to_string());
    }

    let start2 = Instant::now();
    let answer2 = part2();
    let duration2 = start2.elapsed();
    println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);
    if TEST {
        assert_eq!(answer2, TEST_ANSWER.1.to_string());
    } else {
        assert_eq!(answer2, INPUT_ANSWER.1.to_string());
    }


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


    let (mut dir_size, mut directory_set): (HashMap<String, u32>, HashSet<String>)
        = parse_into_directories(&mut lines);


    let cum_dir_sizes: HashMap<String, u32> =
        get_cumulative_directory_sizes(&directory_set, &dir_size);

    let mut total_size: u32 = 0;

    //println!("\n directory list:");
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
                //        println!("\t{d:<20}  \t size: {s}");
            }
        }
    }


    println!("directory entries: {}", directory_set.len());
    let mut answer1 = total_size.to_string();
    return answer1;
}


// ctor_set):(HashMap<String,u32>,HashSet<String> )

fn parse_into_directories(lines: &mut Vec<&str>) -> (HashMap<String, u32>, HashSet<String>) {
    let mut dir_size: HashMap<String, u32> = HashMap::new();
    let mut directory_set: HashSet<String> = HashSet::new();

    let mut cwd = String::from("/");
    directory_set.insert(cwd.clone());
    dir_size.insert(cwd.clone(), 0);

    // Assume first command will be "cd /"
    for line in lines {
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
                        if debug { println!("\t cd __ branch, cwd: {cwd}, d_name: {d_name}") };

                        cwd.push_str(d_name);

                        cwd.push('/');

                        directory_set.insert(cwd.clone());
                        dir_size.insert(cwd.clone(), 0);

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
                let mut c_size = match dir_size.get(&cwd) {
                    None => { 0 }
                    Some(&x) => { x }
                };
                let f: File = line.parse().unwrap();
                //      println!("\t File: {cwd} <|> {} \t{}", f.name,f.size);
                c_size = c_size + f.size;
                let r = dir_size.insert(cwd.clone(), c_size);
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
        // println!("scanning for subdirs of {cwd}");
        for sub_dir in dirs_set {
            let sd_size = match dir_size.get(sub_dir) {
                None => {
                    panic!("can't find size for {sub_dir}");
                }
                Some(x) => { x }
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


fn last_char_is(s: &String, ch: char) -> bool {
    let c = s.chars().last().unwrap();
    if c == ch {
        return true;
    } else {
        return false;
    }
}

/*
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
*/
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

    for i in 0..lines.len() {
        lines[i] = lines[i].trim();
    }


    let (mut dir_size, mut directory_set): (HashMap<String, u32>, HashSet<String>)
        = parse_into_directories(&mut lines);


    let cum_dir_sizes: HashMap<String, u32> =
        get_cumulative_directory_sizes(&directory_set, &dir_size);
    let mut root_used = cum_dir_sizes.get("/").unwrap();
    let mut free_space = PART2_TOTAL_SPACE - root_used;
    let mut need_to_free = PART2_TARGET_UNUSED_SPACE - free_space;


    let mut dir_size_totals: Vec<&u32> = cum_dir_sizes.values().collect();
    //   println!("{:?}", dir_size_totals);


    let mut size_values: Vec<u32> = dir_size_totals.iter().map(|x| **x).collect();
    size_values.sort();

    let mut to_free = 0;

    let mut to_free = 0;
    for i in 0..size_values.len() {
        let v = size_values[i];
        if v >= need_to_free {
            to_free = v;
            break;
        }
    }
    // target answer 1544176

    let mut answer2 = to_free.to_string();
    return answer2;
}
