use std::fs;

/*
    Advent of Code 2022: Day 1
        part1 answer: 71124
        part2 answer: 204639
 */

fn main() {
    print!("Advent of Code 2022, ");
    println!("Day 01\n");

    let input1_filename = "data/day01/part1_input.txt";
    println!("input filename: {input1_filename}");
    let data1 = fs::read_to_string(input1_filename).expect(&*format!("error opening file {}", input1_filename));
    let mut lines: Vec<&str> = data1.split("\n").collect();
    println!("read {} lines\n", lines.len());
    lines.push("\n");

    let l_number = lines.len();
    let mut totals: Vec<i32> = Vec::new();
    let mut c_total = 0;

    for i in 0..l_number {
        let ln = lines[i];
        let l_num = ln.trim().parse::<i32>();
        let tot = match l_num {
            Ok(n) => n,
            Err(_) => 0
        };
        if tot == 0 {
            totals.push(c_total);
            c_total = 0;
        } else {
            c_total += tot
        }
    }

    let mut sorted: Vec<i32> = totals.to_vec();
    sorted.sort();
    sorted.reverse();

    let part1_answer: i32 = sorted[0];
    let part2_answer: i32 = sorted[0] + sorted[1] + sorted[2];

    println!("\t Part 1: {part1_answer}");
    println!("\t Part 2: {part2_answer}");
}
