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
use std::ops::Index;

use parse_display::FromStr;


/*
    Advent of Code 2022: Day 19
        part1 answer:   1616
        part2 answer:   8990

 */

const TEST_ANSWER: (i64, i64) = (33, 56);
const INPUT_ANSWER: (i64, i64) = (1616, 8990);

const PART1_TEST_FILENAME: &str = "data/day19/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day19/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day19/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day19/part2_input.txt";

const PART1_TURNS:u32 = 24;

const TEST: bool = false;

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


#[repr(usize)]
enum Material {
    Ore=0,
    Clay=1,
    Obsidian=2,
    Geode=3
}

#[derive(PartialEq, Debug, Copy, Clone) ]
struct BluePrint {
    id: u32,
    ore_robot_cost: [u16;4],
    clay_robot_cost: [u16;4],
    obsidian_robot_cost: [u16;4],
    geode_robot_cost: [u16;4],
}
impl BluePrint {
    fn new(b: BluePrintInput) -> Self
    {
        BluePrint{
            id: b.id as u32,
            ore_robot_cost: [0,0,0,0],
            clay_robot_cost: [0,0,0,0],
            obsidian_robot_cost: [0,0,0,0],
            geode_robot_cost: [0,0,0,0]
        }
    }
}

#[derive(FromStr, PartialEq, Debug, Copy, Clone) ]
#[display("Blueprint {id}: Each ore robot costs {ore_r_ore} ore. Each clay robot costs {clay_r_ore} ore. Each obsidian robot costs {ob_r_ore} ore and {ob_r_clay} clay. Each geode robot costs {geo_r_ore} ore and {geo_r_ob} obsidian.")]
struct BluePrintInput {
    id:u32,
   ore_r_ore:u16,
    clay_r_ore:u16,
    ob_r_ore: u16,
    ob_r_clay: u16,
    geo_r_ore: u16,
    geo_r_ob :u16
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


#[derive(Hash, Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
struct State {
    turn: u16,
    ore: [u16;4],
    machine: [u16;4],
}
impl Default for State {
    fn default() -> State {
        State {
            turn: 0,
            ore: [0, 0, 0, 0],
            machine: [1, 0, 0, 0],
        }
    }
}
impl  State{

}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"State, t: {:>3} ore: [{:>3},{:>3},{:>3},{:>3}] machine: [{:>3},{:>3},{:>3},{:>3}]",
            self.turn,
            self.ore[0],self.ore[1],self.ore[2], self.ore[3],
              self.machine[0],self.machine[1],self.machine[2], self.machine[3]
        )
    }
}
fn vector_add( a:[u16;4] , b:[u16;4]) -> [u16;4] {
    let mut result: [u16;4] = [0,0,0,0];
    result[0] = a[0] + b[0];
    result[1] = a[1] + b[1];
    result[2] = a[2] + b[2];
    result[3] = a[3] + b[3];
    return result;
}
fn vector_sub(a:[u16;4] , b:[u16;4]) -> [u16;4] {
    let mut result: [u16;4] = [0,0,0,0];
    result[0] = a[0] - b[0];
    result[1] = a[1] - b[1];
    result[2] = a[2] - b[2];
    result[3] = a[3] - b[3];
    return result;
}


fn vector_ge(a:[u16;4] , b:[u16;4]) -> bool {
       a[0] >= b[0] &&
        a[1] >= b[1] &&
        a[2] >= b[2] &&
        a[3] >= b[3]
}

fn search(bp: [[u16; 4]; 4], max_turns: u16) -> u16 {

    let mut max_robots = [u16::MAX; 4];
    for i in 0..3 {
        max_robots[i] = bp.iter().map(|cost| cost[i]).max().unwrap();
    }
    let mut max_geodes =0;
    let mut max_time:u16 = max_turns;

    let mut queue:VecDeque<State> = VecDeque::new();

    let init_state = Default::default();
    queue.push_back(init_state);

    let mats = vec!{Material::Ore, Material::Clay, Material::Obsidian, Material::Obsidian};


  while let Some(State {
                     turn, ore, machine
                 }) = queue.pop_front() {
      for i in 0..bp.len() {
          if machine[i] == max_robots[i] {
              continue;
          }

          let costs = &bp[i];
          let wait_time = (0..3)
              .map(|idx| {
                  match costs[idx] {
                      cost if cost <= ore[idx] => 0 ,
                      _ if machine[idx] == 0 => max_time + 1 ,
                      _ => (costs[idx] - ore[idx] + machine[idx] - 1 ) / machine[idx],
                  }
              }).max().unwrap();

          // time to build bot and for it to produce for 1 turn
          let new_elapsed:u16 = turn + wait_time + 1;
          if new_elapsed >= max_time {
              continue;
          }




          let mut new_inventory: [u16; 4] = [0; 4];
          for idx in 0..machine.len() {
              new_inventory[idx] = ore[idx] + machine[idx] * (wait_time + 1) - costs[idx];
          }

          let mut new_bots = machine;
          new_bots[i] += 1;

          let remaining_time = max_time - new_elapsed;
          if ((remaining_time -1) * remaining_time) /2 + new_inventory[3] + remaining_time * new_bots[3] < max_geodes
          {
              continue;
          }


          queue.push_back(State {
              turn: new_elapsed,
              ore: new_inventory,
              machine: new_bots,
          })
      }
      let geodes = ore[3] + machine[3] * (max_time - turn);
      max_geodes = geodes.max(max_geodes);
  }
   max_geodes
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
        // println!("bpi: {:?}", bpi);
        let mut bp = BluePrint::new(bpi);
        bp.id = bpi.id;
        bp.ore_robot_cost = [bpi.ore_r_ore,0,0,0];
        bp.clay_robot_cost =[bpi.clay_r_ore, 0,0,0];
        bp.obsidian_robot_cost=[bpi.ob_r_ore, bpi.ob_r_clay,0,0];
        bp.geode_robot_cost = [bpi.geo_r_ore,0,bpi.geo_r_ob,0];

        v_blueprint.push(bp);


    }

    let mut answer1 = 0;
    let bp = v_blueprint[0];
    // println!("bp: {:?}", bp);
    for bp in v_blueprint {
        let a_b = [
            bp.ore_robot_cost,
            bp.clay_robot_cost,
            bp.obsidian_robot_cost,
            bp.geode_robot_cost
        ];
        // println!("a_b: {:?}", a_b);

        let r = search(a_b, 24);

        answer1 += bp.id * (r as u32);
    }
    return answer1.to_string();
}


fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));
    let lines: Vec<&str> = data2_s.trim().split("\n").map(|t| t.trim()).collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
    }


    let mut v_blueprint:Vec<BluePrint>  = Vec::new();
    for line in lines {
        let bpi = line.parse::<BluePrintInput>().unwrap();
        // println!("bpi: {:?}", bpi);
        let mut bp = BluePrint::new(bpi);
        bp.id = bpi.id;
        bp.ore_robot_cost = [bpi.ore_r_ore,0,0,0];
        bp.clay_robot_cost =[bpi.clay_r_ore, 0,0,0];
        bp.obsidian_robot_cost=[bpi.ob_r_ore, bpi.ob_r_clay,0,0];
        bp.geode_robot_cost = [bpi.geo_r_ore,0,bpi.geo_r_ob,0];

        v_blueprint.push(bp);


    }

    let mut answer2 = 1;
    let bp = v_blueprint[0];

    for bp in v_blueprint {
        let a_b = [
            bp.ore_robot_cost,
            bp.clay_robot_cost,
            bp.obsidian_robot_cost,
            bp.geode_robot_cost
        ];


        let r = search(a_b, 32);

        answer2 *= r as u32;
    }

    return answer2.to_string();
}
