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
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

const PART1_TURNS:u32 = 24;

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
    id: u32,
    ore_robot_cost: (u32,u32,u32,u32),
    clay_robot_cost: (u32,u32,u32,u32),
    obsidian_robot_cost: (u32,u32,u32,u32),
    geode_robot_cost: (u32,u32,u32,u32),
}
impl BluePrint {
    fn new(b: BluePrintInput) -> Self
    {
        BluePrint{
            id: b.id as u32,
            ore_robot_cost: (b.ore_r_ore,0,0,0),
            clay_robot_cost: (b.clay_r_ore,0,0,0),
            obsidian_robot_cost: (b.ob_r_ore ,b.ob_r_clay,0,0),
            geode_robot_cost: (b.geo_r_ore,0,b.geo_r_ob,0 )
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

#[derive(PartialEq, Debug, Copy, Clone) ]
enum Action {
    BuildGeodeBot,
    BuildObsidianBot,
    BuildClayBot,
    BuildOreBot
}


#[derive(Hash, Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
struct State {
    turn: u32,
    ore: (u32,u32,u32,u32),
    machine: (u32,u32,u32,u32),
}
impl Default for State {
    fn default() -> State {
        State {
            turn: 1,
            ore: (0, 0, 0, 0),
            machine: (1, 0, 0, 0),
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
impl State {
    fn tick(mut self)->State {
        let mut new_state = self.clone();
        new_state.turn +=1 ;
        new_state.ore = tuple_add(self.ore, self.machine);
        return new_state;
    }

    fn tick_and_build(&self,bp:&BluePrint, action:&Action) -> State {
        let mut new_state = self.clone();
        new_state.turn +=1 ;
        new_state.ore = tuple_add(self.ore, self.machine);
        match action {
            Action::BuildGeodeBot => {
                assert!(self.can_build(bp,action,));
                new_state.ore = tuple_sub(self.ore, bp.geode_robot_cost);
                new_state.machine.3 +=1;
            }
            Action::BuildObsidianBot => {
                assert!(self.can_build(bp,action,));
                new_state.ore = tuple_sub(self.ore, bp.obsidian_robot_cost);
                new_state.machine.2 +=1;
            }
            Action::BuildClayBot => {
                assert!(self.can_build(bp,action,));
                new_state.ore = tuple_sub(self.ore, bp.clay_robot_cost);
                new_state.machine.1 +=1;
            }
            Action::BuildOreBot => {
                assert!(self.can_build(bp,action,));
                new_state.ore = tuple_sub(self.ore, bp.ore_robot_cost);
                new_state.machine.0 +=1;
            }
        }
        return new_state;
    }

    fn can_build(&self, blue_print:&BluePrint, action:&Action) -> bool {
        match action {
            Action::BuildGeodeBot => {
               tuple_ge(self.ore, blue_print.geode_robot_cost)
            }
            Action::BuildObsidianBot => {
                tuple_ge(self.ore, blue_print.obsidian_robot_cost)
            }
            Action::BuildClayBot => {
                tuple_ge(self.ore, blue_print.clay_robot_cost)
            }
            Action::BuildOreBot => {
                tuple_ge(self.ore, blue_print.ore_robot_cost)
            }
        }
    }

}

fn tuple_add( a:(u32,u32,u32,u32) , b:(u32,u32,u32,u32))->(u32,u32,u32,u32) {
    return (a.0+b.0, a.1+b.1, a.2+b.2, a.3+b.3);
}

fn tuple_sub( a:(u32,u32,u32,u32) , b:(u32,u32,u32,u32))->(u32,u32,u32,u32) {
    return (a.0-b.0, a.1-b.1, a.2-b.2, a.3-b.3);
}


fn tuple_ge( a:(u32,u32,u32,u32) , b:(u32,u32,u32,u32))->bool {
    let ge_t = (a.0>=b.0, a.1>=b.1, a.2>=b.2, a.3>=b.3);
    return ge_t.0 && ge_t.1 && ge_t.2 && ge_t.3;
}




fn search(bp:&BluePrint, end_turn: u32) -> u32 {

    // store best geode score at each time
    // let mut cache: HashMap<i32, usize> = HashMap::new();
    // for i in 0..=minutes {
    //     cache.insert(i, 0);
    // }

    let mut queue: VecDeque<State> = VecDeque::new();
    let mut visited: HashSet<State> = HashSet::new();
let mut max_geo_machines = 0;
   let mut max_geodes = 0;

    let mut actions:Vec<Action>= Vec::new();
    actions.push(Action::BuildGeodeBot);
    actions.push(Action::BuildObsidianBot);
    actions.push(Action::BuildClayBot);
    actions.push(Action::BuildOreBot);

    let mut start_state = Default::default();
    queue.push_back(start_state);
    let mut best_at_end:u32 = u32::MIN;

        while !queue.is_empty()
        {
        let current = queue.pop_front().unwrap();
        let init_state = current.clone();
            max_geo_machines = cmp::max(init_state.machine.3, max_geo_machines);
            max_geodes = cmp::max(init_state.ore.3, max_geodes);


            visited.insert(current);

        if current.turn > end_turn {
    //        println!("- state at end of turn: \n \t{}", current);
            best_at_end = cmp::max(best_at_end,current.ore.3);
        } else {
            for act in &actions {
                if current.can_build(bp,act) {
                    // if *act==Action::BuildGeodeBot {
                    //     println!("can build geodebot from state: {current}");
                    // }
                     let c = current.tick_and_build(bp,act);

                    // if *act==Action::BuildGeodeBot {
                    //     println!("\t built geoodebot: {c}");
                    // }
                    // // println!("new state after action ({:?}) \t: {}",act, c);
                    if !visited.contains(&c) {
                        // println!("\t inserting {c}");
                        queue.push_back(c);
                    }
                    break;
                }
            }
            // if init_state.machine.3 > 0 {
            //     print!("state has {} geode machines|-, " ,init_state);
            // }

            let c = current.tick();
            // if init_state.machine.3 > 0 {
            //     print!("new state has {} geodes ({}), ", c.ore.3, c);
            // }

            // println!("new state after no action  \t: {}", c);
            if !visited.contains(&c) {
                // println!("\t inserting {c}");
                queue.push_back(c);
            }
        }
    }

    println!("max geo machines: {}", max_geo_machines);
    println!("max geodes: {}", max_geodes);
    return best_at_end;
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
    for r in v_blueprint {
        println!("blueprint: {:?}", r);
        let r_best = search(&r, PART1_TURNS);
        println!("{}: {}",r.id, r_best );
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
