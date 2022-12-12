use std::collections::HashSet;
use std::fs;

/*
    Advent of Code 2022: Day 03
        part1 answer: 7674
        part2 answer: 2805
 */

const TEST: bool = false;

const PART1_TEST_FILENAME: &str = "data/day03/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day03/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day03/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day03/part2_input.txt";

fn main() {
    print!("Advent of Code 2022, ");
    println!("Day 03");

    let answer1 = part1();
    let answer2 = part2();

    println!("\t Part 1: {answer1} ");
    println!("\t Part 2: {answer2} ");

    println!("----------\ndone");
}

fn common_in_group(s_a: &str, s_b: &str, s_c: &str) -> char {
    let h_a = string_to_hashset(s_a);
    let h_b = string_to_hashset(s_b);
    let h_c = string_to_hashset(s_c);

    let sets = [h_a, h_b, h_c];

    // from https://github.com/rust-lang/rfcs/issues/2023
    let intersection = sets
        .iter()
        .skip(1)
        .fold(sets[0].clone(), |acc, hs| {
            acc.intersection(hs).cloned().collect()
        });
    let i_vec: Vec<&char> = intersection.iter().collect();

    return *i_vec[0];
}

fn get_misplaced_item(s: &str) -> char {
    let s_len = s.len();
    let middle = s_len / 2;

    let mut left: Vec<char> = s.chars().collect();
    let right = left.split_off(middle);

    let l_set: HashSet<char> = HashSet::from_iter(left);
    let r_set: HashSet<char> = HashSet::from_iter(right);

    let problem: Vec<&char> = l_set.intersection(&r_set).collect();

    return *problem[0];
}

fn string_to_hashset(s: &str) -> HashSet<char> {
    let h: HashSet<char> = s.chars().collect();
    return h;
}

fn get_priority(ch: char) -> i32 {
    if ch.is_ascii_lowercase() {
        return (ch.to_ascii_lowercase() as i32) - ('a' as i32) + 1;
    } else {
        return (ch.to_ascii_uppercase() as i32) - ('A' as i32) + 27;
    }
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
        let misplaced: char = get_misplaced_item(ln);
        let priority: i32 = get_priority(misplaced);
        answer1 += priority;
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

    let mut total = 0;
    let mut current = 0;
    while current < l_num {
        let (a, b, c): (&str, &str, &str) = (lines[current].trim(), lines[current + 1].trim(),
                                             lines[current + 2].trim());
        current += 3;
        let ch: char = common_in_group(a, b, c);
        let pr: i32 = get_priority(ch);
        total += pr;
    }

    let answer2 = total;
    return answer2;
}
