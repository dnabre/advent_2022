#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]



use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::{fmt, fs};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::io::Write;
use std::path::Component::ParentDir;
use std::process::exit;
use std::time::Instant;


/*
    Advent of Code 2022: Day 16`
        part1 answer:
        part2 answer:
 */


const TEST_ANSWER: (i64, i64) = (1651, 54);
// 25.652ms
const INPUT_ANSWER: (i64, i64) = (2077, 2741);


const PART1_TEST_FILENAME: &str = "data/day16/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day16/part1_input.txt";
const PART2_TEST_FILENAME: &str = "data/day16/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day16/part2_input.txt";




const TEST: bool = true;



fn string_to_pair(s:&str) -> (char,char) {

    let mut c_a = s.chars();
    let a:char = c_a.next().unwrap();
    let b:char = c_a.next().unwrap();


    return (a,b);
}



#[derive(Debug, PartialEq, Eq, Clone)]
struct Room {
    id: usize,
    flow: i32,
    connects: HashSet<usize>,
}

impl Display for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Room [{}] @ {}: {:?}", self.id, self.flow, self.connects)
    }
}


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
    // if TEST {
    //     if answer2 != TEST_ANSWER.1.to_string() {
    //         println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
    //                  answer2, TEST_ANSWER.1.to_string())
    //     }
    // } else {
    //     if answer2 != INPUT_ANSWER.1.to_string() {
    //         println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
    //                  answer2, TEST_ANSWER.1.to_string())
    //     }
    // }
    println!("----------\ndone");
}













fn parse_room(line: &String, char_map:&HashMap<(char,char),usize>) -> (usize, i32, HashSet<usize>) {
    let (valve, neighbors) = line.split_once("; ").unwrap();
    let valve = valve.strip_prefix("Valve ").unwrap();
    let (name, flow) = valve.split_once(" has flow rate=").unwrap();
    let flow: i32 = flow.parse().unwrap();
    let neighbors = neighbors
        .strip_prefix("tunnels lead to valves ")
        .or_else(|| neighbors.strip_prefix("tunnel leads to valve "))
        .unwrap();

    let neighbors:HashSet<&str> = neighbors.split_terminator(", ").collect();

    let id:usize = *char_map.get(&string_to_pair(name)).unwrap();
    let i_neigh:HashSet<usize> = neighbors.iter().map(|s|*char_map.get(&string_to_pair(s)).unwrap()).collect();
    (id, flow,i_neigh)
}

const MAX_MINUTES: i32 = 30;
const START_ROOM:&str = "AA";

fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };
    let file = File::open(p1_file).expect(&*format!("error opening file {}", p1_file));
    let bfile = BufReader::new(file);
    let lines:Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();

    if TEST {
        let l_num = lines.len();
        println!("\t read {} lines from {}", l_num, p1_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }


        //let mut room_v:Vec<Room> = Vec::with_capacity(lines.len());
    //let mut name_to_room:HashMap<&str, Room> = HashMap::with_capacity(lines.len());
    let mut char_index:usize =0;
    let mut char_to_number:HashMap<(char,char), usize> = HashMap::new();
    let mut char_name_vec:Vec<(char,char)> = Vec::new();

    for one in 'A'..='Z' {
        for two in 'A' ..='Z'{
            let t = (one, two);
            char_name_vec.push(t);
            char_to_number.insert(t, char_index);
            char_index += 1;
        }
    }
    let char_to_number = char_to_number;
    let char_name_vec = char_name_vec;

    let mut room_vec:Vec<Room> = Vec::new();
    for i in 0..lines.len() {
        let  (name, flow,neighbors) =   parse_room(&lines[i], &char_to_number);
        let new_room = Room{
            id: name,
            flow: flow,
            connects: neighbors,
        };

        println!("{}", lines[i]);
        println!("\t {}", new_room);
        let p_list:Vec<(char,char)> = new_room.connects.iter().map(|s| char_name_vec[*s]).collect();
        println!("\t Room [{:?}] @ {}: {:?}", char_name_vec[new_room.id], new_room.flow,p_list);


        room_vec.push(new_room);

    }


    // note in search expansion, include an option to stand sill for round



    let answer1 = String::new();
    return answer1.to_string();
}


fn part2() -> String {
    println!("   start part 2");
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };

    let file = File::open(p2_file).expect(&*format!("error opening file {}", p2_file));
    let bfile = BufReader::new(file);
    let lines:Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();

    if TEST {
        let l_num = lines.len();
        println!("\t read {} lines from {}", l_num, p2_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }



    let answer2 = String::new();
    return answer2.to_string();
}
