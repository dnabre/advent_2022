#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;

/*
    Advent of Code 2022: Day 02
        part1 answer: 11063
        part2 answer: 10349
 */


const PART1_TEST_FILENAME: &str  = "data/day02/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day02/part1_input.txt";

const PART2_TEST_FILENAME: &str  = "data/day02/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day02/part2_input.txt";

fn main() {
    print!("Advent of Code 2022, ");
    println!("Day 02");


    let answer1 = part1();
    let answer2 = part2();

    println!("\t Part 1: {answer1} ");
    println!("\t Part 2: {answer2} ");

    println!("----------\ndone");
}

fn mirror(left:char) -> char {
    return match left {
        'A' => 'X',
        'B' => 'Y',
        'C' => 'Z',
        'X' => 'A',
        'Y' => 'B',
        'Z' => 'C',
        _ => '0'
    };
}

fn lose(left:char) -> char {
    return match left {
        'A' => 'Z',
        'B' => 'X',
        'C' => 'Y',
        _ => '0'
    };
}

fn win(left:char) -> char {
    return match left {
        'A' => 'Y',
        'B' => 'Z',
        'C' => 'X',
        _ => '0'
    };
}






fn should_play(left:char,  goal:char) -> char {
    if goal == 'X' {
        //lose
        return lose(left);
    }
    if goal =='Y' {
        //tie
        return mirror(left);
    }
    if goal =='Z' {
        //win
        return win(left);
    }
    panic!("unknown goal: {left}");
}


fn score(left:char, right:char)  -> i32 {
    if (left == 'A' && right == 'X') ||
        (left == 'B' && right == 'Y') ||
        (left == 'C' && right == 'Z')
    {
        //tie
        return 3 + value(left);
    }
    if (left == 'A') && (right == 'Z') {
        // lose Rock beats Scissors
        return value(right);
    }
    if (left == 'B') && (right == 'X') {
        // lose Paper beats Rock
        return value(right);
    }

    if (left == 'C') && (right == 'Y') {
        // lose Scisorrs beats
        return value(right);
    }
    // otherwise we won
    return value(right) + 6;
}


fn value(ch:char) -> i32 {
    if  (ch == 'A') || (ch == 'X') {
        return 1;
    } else if (ch == 'B') || (ch == 'Y') {
        return 2;
    }  else if  (ch == 'C') || (ch == 'Z') {
        return 3;
    }
    return -1;
}



fn part1()->i32 {
    let p1_file = PART1_INPUT_FILENAME;
    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}",p1_file));
    let lines: Vec<&str> = data1_s.split("\n").collect();
    // println!("\t read {} lines from {}", lines.len(), p1_file);
    let l_num = lines.len();
    let mut answer1: i32 = 0;
    for ln in lines {
        //println!("{}", ln);
        let mut s = ln.split_ascii_whitespace().take(2);
        let left = s.next().unwrap().chars().nth(0).unwrap();
        let right = s.next().unwrap().chars().nth(0).unwrap();

        let sc = score(left, right);
        answer1 += sc;
        //   println!("input  \t : {} \t : {} \tscore: {}", left, right, sc);
    }
    return answer1;
}

fn part2() -> i32 {
    let p1_file = PART2_INPUT_FILENAME;
    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}",p1_file));
    let lines: Vec<&str> = data1_s.split("\n").collect();
    //  println!("\t read {} lines from {}", lines.len(), p1_file);
    let l_num = lines.len();
    let mut answer1: i32 = 0;
    for ln in lines {
        //println!("{}", ln);
        let mut s = ln.split_ascii_whitespace().take(2);
        let left = s.next().unwrap().chars().nth(0).unwrap();
        let right = s.next().unwrap().chars().nth(0).unwrap();
        let play:char = should_play(left,right);

        let sc = score(left,play );

        answer1 += sc;
        //         println!("input  \t : {}  _  {} -> {} \tscore: {}", left, right, play, sc);
    }
    return answer1;

}
