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
        part1 answer:
        part2 answer:

 */


const TEST_ANSWER: (i32, i32) = (0, 0);
const INPUT_ANSWER: (i32, i32) = (0, 0);


const PART1_TEST_FILENAME: &str = "data/day11/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day11/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day11/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day11/part2_input.txt";

const TEST: bool = true;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("11");


    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();

    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);

    // if TEST {
    //     assert_eq!(answer1, TEST_ANSWER.0.to_string());
    // } else {
    //     assert_eq!(answer1, INPUT_ANSWER.0.to_string());
    // }
    //
    // let start2 = Instant::now();
    // let answer2 = part2();
    // let duration2 = start2.elapsed();
    //
    // println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);
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


/*
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
 */

#[derive(FromStr, PartialEq, Debug, Clone)]
#[display(
"Monkey {monkey_id}:|Test: divisible by {test_divider}|If true: throw to monkey {true_monkey}|If false: throw to monkey {false_monkey}")]
#[from_str(default)]
struct Monkey {
    monkey_id: usize,
    items: VecDeque<i32>,
    oper_type: Operation,
    oper_amount: i32,
    test_divider: i32,
    true_monkey: usize,
    false_monkey: usize,
    items_inspected: i32,
}

impl Monkey {
    fn inspect(&mut self) -> (Vec<i32>, Vec<i32>) {
        let mut items_for_true_monkey: Vec<i32> = Vec::new();
        let mut items_for_false_monkey: Vec<i32> = Vec::new();

        let init_worry: i32 = match self.items.pop_front() {
            None => {
                panic!("attempted to inspect on monkey without items: {self}");
            }
            Some(x) => { x }
        };
        println!(" Monkey inspects an item with worry level of {init_worry}.");


        let mut new_worry = match self.oper_type {
            Operation::Mult => { init_worry * self.oper_amount }
            Operation::Add => { init_worry + self.oper_amount }
            Operation::Square => { init_worry * init_worry }
            Operation::None => {
                panic!("monkey shouldn't have Operation::None, {self}");
            }
        };
        println!("    Worry level is {} to {}.",
                 match self.oper_type {
                     Operation::Mult => { format!("multiplied by {}", self.oper_amount) }
                     Operation::Add => { format!("increased by {}", self.oper_amount) }
                     Operation::Square => { format!("multiplied by itself") }
                     Operation::None => { panic!("shouldn't have Operation::None, {self}") }
                 }, new_worry);

        new_worry = new_worry / 3;
        println!("    Monkey gets bored with item. Worry level is divided by 3 to {new_worry}.");
        if new_worry % self.test_divider == 0 {
            println!("    Current worry level is not divisible by {}.", self.test_divider);
            println!("Item with worry level {new_worry} is thrown to monkey {}.",
                     self.true_monkey);
//            monkey[self.true_monkey].items.push_back(new_worry);
        } else {
            println!("    Current worry level is divisible by {}.", self.test_divider);
        }
        return  (items_for_true_monkey, items_for_false_monkey);

    }
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


//"Monkey {monkey_id}:|Test: divisible by {test_divider}|If true: throw to monkey {true_monkey}|If false: throw to monkey {false_monkey}")]
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
    println!("number of monkeys: {}", monkey.len());
    for i in 0..monkey.len() {
        println!("monkey[{i:>3}]: {}", monkey[i]);
    }
    // Parsing done
    let number_of_monkeys = monkey.len();
    for mut current_monkey in &monkey {
        println!("Monkey {}:", current_monkey.monkey_id);
        let (true_id, false_id) = (current_monkey.true_monkey, current_monkey.false_monkey);
        let (true_list, false_list) =  current_monkey.inspect();
        for item in true_list {
            monkey[true_id].items.push_back(item);
        }
        for item in false_list{
            monkey[false_id].items.push_back(item);
        }
        break;
    }


    println!();
    println!();
    let mut answer1 = String::new();
    return answer1;
}


fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));

    let mut lines: Vec<&str> = data2_s.trim().split("\n").map(|t| t.trim_end()).collect();
    let l_num = lines.len();

    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
    }


    let mut answer2 = String::new();
    return answer2;
}
