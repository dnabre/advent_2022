#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]




use std::cmp::Ordering;
use std::fs;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Instant;


use serde_json::{json, Value};
use serde_json::Value::{Number,Array};

use itertools::Itertools;
/*
    Advent of Code 2022: Day 13
        part1 answer: 140
        part2 answer: 24948

 */


const TEST_ANSWER: (i32, i32) = (13, 140);
const INPUT_ANSWER: (i32, i32) = (5882, 24948);


const PART1_TEST_FILENAME: &str = "data/day13/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day13/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day13/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day13/part2_input.txt";

const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("13");

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

#[cfg(windows)]
const D_LINE_ENDING: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const D_LINE_ENDING: &'static str = "\n\n";

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";


fn compare_vec(v1:&Vec<Value>, v2:&Vec<Value>) -> Ordering {
    for (l1, r1) in v1.iter().zip(v2) {
        let cmp = compare_values(l1,r1);
        if cmp != Ordering::Equal {
            return cmp;
        }
    }
    // zip arrays are equal, go by length

    return v1.len().partial_cmp(&v2.len()).unwrap();
}


fn compare_values(left:&Value, right:&Value) -> Ordering {
    let p = (&left,&right);
    match p {
        (Value::Number(left_num), Value::Number(right_num)) => {
            let l_i64 = left_num.as_i64().unwrap();
            let r_i64 = right_num.as_i64().unwrap();
            // println!("\t comparing numbers: {:?} {:?} : {}", l_i64, r_i64, l_i64 <= r_i64);
            if l_i64 == r_i64 { return Ordering::Equal}
            if l_i64 < r_i64 { return Ordering::Less} else { return Ordering::Greater};
        }
        ,
        (Value::Number(left_num), Value::Array(right_a)) => {
            let new_list = json!([left_num]);
            // println!("\t comparing new list, list: {:?} {:?} ", new_list, right);
            return compare_values(&new_list,&right)
        },
        (Value::Array(left_a), Value::Number(right_num)) => {
            let new_list = json!([right_num]);
            // println!("\t comparing list, new list: {:?} {:?} ",  right, new_list);
            return  compare_values(&left, &new_list)
        },
        (Value::Array(left_a), Value::Array(right_a)) => {
            return compare_vec(&left_a,&right_a);
        },
        _ => panic!("not sure what to do with {:?}", p)
    };

}

fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };

    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let mut lines: Vec<&str> = data1_s.trim().split("\n").map(|t| t.trim()).collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
    }
    let s = fs::read_to_string(p1_file).unwrap();
    let vs:Vec<Value> =  s.lines()
                            .filter(|l| !l.is_empty())
                            .map(|line| serde_json::from_str(line).unwrap()).collect();

    // println!("{:?}", vs);

    let v_pairs = vs.chunks(2).map(|c| c.to_vec()).collect_vec();
    // println!("{:?}", v_pairs);

    let mut counter = 0;
    let mut good_counter=0;
    let mut ok=true;
    let mut good_indices:Vec<i32> = Vec::new();
    let mut index_sum = 0;
    for p in v_pairs {
        let (a,b) = (p[0].clone(), p[1].clone());
        counter += 1;

        // println!("pair #{counter}:");
        // println!("\t a: {:?}",a);
        // println!("\t b: {:?}",b);
        // println!("==========");

         match compare_values(&a,&b) {
            Ordering::Less => {

                good_indices.push(counter);

            }
            Ordering::Equal => {
                good_indices.push(counter);


            }
            Ordering::Greater => {

                }
        }



    }
    // println!();
    // println!("good indices: {:?}", good_indices);
    let sum:i32 = good_indices.iter().sum();
    let mut answer1 = sum.to_string();
    return answer1;
}


fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));

    let mut lines: Vec<&str> = data2_s.trim().split("\n").map(|t| t.trim()).collect();
    let l_num = lines.len();

    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
    }

    let s = fs::read_to_string(p2_file).unwrap();
    let mut vs:Vec<Value> =  s.lines()
        .filter(|l| !l.is_empty())
        .map(|line| serde_json::from_str(line).unwrap()).collect();
    let packet_2 =serde_json::to_value(vec![vec![2]]).unwrap();
    let packet_6 =serde_json::to_value(vec![vec![6]]).unwrap();
    vs.push( packet_2.clone());
    vs.push(packet_6.clone());

    vs.sort_by(|v1,v2| compare_values(v1,v2));

    let mut p2_index:usize=0;
    let mut p6_index:usize=0;
    let mut index = 1;
    for v in &vs {
       // println!("{:?}", v);
        if v.eq(&packet_2){
            p2_index = index;
        };
        if v.eq(&packet_6) {
            p6_index = index;
        }
        index += 1;
    }
    println!("packet 2 index: {}", p2_index);
    println!("packet 6 index: {}", p6_index);




    let mut answer2 = (p2_index * p6_index).to_string();
    return answer2;
}
