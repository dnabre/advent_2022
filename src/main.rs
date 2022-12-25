#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::collections::HashSet;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs;
use std::time::Instant;



/*
    Advent of Code 2022: Day 25

        part1 answer:
        part2 answer:

part1?  34182852926025

 */


const TEST_ANSWER: (&str,&str)= ("2=-1=0", "");
const INPUT_ANSWER: (&str,&str) = ("2-0-01==0-1=2212=100", "");

const PART1_TEST_FILENAME: &str = "data/day25/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day25/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day25/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day25/part2_input.txt";


const TEST: bool = true;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("25");                           // insert Day

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
    //
    // println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);
    //
    // // if TEST {
    //     assert_eq!(answer2, TEST_ANSWER.1.to_string());
    // } else {
    //     assert_eq!(answer2, INPUT_ANSWER.1.to_string());
    // }

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


fn map_ch_to_value(ch:u8) -> i8 {
    match ch {
        b'2' => {2 }
        b'1' => {1 }
        b'0' => {0 }
        b'-' => {-1 }
        b'=' => {-2 }
        x =>{ panic!("bad SNAFU-it, {}", x)}
    }
}

fn map_value_to_ch(i:i8) -> char {
    match i {
        2 => {'2'  }
        1  => {'1' }
        0  => {'0'}
        -1 => {'-' }
        -2 => {'=' }
        x =>{ panic!("bad SNAFU-it, {}", x)}
    }



}


fn snafu5_to_decimal(input_string:&mut  String) -> i64 {
    let  bbytes = input_string.as_bytes();
    let mut bytes:Vec<u8> = Vec::from(bbytes);
    bytes.reverse();
  //  println!("\tbytes   : {:?}", bytes);
    let mut f_value:i64 = 0;

    for i in 0..bbytes.len() {
        f_value *= 5;
        let b = bytes.pop().unwrap();

        let f = map_ch_to_value(b);

        f_value += f as i64;


    }

    return f_value;
}

fn decimal_to_snafu5(dec:i64) -> String {
    let mut sb:Vec<char> = Vec::new();


    let mut total = dec;
    while total >0 {
        let fit = ((total +  2 ) % 5) -2;
        let ch =map_value_to_ch(fit as i8);
        sb.push(ch);
        total = (total - fit) /5 ;

    }
sb.reverse();
    let mut snafu = String::new();
    for ch in sb {
        snafu.push(ch);
    }


    return snafu;
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
    }

    let mut dec_values:Vec<i64> = Vec::new();
    let mut snafu5_values:Vec<String> = Vec::new();

    let mut sum:i64 = 0;
    for ln in lines {

        let mut s = String::from(ln);
        let val:i64 = snafu5_to_decimal(&mut s.clone());
        let and_back = decimal_to_snafu5(val);
        let ok = and_back.eq(ln);

        print!("\t SNAUF  : {:>20}", s);
        print!("\t decimal: {:>20}", val);
        println!("\t SNAUF  :  {:>20} \t{}", and_back, ok);
        sum += val;
        dec_values.push(val);
        snafu5_values.push(s);


    }




    let answer1 = decimal_to_snafu5(sum);
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

    }





    let mut answer2 =String::new();
    return answer2;
}
