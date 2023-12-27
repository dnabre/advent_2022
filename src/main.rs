#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]


use std::time::Instant;

/*
    Advent of Code 2022: Day 22
        part1 answer:   514786
        part2 answer:

*/
const ANSWER: (&str, &str) = ("514786", "Button Pressed");

fn main() {
    let _filename_test = "data/day22/test_input_01.txt";
    let _filename_test2 = "data/day22/test_input_02.txt";

    let filename_part1 = "data/day22/part1_input.txt";
    let filename_part2 = "data/day22/part2_input.txt";


    println!("Advent of Code, Day 22");
    println!("    ---------------------------------------------");
    let start1 = Instant::now();
    let answer1 = part1(_filename_test);
    let duration1 = start1.elapsed();

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!(
            "\t\t ERROR: Answer is WRONG. Got: {}, Expected {}",
            answer1, ANSWER.0
        );
    }

    let start2 = Instant::now();
    let answer2 = part2(_filename_test2);
    let duration2 = start2.elapsed();

    println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    if ANSWER.1 != answer2 {
        println!(
            "\t\t ERROR: Answer is WRONG. Got: {}, Expected {}",
            answer2, ANSWER.1
        );
    }
    println!("    ---------------------------------------------");
}


fn part1(input_file: &str) -> String {
    let lines = advent_2022::file_to_lines(input_file);
    println!("\n part1 reading {} lines from {}", lines.len(), input_file);
    for i in 0..lines.len() {

        println!("{i:3}\t len: {:5}", lines[i].len())
    }

    let answer = 0;
    return answer.to_string();
}

fn part2(input_file: &str) -> String {
    let lines = advent_2022::file_to_lines(input_file);
    println!("\n part2 reading {} lines from {}", lines.len(), input_file);
    for i in 0..lines.len() {

        println!("{i:3}\t len: {:5}", lines[i].len())
    }

    let answer = 0;
    return answer.to_string();
}
