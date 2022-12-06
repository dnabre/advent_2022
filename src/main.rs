use std::fs;
use std::time::Instant;

/*
    Advent of Code 2022: Day 06
        part1 answer: 1109
        part2 answer: 3955
 */

const TEST: bool = false;

const PART1_TEST_FILENAME: &str = "data/day06/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day06/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day06/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day06/part2_input.txt";

const PART1_MARKER_SIZE: usize = 4;
const PART2_MARKER_SIZE: usize = 14;

fn main() {
    print!("Advent of Code 2022, Day 06");
    println!("");                           // insert Day

    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();
    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);

    let start2 = Instant::now();
    let answer2 = part2();
    let duration2 = start2.elapsed();
    println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);

    println!("----------\ndone");
}



fn check_unique(slice: &[char]) -> bool {
    let end = slice.len();
    for i in 0..end {
        let ch = slice[i];
        for j in i + 1..end {
            if ch == slice[j] { return false; }
        }
    }
    return true;
}

fn index_of_first_marker(s: &str, marker_size: usize) -> usize {
    let bytes: Vec<char> = s.chars().collect();
    for start in 0..=bytes.len() {
        let end = start + marker_size;
        if end > bytes.len() { break; }
        let sl = &bytes[start..end];
        let is_unique = check_unique(sl);
        if is_unique { return end; }
    }
    return bytes.len();
}

fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };
    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let lines: Vec<&str> = data1_s.trim().split("\n").collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
    }

    let  tests: Vec<&str> = lines.iter().map(|ln| ln.trim()).collect();
    let mut marker = 0;
    for t in &tests {
        marker = index_of_first_marker(t, PART1_MARKER_SIZE);
        if TEST { println!("len: {:>2} ,\t {} \t marker: {}", t.len(), t, marker) };
    }

    let  answer1 = marker.to_string();
    return answer1;
}


fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));

    let lines: Vec<&str> = data2_s.trim().split("\n").collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
    }

    let  tests: Vec<&str> = lines.iter().map(|ln| ln.trim()).collect();
    let mut marker = 0;
    for t in &tests {
        marker = index_of_first_marker(t, PART2_MARKER_SIZE);
        if TEST { println!("len: {:>2} ,\t {} \t marker: {}", t.len(), t, marker) };
    }
    let answer2 = marker.to_string();
    return answer2;
}
