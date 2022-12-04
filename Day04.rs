use std::fs;
use std::time::Instant;

/*
    Advent of Code 2022: Day 04
        part1 answer: 657
        part2 answer: 938
 */

const TEST: bool = false;

const PART1_TEST_FILENAME: &str = "data/day04/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day04/part1_input.txt";
const PART2_TEST_FILENAME: &str = "data/day04/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day04/part2_input.txt";


fn main() {
	print!("Advent of Code 2022, ");
	println!("Day 04");

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


fn parse_dash_pair(s: &str) -> (i32, i32) {
	let (l, r) = s.split_once("-").unwrap();
	let l_n: i32 = l.parse().unwrap();
	let r_n: i32 = r.parse().unwrap();
	return (l_n, r_n);
}

fn in_range(r1: i32, r2: i32, t1: i32, t2: i32) -> bool {
	if (r1 <= t1) && (t2 <= r2) {
		return true; // second inside first
	}
	if (t1 <= r1) && (r2 <= t2) {
		return true; // first inside second
	}
	return false;
}

fn do_range_overlap(r1: i32, r2: i32, t1: i32, t2: i32) -> bool {
	if (r1 <= t2) && (t1 <= r2) {
		return true;
	}
	return false;
}


fn part1() -> i32 {
	let p1_file = match TEST {
		true => PART1_TEST_FILENAME,
		false => PART1_INPUT_FILENAME
	};
	let data1_s =
	    fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
	let lines: Vec<&str> = data1_s.trim().split("\n").collect();
	let l_num = lines.len();
	if TEST { println!("\t read {} lines from {}", l_num, p1_file); }

	let mut answer1 = 0;
	for ln in lines {
		let ln = ln.trim();
		let (left, right) = ln.split_once(",").unwrap();
		let (f1, f2) = parse_dash_pair(left);
		let (s1, s2) = parse_dash_pair(right);
		let oo = in_range(f1, f2, s1, s2);
		if oo { answer1 += 1; }
	}

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
	for ln in lines {
		let ln = ln.trim();
		let (left, right) = ln.split_once(",").unwrap();
		let (f1, f2) = parse_dash_pair(left);
		let (s1, s2) = parse_dash_pair(right);

		let oo = do_range_overlap(f1, f2, s1, s2);
		if oo { answer2 += 1; }
	}

	return answer2;
}
