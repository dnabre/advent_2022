#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]


use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;
use std::fmt;

use parse_display::FromStr;

/*
    Advent of Code 2022: Day 16
        part1 answer:
        part2 answer:

 */

const TEST_ANSWER: (u32, u32) = (0, 0);
const INPUT_ANSWER: (u32, u32) = (0, 0);

const PART1_TEST_FILENAME: &str = "data/day16/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day16/part1_input.txt";
const PART2_TEST_FILENAME: &str = "data/day16/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day16/part2_input.txt";


const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("16");

    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();

    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);

    // if TEST {
    //     assert_eq!(answer1, TEST_ANSWER.0.to_string());
    // } else {
    //     assert_eq!(answer1, INPUT_ANSWER.0.to_string());
    // }

    let start2 = Instant::now();
    let answer2 = part2();
    let duration2 = start2.elapsed();

    println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);

    // if TEST {
    //     assert_eq!(answer2, TEST_ANSWER.1.to_string());
    // } else {
    //     assert_eq!(answer2, INPUT_ANSWER.1.to_string());
    // }


    println!("----------\ndone");
}

const MAX_TIME:i32 = 30;


#[derive(PartialEq, Debug,Eq,Hash)]
struct Room {
    id:i32,
    value:i32,
    connected:Vec<i32>
}
//Valve LW has flow rate=0; tunnels lead to valves AA, HT

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value {} has flow rate={}; tunnels leads to values: {:?}",
               self.id, self.value, self.connected
        )
    }
}


impl Default for Room {
    fn default() -> Room {
        Room {
            id: -1,
            value: -1,
            connected: Vec::new(),
        }
    }
}



fn alpha_to_num(s:&String) -> i32{
    let ch1:u8 = s.chars().nth(0).unwrap() as u8;
    let ch2:u8 = s.chars().nth(1).unwrap() as u8;
    let mut base:i32 = 0;
    let right =  ch1 - ('A' as u8) ;
    if ch1 == ch2 {
        return right as i32;
    } else {
        base = 26;
    }
    let left:i32 =  (ch2 - ('A' as u8)) as i32;
    let p = (right as i32) + (26  * left) as i32 + base ;
    return p;
}


fn generate_alpha_mapping()->HashMap<(char,char),i32> {
    let mut n_to_a: HashMap<(char, char), i32> = HashMap::new();

    for ch1 in 'A'..='Z' {
        for ch2 in 'A'..='Z' {
            let p = (ch1, ch2);
            let mut sb = String::new();
            sb.push(ch1);
            sb.push(ch2);
            let n = alpha_to_num(&sb);
            let r = n_to_a.insert(p, n);
            if r != None {
                panic!("overwrote {sb}");
            }
        }
    }
    return n_to_a;
}


fn parse_room(ln: &mut &str) -> Room {
    let (mut l, mut r) = ln.split_once("=").unwrap();
    // println!("l=|{}|", l);
    let (ch1, ch2) = (l.chars().nth(6).unwrap(), l.chars().nth(7).unwrap());
    let mut s_id = String::with_capacity(2);
    s_id.push(ch1);
    s_id.push(ch2);
    // println!("ch1={ch1}, ch2={ch2}");
    let (mut l, mut r) = r.split_once(";").unwrap();
    let val: i32 = l.parse().unwrap();
    // println!("val= |{}|", val);
    // println!("r=|{r}| pre tunnels");
    let mut v_part;
    if r.contains("tunnels") {
        let (mut l, mut r) = r.split_at(24);
        v_part = r;
        // println!("l= |{l}| r=|{r}|");
    } else {
        let (mut l , mut r) = r.split_at(23);
        v_part = r;
        // println!("l= |{l}| r=|{r}|");
    }

    let parts: Vec<&str> = v_part.split(", ").collect();
    // println!("{:?}", parts);
    let mut v: Vec<i32> = Vec::new();

    for p in parts {
        let n = alpha_to_num(&p.to_string());
        // println!("p=|{p}|");
        // println!("p=|{p}|, n={n}");
        v.push(n);
    }
    // println!("s_id: |{}|", s_id);
    // println!("{:?}", v);

    let mut r=   Room {
        id: alpha_to_num(&s_id),
        value: val,
        connected: v.clone(),
    };
    return r;
}
pub fn factorial(num: u128) -> u128 {
    match num {
        0  => 1,
        1.. => (1..num+1).product(),
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
    let n_to_a = generate_alpha_mapping();

    let mut v_room:Vec<Room> = Vec::new();
    for i in 0..lines.len() {
        let mut ln = lines[i];
        ln = ln.trim();
        let mut r = parse_room(&mut ln);
        v_room.push(r);
    }
    println!("parsed {} rooms", v_room.len());
    for r in &v_room {
        println!("room: {:?}", r);
    }

    let mut rooms_without_zeros:u32 = 0;
    for r in &v_room {
        if r.value != 0 {
            rooms_without_zeros += 1;

        }
    }
    println!("non-zero values : {}, permutes: {}",
        rooms_without_zeros, factorial(rooms_without_zeros as u128));

    let answer1 = String::new();
    return answer1;
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
    let n_to_a = generate_alpha_mapping();








    let answer2 = String::new();
    return answer2;
}
