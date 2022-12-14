use std::cmp::Ordering;
use std::fmt::Debug;
use std::fs;
use std::time::Instant;

use serde_json::{json, Value};

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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    packet: Value,
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.packet, &other.packet) {
            (Value::Number(left_num), Value::Number(right_num)) => {
                let l_i64 = left_num.as_i64().unwrap();
                let r_i64 = right_num.as_i64().unwrap();
                return l_i64.cmp(&r_i64);
            }
            (Value::Number(left_num), Value::Array(_)) => {
                let p_new_list = Packet { packet: json!([left_num]) };
                return p_new_list.cmp(other);
            }
            (Value::Array(_), Value::Number(right_num)) => {
                let p_new_list = Packet { packet: json!([right_num]) };
                return self.cmp(&p_new_list);
            }
            (Value::Array(left_a), Value::Array(right_a)) => {
                for (p_l, p_r) in left_a.iter().map(|p| Packet { packet: p.clone() })
                                        .zip(right_a.iter().map(|p| Packet { packet: p.clone() })) {
                    let cmp = p_l.cmp(&p_r);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                return left_a.len().partial_cmp(&right_a.len()).unwrap();
            }
            _ => panic!("not sure what to do with {:?}", (self, other))
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn string_to_packet_vec(s: &String) -> Vec<Packet> {
    return s.lines()
        .filter(|l| !l.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .map(|v| Packet { packet: v })
        .collect();
}

fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };

    let s = fs::read_to_string(p1_file).unwrap();
    let packet_vec = string_to_packet_vec(&s);
    if TEST {
        println!("\t read {} lines from {}", packet_vec.len(), p1_file);
    }

    let mut index = 0;
    let mut good_index_sum:i32 = 0;
    for p in packet_vec.chunks(2) {
        let (a, b) = (p[0].clone(), p[1].clone());
        index += 1;
        let left_pack = a;
        let right_pack = b;
        if left_pack <= right_pack {
            good_index_sum += index;
        }
    }
    let answer1 = good_index_sum.to_string();
    return answer1;
}

fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };

    let s = fs::read_to_string(p2_file).unwrap();
    let mut packet_vec = string_to_packet_vec(&s);
    if TEST {
        println!("\t read {} lines from {}", packet_vec.len(), p2_file);
    }

    let packet_2 = Packet { packet: serde_json::to_value(vec![vec![2]]).unwrap() };
    let packet_6 = Packet { packet: serde_json::to_value(vec![vec![6]]).unwrap() };
    packet_vec.push(packet_2.clone());
    packet_vec.push(packet_6.clone());
    packet_vec.sort();

    let mut p2_index: usize = 0;
    let mut p6_index: usize = 0;
    let mut index = 1;
    for p in packet_vec {
        if p.eq(&packet_2) {
            p2_index = index;
        }
        if p.eq(&packet_6) {
            p6_index = index;
        }
        index += 1;
    }

    let answer2 = (p2_index * p6_index).to_string();
    return answer2;
}
