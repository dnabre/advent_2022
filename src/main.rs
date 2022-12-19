#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::{cmp, fmt};
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::fmt::{Display, Formatter};
use std::fs;

use std::time::{Instant,Duration};

use parse_display::FromStr;



/*
    Advent of Code 2022: Day 19
        part1 answer: =
        part2 answer:

 */

const TEST_ANSWER: (i64, i64) = (0, 0);
const INPUT_ANSWER: (i64, i64) = (0, 0);

const PART1_TEST_FILENAME: &str = "data/day19/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day19/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day19/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day19/part2_input.txt";

const TURNS:isize = 24;

const TEST: bool = true;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("19");                           // insert Day


    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();

    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);
    //
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

    println!("----------\ndone");
}

#[derive(PartialEq, Debug, Copy, Clone) ]
struct BluePrint {
    id: u8,
    ore_robot_cost: u8,
    clay_robot_cost: u8,
    obsidian_robot_cost: (u8,u8),
    geode_robot_cost: (u8,u8),
}
impl BluePrint {
    fn new(b: BluePrintInput) -> Self
    {
        BluePrint{
            id: b.id as u8,
            ore_robot_cost: b.ore_r_ore as u8,
            clay_robot_cost: b.clay_r_ore as u8,
            obsidian_robot_cost: (b.ob_r_ore as u8,b.ob_r_clay as u8),
            geode_robot_cost: (b.geo_r_ore as u8,b.geo_r_ob as u8)
        }
    }
}

#[derive(FromStr, PartialEq, Debug, Copy, Clone) ]
#[display("Blueprint {id}: Each ore robot costs {ore_r_ore} ore. Each clay robot costs {clay_r_ore} ore. Each obsidian robot costs {ob_r_ore} ore and {ob_r_clay} clay. Each geode robot costs {geo_r_ore} ore and {geo_r_ob} obsidian.")]
struct BluePrintInput {
    id:u32,
   ore_r_ore:u32,
    clay_r_ore:u32,
    ob_r_ore: u32,
    ob_r_clay: u32,
    geo_r_ore: u32,
    geo_r_ob :u32
}

impl Default for BluePrintInput {
    fn default() -> BluePrintInput {
        BluePrintInput {
            id: 0,
            ore_r_ore: 0,
            clay_r_ore: 0,
            ob_r_ore: 0,
            ob_r_clay: 0,
            geo_r_ore: 0,
            geo_r_ob: 0,
        }
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct State {
    turn: u32,
    ore: (u32,u32,u32,u32),
    machine: (u32,u32,u32,u32),
}
impl Default for State {
    fn default() -> State {
        State {
            turn: 0,
            ore: (1, 0, 0, 0),
            machine: (0, 0, 0, 0),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"State, t: {:>3} ore: {:>3},{:>3},{:>3},{:>3}, machine: {:>3},{:>3},{:>3},{:>3}",
            self.turn,
               self.ore.0, self.ore.1, self.ore.2, self.ore.3,
                self.machine.0,self.machine.1,self.machine.2,self.machine.3
        )
    }
}

fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };
    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let lines: Vec<&str> = data1_s.trim().split("\n").map(|t| t.trim()).collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
    }
    
    let mut v_blueprint:Vec<BluePrint>  = Vec::new();
    for line in lines {
        let bpi = line.parse::<BluePrintInput>().unwrap();
        let bp = BluePrint::new(bpi);
        v_blueprint.push(bp);
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

    let mut answer2 =String::new();
    return answer2;
}
