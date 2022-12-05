
use std::fs;
use std::time::{Instant};

use parse_display::FromStr;

/*
    Advent of Code 2022: Day 05
        part1 answer:   VRWBSFZWM
        part2 answer:   RBTWJWMCF
 */

#[cfg(windows)]
const D_LINE_ENDING: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const D_LINE_ENDING: &'static str = "\n\n";

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";


#[derive(FromStr, Debug)]
#[display("move {count} from {src} to {dest}")]
struct Move {
    count: usize,
    src: usize,
    dest: usize,
}


const TEST: bool = false;

const PART1_TEST_FILENAME: &str = "data/day05/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day05/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day05/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day05/part2_input.txt";
const MAX_STACK: usize = 9;


fn main() {
    print!("Advent of Code 2022, ");
    println!("Day 05");

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

fn parse_moves(moves: &str) -> Vec<Move> {
    let mut moves_v: Vec<Move> = Vec::new();
    let move_lines = moves.lines();
    for ln in move_lines {
        let m: Move = ln.parse().unwrap();
        moves_v.push(m);
    }
    return moves_v;
}

fn parse_inital_stack(initial: &str, stacks: &mut [Vec<char>; MAX_STACK + 1]) {
    let mut max_len_length = 0;
    let init_lines = initial.split(LINE_ENDING);
    let mut s_lines: Vec<&str> = Vec::new();
    for l in init_lines {
        let r = l.trim_end();
        if r.len() > max_len_length { max_len_length = r.len(); }
        s_lines.push(r);
    }
    s_lines.pop();
    let mut padded: Vec<String> = Vec::new();
    for l in s_lines {
        format!("{}", l);
        let r = format!("{:<width$} ", l, width = max_len_length);
        padded.push(r);
    }

    let num_lines = padded.len();

    for _i in 0..num_lines {
        let ln_string = padded.pop().unwrap();
        let mut ln = ln_string.as_str();
        let mut stack_number = 1;
        while ln.len() > 0 {
            let (left, rest) = ln.split_at(4);
            ln = rest;
            let ch = left.chars().nth(1).unwrap();
            if ch != ' ' {
                stacks[stack_number].push(ch);
            }
            stack_number += 1;
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
    if TEST {
        let lines: Vec<&str> = data1_s.trim().split("\n").collect();
        let l_num = lines.len();
        println!("\t read {} lines from {}", l_num, p1_file);
    }

    let (initial, moves) = data1_s.split_once(D_LINE_ENDING).unwrap();
    let mut stacks: [Vec<char>; MAX_STACK + 1] = Default::default();

    parse_inital_stack(initial, &mut stacks);
    let moves_v: Vec<Move> = parse_moves(moves);

    // Run moves
    for mut m in moves_v
    {
        while m.count > 0 {
            let v = stacks[m.src].pop().unwrap();
            stacks[m.dest].push(v);
            m.count -= 1;
        }
    }

    let mut answer1 = String::new();
    for i in 1..=MAX_STACK {
        let ch = stacks[i].pop().unwrap();
        answer1.push(ch);
    }
    return answer1;
}


fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));

    if TEST {
        let lines: Vec<&str> = data2_s.trim().split("\n").collect();
        let l_num = lines.len();
        println!("\t read {} lines from {}", l_num, p2_file);
    }

    let (initial, moves) = data2_s.split_once(D_LINE_ENDING).unwrap();
    let mut stacks: [Vec<char>; MAX_STACK + 1] = Default::default();

    parse_inital_stack(initial, &mut stacks);
    let moves_v: Vec<Move> = parse_moves(moves);

    // Run moves
    for  m in moves_v
    {
        let mut buffer: Vec<char> = Vec::new();
        // pick up m.count items from stack m.src, reversing order
        for _c in 0..m.count {
            let v = stacks[m.src].pop().unwrap();
            buffer.push(v);
        }
        // push on m.count items from buffer to stack m.dest, reversing the order back to forward
        for _c in 0..m.count {
            let v: char = buffer.pop().unwrap();
            stacks[m.dest].push(v);
        }
    }

    let mut answer2 = String::new();
    for i in 1..=MAX_STACK {
        let ch = stacks[i].pop().unwrap();
        answer2.push(ch);
    }
    return answer2;
}
