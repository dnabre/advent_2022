#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::fs;
use std::collections::HashSet;
use std::time::{Duration, Instant};

/*
    Advent of Code 2022: Day 05
        part1 answer:
        part2 answer:
 */

use parse_display::FromStr;

#[cfg(windows)]
const D_LINE_ENDING: &'static str ="\r\n\r\n";
#[cfg(not(windows))]
const D_LINE_ENDING: &'static str ="\n\n";

#[derive(FromStr, Debug)]
#[display("move {count} from {src} to {dest}")]
struct Move {
	count: i32,
	src: i32,
	dest: i32,
}


const TEST: bool = true;

const PART1_TEST_FILENAME: &str = "data/day05/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day05/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day05/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day05/part2_input.txt";


fn main() {
	print!("Advent of Code 2022, ");
	println!("Day 05");

	let start1 = Instant::now();
	let answer1 = part1();
	let duration1 = start1.elapsed();

	let start2 = Instant::now();
	let answer2 = part2();
	let duration2 = start2.elapsed();

	println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);
	println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);

	println!("----------\ndone");
}



fn part1() -> i32 {
	let p1_file = match TEST {
		true => PART1_TEST_FILENAME,
		false => PART1_INPUT_FILENAME
	};
	let data1_s =
	    fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));


	//let lines: Vec<&str> = data1_s.trim().split("\n").collect();
	//let l_num = lines.len();
	//if TEST { println!("\t read {} lines from {}", l_num, p1_file); }


	let (initial,moves) = ("","");

	let (initial, moves) = data1_s.split_once(D_LINE_ENDING).unwrap();

	println!("initial:\n{}", initial);

	println!("---------");

	println!("moves:\n{}", moves );

	/*
	let moves: Vec<_> = moves
		.lines()
		.map(|line| line.parse::<Move>().unwrap())
		.collect();
*/




	let mut answer1 = 0;
	return answer1;
}

fn part2() -> i32 {
	let p2_file = match TEST {
		true => PART2_TEST_FILENAME,
		false => PART2_INPUT_FILENAME
	};
	let data2_s =
	    fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));
	let lines: Vec<&str> = data2_s.trim().split("\n").collect();
	let l_num = lines.len();
	if TEST { println!("\t read {} lines from {}", l_num, p2_file); }

	let mut answer2 = 0;

	return answer2;
}
