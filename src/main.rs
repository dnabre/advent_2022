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
        print!("{}\t:", i);
        let ln = lines[i];
        println!("{}", ln);

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

        println!("\t line tot: {}" ,tot) ;
     /*
        if i == (l_number - 1) {
            println!("ending c_total: {}", c_total);
            totals.push(c_total);
        }

      */
    }




    let mut sorted:Vec<i32> = totals.to_vec();
    sorted.sort();
    sorted.reverse();

    let part1_answer:i32 = sorted[0];
    let part2_answer:i32 = sorted[0] + sorted[1] + sorted[2];

    for i in sorted {
        println!("{}", i);
    }

    println!("\t Part 1: {part1_answer}");
    println!("\t Part 2: {part2_answer}");

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