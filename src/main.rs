#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::collections::{HashMap, LinkedList, VecDeque};
use std::collections::HashSet;
use std::{cmp, fmt};
use std::fmt::{Display, Formatter};
use std::fs;
use std::time::{Instant,Duration};


use parse_display::FromStr;



/*
    Advent of Code 2022: Day 20
        part1 answer: 13289
        part2 answer: 2865721299243
 */

const TEST_ANSWER: (i64, i64) = (3, 1623178306);
const INPUT_ANSWER: (i64, i64) = (13289, 2865721299243);

const PART1_TEST_FILENAME: &str = "data/day20/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day20/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day20/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day20/part2_input.txt";

const DECRYPTION_KEY:(i64,i64) = (1,811589153);
const ITERATIONS:(usize,usize) = (1,10);

const TEST: bool = true;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("20");                           // insert Day


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
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";


struct CList {
    vec:Vec<i64>,
    size:usize,
    wrap:i64,
}

impl Display for CList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut sb = String::new();
        sb.push('[');
        if self.size > 1 {
            for i in 0..(self.wrap as usize) {
                sb.push_str(self.vec[i].to_string().as_str());
                sb.push(',');
                sb.push(' ');
            }
            sb.push_str(self.vec[self.vec.len()-1 as usize].to_string().as_str());
        } else {
            if self.size == 1 {
                sb.push_str(self.vec[0].to_string().as_str());
            }
        }
        sb.push(']');
        return write!(f, "{}", sb);
    }
}
impl CList {
    fn new(v:&Vec<i64> ) -> Self {
        CList {
            vec: v.clone(),
            size: v.len(),
            wrap: (v.len() - 1) as i64,
        }
    }

    fn pop_front(&mut self) -> i64 {
        let r = self.vec.remove(0);
        return r;
    }

    fn size(&self) -> usize {
        return self.vec.len();
    }
    fn insert(&mut self, index:i64, value:i64) {
        let mut new_index = index;
        if index.abs() > self.wrap  {
            if index < 0 {
                let pos_index = index.abs();
                let w_index = index % self.wrap;
                let new_neg_index = -1 * w_index;
                let new_index = self.wrap + new_neg_index;
                println!("negative index: {index} , converted to {new_index}");
            } else {
                new_index = index % self.wrap;
            }
        }
        assert!(new_index >= 0);
        assert!(new_index < self.size as i64);

        self.vec.insert(new_index as usize,value);


    }
}

#[derive(Clone)]
struct Number {
    original_index: usize,
    move_by: i64
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.original_index == other.original_index
    }
}

fn shift_element<T>(vec    : &Vec<T>,
                    index  : usize,
                    offset : i64) -> Vec<T>
where T:Clone+PartialEq
{
    let len = vec.len() as i64;

    // concat three copies of the list
    let tripled = [vec.clone(), vec.clone(), vec.clone()].concat();

    //wrap offset into tripled lists
    // just mods by len-1, but does witchcraft to handle mod of negative value
    let offset = (offset % (len-1) + len) % len
                        + if offset > 0 {1}
                        else {0};

    //build return vector
    [
    // make vec of target value
        vec![vec[index].clone()],
        tripled.into_iter()
            .skip( index +offset as usize)
            .filter(|el| *el != vec[index])
            .take(vec.len() -1)
            .collect()
        ].concat()
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
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}\n", lines[0].len());
        }
    }

    let decryption_key:i64 = DECRYPTION_KEY.0;
    let iterations:usize = ITERATIONS.0;
    let  orig_numbers:Vec<i64> = lines.into_iter().map(|n| n.parse::<i64>().unwrap()).collect();

    let mut seq :Vec<Number> = orig_numbers.iter()
        .enumerate()
        .map(|(original_index, &move_by)|
            Number {
                original_index,
                move_by: move_by * decryption_key
            })
        .collect();
  //  println!("seq: {:?}", seq);

    let len = seq.len();

    for _ in 0..iterations {
        for i in 0..len {
            let index = seq.iter().position(
                |num| num.original_index == i ).unwrap();

            let offset = seq[index].move_by;
            let shifted = shift_element(&seq, index, offset);
            seq = shifted;
        }
    }

    let index_zero = seq.iter()
        .position(|num| num.move_by == 0).unwrap();

   let t =  seq[(index_zero + 1000) % len].move_by
        + seq[(index_zero + 2000) % len].move_by
        + seq[(index_zero + 3000) % len].move_by;


    let answer1 = t.to_string();
    return answer1.to_string();
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
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }


    let decryption_key:i64 = DECRYPTION_KEY.1;
    let iterations:usize = ITERATIONS.1;
    let  orig_numbers:Vec<i64> = lines.into_iter().map(|n| n.parse::<i64>().unwrap()).collect();

    let mut seq :Vec<Number> = orig_numbers.iter()
        .enumerate()
        .map(|(original_index, &move_by)|
            Number {
                original_index,
                move_by: move_by * decryption_key
            })
        .collect();
    //  println!("seq: {:?}", seq);

    let len = seq.len();

    for _ in 0..iterations {
        for i in 0..len {
            let index = seq.iter().position(
                |num| num.original_index == i ).unwrap();

            let offset = seq[index].move_by;
            let shifted = shift_element(&seq, index, offset);
            seq = shifted;
        }
    }

    let index_zero = seq.iter()
        .position(|num| num.move_by == 0).unwrap();

    let t =  seq[(index_zero + 1000) % len].move_by
        + seq[(index_zero + 2000) % len].move_by
        + seq[(index_zero + 3000) % len].move_by;


    let mut answer2  = t.to_string();
    return answer2;
}
