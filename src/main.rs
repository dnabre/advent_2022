#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use std::fs;
use std::iter;
use std::ops::{Add, Index};
use std::thread::current;
use std::time::Instant;

use parse_display::FromStr;


/*
    Advent of Code 2022: Day 21
        part1 answer:
        part2 answer:

 */

const TEST_ANSWER: (u32, u32) = (0, 0);
const INPUT_ANSWER: (u32, u32) = (0, 0);


const PART1_TEST_FILENAME: &str = "data/day21/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day21/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day21/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day21/part2_input.txt";


const TEST: bool = true;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("21");                           // insert Day


    // test();
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

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[derive(FromStr, PartialEq, Hash,Debug,Copy,Clone)]
enum Op {
    #[display("+")]
    Plus,
    #[display("-")]
    Sub,
    #[display("/")]
    Div,
    #[display("*")]
    Mul
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Op::Plus => {"+"}
            Op::Sub => {"-"}
            Op::Div => {"/"}
            Op::Mul => {"*"}
        })
    }
}

#[derive(FromStr, Hash, PartialEq,Debug,Clone)]
enum MonkeyOp {
    #[display("{0}")]
    Number (i32),
    #[display("{left} {op} {right}")]
    Eq {left:String,op:Op,right:String},
}

impl fmt::Display for MonkeyOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MonkeyOp::Number(x) => {
                write!(f,"{:>5}", x)}
            MonkeyOp::Eq { left, op, right } => {
                    write!(f,"{left} {op} {right}")
            }
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
    let mut lines: Vec<&str> = data1_s.trim().split(LINE_ENDING).collect();
    let l_num = lines.len();
    if !TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }


    let mut name_to_op:HashMap<String,MonkeyOp> = HashMap::new();
    let mut v_namelist:Vec<String> = Vec::new();


    for ln in lines {
    //    println!("{}", ln);
        let (l, mut r) = ln.split_once(":").unwrap();

        let m_name: String = String::from(l);
        if m_name.ne("root") {
            v_namelist.push(m_name.clone());
        }
        let m_op = r.trim().parse::<MonkeyOp>().unwrap();
        name_to_op.insert(m_name, m_op);

    }

    let root_op = name_to_op.get(&String::from("root"));
    println!("root operatoin: {:?}", root_op);
    let mut once = false;
    //while name_to_op.len() > 1 {
     'reduce: while !once {
         once = true;
        let mut v_names_to_clear:Vec<String> = Vec::new();
        let mut o_op:Option<(&String, &Op, &String)>;
        for c_name in &v_namelist {
            let v = name_to_op.get(&c_name.clone());

            match v {
                None => {
                    o_op = None;
                    println!("!! could not find operation for name: {}", c_name);
                }
                Some(m_op) => {
                    match m_op {
                        MonkeyOp::Number(_) => {
                            continue;
                        }
                        MonkeyOp::Eq { left, op, right } => {
                            o_op = Some((left,op,right));
                        }
                    }
                }
            }
            
            let current_op = match o_op {
                None => {
                    println!("!! could not find operation");
                    break 'reduce;
                }
                Some((left,op,right)) => {
                    println!(" \t working on {} {} {}", left, op , right);
                    let left_v = name_to_op.get(left).unwrap();
                    let right_v = name_to_op.get(right).unwrap();
                    let (l,r) = match (left_v,right_v) {
                            (MonkeyOp::Number(x), MonkeyOp::Number(y)) => (x,y),
                            (_,_) => {
                                continue
                            }
                        };
                        let result = match op {
                            Op::Plus => {l+r}
                            Op::Sub => {l - r}
                            Op::Div => {l / r}
                            Op::Mul => {l * r}
                        };
                        println!("\t reducing {c_name} :: {left} {op} {right} => {l} {op} {r} => {result}");
                      //  name_to_op.remove(left);


                        // v_names_to_clear.push(left.clone());
                        // v_names_to_clear.push(right.clone());
                        // name_to_op.insert(c_name.clone(), MonkeyOp::Number(result));

                    }



            };

        }


    }


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
    let mut lines: Vec<&str> = data2_s.trim().split(LINE_ENDING).collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
    }



    let mut answer2 = String::new();
    return answer2;
}
