#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fs;
use std::time::Instant;
use parse_display::FromStr;

/*
    Advent of Code 2022: Day 11
        part1 answer: 2713310158
        part2 answer: 20151213744

 */


const TEST_ANSWER: (i64, i64) = (10605, 2713310158);
const INPUT_ANSWER: (i64, i64) = (58794, 20151213744);


const PART1_TEST_FILENAME: &str = "data/day11/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day11/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day11/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day11/part2_input.txt";

const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("11");


    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();

    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);

    if TEST {
        assert_eq!(answer1, TEST_ANSWER.0.to_string());
    } else {
        assert_eq!(answer1, INPUT_ANSWER.0.to_string());
    }

    let start2 = Instant::now();
    let answer2 = part2();
    let duration2 = start2.elapsed();

    println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);
    // if TEST {
    //     assert_eq!(answer2, TEST_ANSWER.1.to_string());
    // } else {
    //     assert_eq!(answer2, INPUT_ANSWER.1.to_string());
    //}
    println!("----------\ndone");
}


#[derive(PartialEq, Debug, Copy, Clone)]
enum Operation {
    Mult,
    Add,
    Square,
    None,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op_char = match self {
            Operation::Mult => { '*' }
            Operation::Add => { '+' }
            Operation::Square => { '□' }
            Operation::None => { '?' }
        };

        write!(f, "{}", op_char)
    }
}


#[derive(FromStr, PartialEq, Debug, Clone)]
#[display(
"Monkey {monkey_id}:|Test: divisible by {test_divider}|If true: throw to monkey {true_monkey}|If false: throw to monkey {false_monkey}")]
#[from_str(default)]
struct Monkey {
    monkey_id: usize,
    items: VecDeque<i64>,
    oper_type: Operation,
    oper_amount: i64,
    test_divider: i64,
    true_monkey: usize,
    false_monkey: usize,
    items_inspected: i64,
}


impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op_amount_str = match self.oper_type {
            Operation::Mult => { self.oper_amount.to_string() }
            Operation::Add => { self.oper_amount.to_string() }
            Operation::Square => { String::from("old") }
            Operation::None => {
                panic!("unknown old operator")
            }
        };
        write!(f, "monkey {:<2}:old: {} {:<4}, div: {:<3}, true: {:<2} false: {:<2}, i times: {:6>} items: {:?}",
               self.monkey_id, self.oper_type, op_amount_str,
               self.test_divider, self.true_monkey, self.false_monkey, self.items_inspected, self.items
        )
    }
}

impl Default for Monkey {
    fn default() -> Monkey {
        Monkey {
            monkey_id: 0,
            items: VecDeque::new(),
            oper_type: Operation::None,
            oper_amount: -1,
            test_divider: -1,
            true_monkey: 0,
            false_monkey: 0,
            items_inspected: 0,
        }
    }
}

fn parse_monkeys(mut lines: Vec<&str>) -> Vec<Monkey> {
    let mut monkey: Vec<Monkey> = Vec::new();
    let mut m_line_group: Vec<&str> = Vec::new();
    let mut monkey_parse_buffer = String::new();
    let mut items_line;
    let mut operation_line;
    let mut index = 0;


    while index < lines.len() {
        monkey_parse_buffer.push_str(lines[index]);  // Monkey #:
        monkey_parse_buffer.push('|');
        monkey_parse_buffer.push_str(lines[index + 3]); // Test:..
        monkey_parse_buffer.push('|');
        monkey_parse_buffer.push_str(lines[index + 4]); // If true..
        monkey_parse_buffer.push('|');
        monkey_parse_buffer.push_str(lines[index + 5]); // If false..

        items_line = lines[index + 1];           // Starting items...
        operation_line = lines[index + 2];       // Operation...


        index += 6;

        index += 1;
        let mut m: Monkey = monkey_parse_buffer.parse().unwrap();
        m.items = items_line.split_once(":").unwrap().1
            .split(",").map(|p| p.trim().parse().unwrap()).collect();
        if operation_line.contains("*") {
            m.oper_type = Operation::Mult;
        } else {
            m.oper_type = Operation::Add;
        }
        let op_pair = match m.oper_type {
            Operation::Mult => { operation_line.rsplit_once("*").unwrap() }
            Operation::Add => { operation_line.rsplit_once("+").unwrap() }
            _ => {
                panic!("Monkey's operation should either Mult or Add at this point.")
            }
        };

        let mut op_part = op_pair.1;
        op_part = op_part.trim();
        if op_part.eq("old") {
            m.oper_type = Operation::Square;
        } else {
            m.oper_amount = op_part.parse().unwrap();
        }

        monkey.push(m);
        //  println!("pushed monkey {} after parsing through input {} of {}", mm.monkey_id, index, lines.len());
        monkey_parse_buffer = String::new();
    }
    monkey
}


fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };

    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let mut lines: Vec<&str> = data1_s.trim().split("\n").map(|t| t.trim()).collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
    }


    let mut monkey = parse_monkeys(lines);

   //  let monkey_number = monkey.len();
   //  let item_number:usize = monkey.iter().map(|m| m.items.len()).sum();
   //
   // println!("number of monkeys: {}", monkey_number);
   //  println!("number of items: {}", item_number);
   //  for i in 0..monkey.len() {
   //      println!("monkey[{i:>3}]: {}", monkey[i]);
   //  }

    let number_of_monkeys = monkey.len();
    for round in 1..=20 {
        for i in 0..monkey.len() {
            while !monkey[i].items.is_empty() {
                // println!("Monkey {}:", monkey[i].monkey_id);

                let init_worry: i64 = match monkey[i].items.pop_front() {
                    None => {
                        panic!("attempted to inspect on monkey without items: {}", monkey[i]);
                    }
                    Some(x) => { x }
                };
                // println!(" Monkey inspects an item with worry level of {init_worry}.");
                monkey[i].items_inspected +=1;

                let mut new_worry = match monkey[i].oper_type {
                    Operation::Mult => { init_worry * monkey[i].oper_amount }
                    Operation::Add => { init_worry + monkey[i].oper_amount }
                    Operation::Square => { init_worry * init_worry }
                    Operation::None => {
                        panic!("monkey shouldn't have Operation::None, {}", monkey[i]);
                    }
                };

                // println!("    Worry level is {} to {}.",
                //          match monkey[i].oper_type {
                //              Operation::Mult => { format!("multiplied by {}", monkey[i].oper_amount) }
                //              Operation::Add => { format!("increased by {}", monkey[i].oper_amount) }
                //              Operation::Square => { format!("multiplied by itself") }
                //              Operation::None => { panic!("shouldn't have Operation::None, {}", monkey[i]) }
                //          }, new_worry);

                new_worry = new_worry / 3;
                // println!("    Monkey gets bored with item. Worry level is divided by 3 to {new_worry}.");
                if new_worry % monkey[i].test_divider != 0 {
                    // println!("    Current worry level is not divisible by {} (false)", monkey[i].test_divider);
                    // println!("    Item with worry level {new_worry} is thrown to monkey {}.",monkey[i].true_monkey);
                    let t_index = monkey[i].false_monkey;
                    monkey[t_index].items.push_back(new_worry);
                } else {
                    let t_index = monkey[i].true_monkey;
                    // println!("    Current worry level is divisible by {} (true).", monkey[i].test_divider);
                    monkey[t_index].items.push_back(new_worry);
                }
            }
        }
    }
    let mut i_vec = Vec::new();
    for i in 0..monkey.len() {
    //    println!("{}", monkey[i]);
        i_vec.push(monkey[i].items_inspected);
    }
    i_vec.sort();
    i_vec.reverse();
    let answer1_i = i_vec[0] * i_vec[1];
   // println!("{:?}",i_vec);
    let mut answer1 = answer1_i.to_string();
    return answer1;
}

fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));

    let mut lines: Vec<&str> = data2_s.trim().split("\n").map(|t| t.trim()).collect();
    let l_num = lines.len();

    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
    }
    let mut monkey = parse_monkeys(lines);
    let check_rounds = vec![1,20,1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000];
    // println!("number of monkeys: {}", monkey.len());
    // for i in 0..monkey.len() {
    //     println!("monkey[{i:>3}]: {}", monkey[i]);
    // }
    let mod_base:i64= monkey.iter().map(|m| m.test_divider).product();
    println!("using mod_base: {mod_base}");


    let number_of_monkeys = monkey.len();
    for round in 1..=10000 {
        for i in 0..monkey.len() {
            while !monkey[i].items.is_empty() {
                // println!("Monkey {}:", monkey[i].monkey_id);

                let init_worry: i64 = match monkey[i].items.pop_front() {
                    None => {
                        panic!("attempted to inspect on monkey without items: {}", monkey[i]);
                    }
                    Some(x) => { x }
                } as i64;
                // println!(" Monkey inspects an item with worry level of {init_worry}.");
                monkey[i].items_inspected +=1;

                let mut new_worry = match monkey[i].oper_type {
                    Operation::Mult => { init_worry * monkey[i].oper_amount }
                    Operation::Add => { init_worry + monkey[i].oper_amount }
                    Operation::Square => { init_worry * init_worry }
                    Operation::None => {
                        panic!("monkey shouldn't have Operation::None, {}", monkey[i]);
                    }
                } ;
                new_worry = new_worry % mod_base;
                // println!("    Worry level is {} to {}.",
                //          match monkey[i].oper_type {
                //              Operation::Mult => { format!("multiplied by {}", monkey[i].oper_amount) }
                //              Operation::Add => { format!("increased by {}", monkey[i].oper_amount) }
                //              Operation::Square => { format!("multiplied by itself") }
                //              Operation::None => { panic!("shouldn't have Operation::None, {}", monkey[i]) }
                //          }, new_worry);

             //   new_worry = new_worry / 3;
                // println!("    Monkey gets bored with item. Worry level is divided by 3 to {new_worry}.");
                if new_worry % monkey[i].test_divider != 0 {
                    // println!("    Current worry level is not divisible by {} (false)", monkey[i].test_divider);
                    // println!("    Item with worry level {new_worry} is thrown to monkey {}.",monkey[i].true_monkey);
                    let t_index = monkey[i].false_monkey;
                    monkey[t_index].items.push_back(new_worry);
                } else {
                    let t_index = monkey[i].true_monkey;
                    // println!("    Current worry level is divisible by {} (true).", monkey[i].test_divider);
                    monkey[t_index].items.push_back(new_worry);
                }
            }
        }
        // if check_rounds.contains(&round) {
        //     println!("== After round {round} ==");
        //     for i in 0..monkey.len() {
        //         println!("Monkey {i} inspected items {} times.", monkey[i].items_inspected);
        //     }
        //     println!();
        // }
    }
    let mut i_vec = Vec::new();
    for i in 0..monkey.len() {
        //    println!("{}", monkey[i]);
        i_vec.push(monkey[i].items_inspected);
    }
    i_vec.sort();
    i_vec.reverse();
    let answer2_i = i_vec[0] * i_vec[1];
    // println!("{:?}",i_vec);

    let mut answer2 = answer2_i.to_string();
    return answer2;
}
