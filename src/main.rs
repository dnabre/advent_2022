use std::fs;

fn main() {
    print!("Advent of Code 2022, ");
    println!("Day 01\n");

    let input1_filename = "data/day01/part1_input.txt";
    println!("input filename: {input1_filename}");
    let data1 = fs::read_to_string(input1_filename).expect(&*format!("error opening file {}",input1_filename));
    let lines: Vec<&str>  = data1.split("\n").collect();
    println!("read {} lines\n", lines.len());

    let mut totals:Vec<i32> = Vec::new();

    let mut count:i32 = 0;
    let mut c_total:i32 = 0;
    for ln in lines {

        count+=1;

        let l_num = ln.trim().parse::<i32>();
        let tot = match l_num {
            Ok(n) => n,
            Err(error) => 0
        };
        if(tot == 0) {
            totals.push(c_total);
            c_total = 0;
        } else {
            c_total += tot;
        }
       // println!{"line {}: {} \t parsed: {}, c_total: {}", count, ln, tot, c_total};
    }
    println!("number of groups: {}", totals.len());
    let mut max:i32 = 0;
    for i in totals {
        if(i > max)  {
            max = i;
        }
    }
    println!("Highest Calorie group total: {max}");



    println!("\ndone");
}

/*
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

 */

/*
fn main() {
    let filename = "src/input.txt";
    let data = read_input(&filename);
    let trimmed = data.trim();
    let split_data: Vec<&str> = trimmed.split("\n\n").collect();
 */