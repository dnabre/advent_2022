#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::time::Instant;

use parse_display::FromStr;



/*
    Advent of Code 2022: Day 10
        part1 answer: 15220
        part2 answer:

 */


const TEST_ANSWER: (u32, u32) = (13140, 0);
const INPUT_ANSWER: (u32, u32) = (15220,0);

const PART1_TEST_0_FILENAME: &str = "data/day10/part1_test_0.txt";
const PART1_TEST_FILENAME: &str = "data/day10/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day10/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day10/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day10/part2_input.txt";

const TEST: bool = true;
fn main() {
    print!("Advent of Code 2022, Day ");
    println!("10");


    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();

    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);

    if TEST {
        assert_eq!(answer1, TEST_ANSWER.0.to_string());
    } else {
        assert_eq!(answer1, INPUT_ANSWER.0.to_string());
    }

    // let start2 = Instant::now();
    // let answer2 = part2();
    // let duration2 = start2.elapsed();
    // println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);
    // // if TEST {
    //     assert_eq!(answer2, TEST_ANSWER.1.to_string());
    // } else {
    //     assert_eq!(answer2, INPUT_ANSWER.1.to_string());
    // }


    println!("----------\ndone");
}

#[derive(PartialEq, Debug,Copy,Clone)]
struct Machine {
    register:i32,
    cycle_counter:i32
}

    impl fmt::Display for Machine {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f,"\t machine, cycle {:>12}, register {:>12}", self.cycle_counter, self.register)
    }
}
impl Default for Machine {
    fn default() -> Machine {
        Machine { cycle_counter:1, register:1}
    }
}

#[derive(FromStr, PartialEq, Debug,Copy,Clone)]
enum Instruction {
    #[display("noop")]
    Noop,
    #[display("addx {0}")]
    Addx(i32),
    #[display("<wait>")]
    WaitState
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Noop => {write!(f,"noop")},
            Instruction::Addx(x) => {write!(f,"addx {x}")},
            Instruction::WaitState => {write!(f,"<wait>")}
        }
    }
}


fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };

    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let mut lines: Vec<&str> = data1_s.trim().split("\n").map(|t| t.trim_end()).collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
    }
    let mut code:Vec<Instruction> = Vec::new();

    for ln in lines {
        let i:Instruction = ln.parse().unwrap();
        match i {
            Instruction::Noop => { code.push(i)}
            Instruction::Addx(x) => {
                code.push(Instruction::WaitState);
                code.push(Instruction::Addx(x))
            }
            Instruction::WaitState => {}
        }
    }

    let mut m:Machine = Default::default();
    println!("inital machine: {m}\n");

    let mut total_ss = 0;
    for i in code {
        //println!("\t{i:<8}");


        match i {
            Instruction::Noop => {}
            Instruction::Addx(x) => {m.register+=x}
            Instruction::WaitState => {}
        }
        m.cycle_counter += 1;
        if (m.cycle_counter -20)  % 40 == 0 {
            let ss = m.cycle_counter * m.register;
   //         println!("\tss-> {m}, signal strength: {ss:>14}");
            total_ss += ss;
        }
    }

    println!("\nfinal machine: {m}");
    println!("total signal strength: {total_ss}");
    let mut answer1 = total_ss.to_string();
    return answer1;
}


fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));

    let mut lines: Vec<&str> = data2_s.trim().split("\n").map(|t| t.trim_end()).collect();
    let l_num = lines.len();

    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
    }
    for ln in lines {
        println!("\t{}", ln);
    }

    let mut answer2 = 0.to_string();
    return answer2;
}
