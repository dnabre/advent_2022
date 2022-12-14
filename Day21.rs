use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::fs;
use std::time::Instant;

use parse_display::FromStr;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

/*
    Advent of Code 2022: Day 21
        part1 answer: 331120084396440
        part2 answer: 3378273370680

 */


const TEST_ANSWER: (i64, i64) = (152, 301);
const INPUT_ANSWER: (i64, i64) = (331120084396440, 3378273370680);


const PART1_TEST_FILENAME: &str = "data/day21/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day21/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day21/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day21/part2_input.txt";


const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("21");                           // insert Day


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
    if TEST {
        assert_eq!(answer2, TEST_ANSWER.1.to_string());
    } else {
        assert_eq!(answer2, INPUT_ANSWER.1.to_string());
    }


    println!("----------\ndone");
}

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";


const PART2_MNAME: &str = "humn";

#[derive(FromStr, PartialEq, Hash, Debug, Copy, Clone)]
enum Op {
    #[display("+")]
    Plus,
    #[display("-")]
    Sub,
    #[display("/")]
    Div,
    #[display("*")]
    Mul,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Op::Plus => { "+" }
            Op::Sub => { "-" }
            Op::Div => { "/" }
            Op::Mul => { "*" }
        })
    }
}

#[derive(FromStr, Hash, PartialEq, Debug, Clone)]
enum MonkeyOp {
    #[display("{0}")]
    Number(i64),
    #[display("{left} {op} {right}")]
    Eq { left: String, op: Op, right: String },
}

impl fmt::Display for MonkeyOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MonkeyOp::Number(x) => {
                write!(f, "{:>5}", x)
            }
            MonkeyOp::Eq { left, op, right } => {
                write!(f, "{left} {op} {right}")
            }
        }
    }
}


#[derive(Hash, PartialEq, Debug, Clone)]
enum MonkeyOpD {
    Number(Decimal),
    Eq { left: String, op: Op, right: String },
}


fn do_monkey_op(left_v: i64, right_v: i64, op: Op) -> i64 {
    match op {
        Op::Plus => { left_v + right_v }
        Op::Sub => { left_v - right_v }
        Op::Div => { left_v / right_v }
        Op::Mul => { left_v * right_v }
    }
}

fn do_monkey_op_d(left_v: Decimal, right_v: Decimal, op: Op) -> Decimal {
    match op {
        Op::Plus => { left_v + right_v }
        Op::Sub => { left_v - right_v }
        Op::Div => { left_v / right_v }
        Op::Mul => { left_v * right_v }
    }
}


fn monkey_op_d(p0: MonkeyOp) -> MonkeyOpD {
    match p0 {
        MonkeyOp::Number(x) => {
            MonkeyOpD::Number(Decimal::from_i64(x).unwrap())
        }
        MonkeyOp::Eq { left, op, right } => {
            let l = left;
            let r = right;
            let o = op;
            MonkeyOpD::Eq {
                left: l,
                op: o,
                right: r,
            }
        }
    }
}


fn parse_monkeys(lines: &mut Vec<&str>) -> HashMap<String, MonkeyOp> {
    let mut name_to_op: HashMap<String, MonkeyOp> = HashMap::new();
    let mut v_namelist: Vec<String> = Vec::new();


    for ln in lines {
        let (l, r) = ln.split_once(":").unwrap();

        let m_name: String = String::from(l);
        if m_name.ne("root") {
            v_namelist.push(m_name.clone());
        }
        let m_op = r.trim().parse::<MonkeyOp>().unwrap();
        name_to_op.insert(m_name, m_op);
    }
    name_to_op
}

fn parse_monkeys_d(lines: &mut Vec<&str>) -> HashMap<String, MonkeyOpD> {
    let mut name_to_op: HashMap<String, MonkeyOpD> = HashMap::new();
    let mut v_namelist: Vec<String> = Vec::new();


    for ln in lines {
        let (l, r) = ln.split_once(":").unwrap();

        let m_name: String = String::from(l);
        if m_name.ne("root") {
            v_namelist.push(m_name.clone());
        }
        let m_op = r.trim().parse::<MonkeyOp>().unwrap();

        let m_op_d = monkey_op_d(m_op);

        name_to_op.insert(m_name, m_op_d);
    }
    name_to_op
}


fn solve_map_for<'a>(solve_for: &'a str, name_to_op: &'a HashMap<String, MonkeyOpD>, value_hash: &'a mut HashMap<&'a str, Decimal>) -> Decimal {
    let mut todo = Vec::new();
    todo.push(solve_for);
    while !todo.is_empty() {
        let current = todo.pop().unwrap();
        if value_hash.contains_key(current) {
            continue;
        }
        let monkey_op = name_to_op.get(current).unwrap();
        match monkey_op {
            MonkeyOpD::Number(n) => {
                value_hash.insert(current, (*n).into());
                continue;
            }
            MonkeyOpD::Eq { left, op, right } => {
                if value_hash.contains_key(left.as_str()) && value_hash.contains_key(right.as_str()) {
                    let (left_v, right_v) = (value_hash.get(left.as_str()).unwrap(),
                                             value_hash.get(right.as_str()).unwrap());
                    let result = do_monkey_op_d(*left_v, *right_v, *op);
                    value_hash.insert(current, result);
                } else {
                    // we're missing a value, so retry current op
                    todo.push(current);
                    if !value_hash.contains_key(left.as_str()) {
                        // we need left
                        todo.push(left);
                    }
                    if !value_hash.contains_key(right.as_str()) {
                        todo.push(right);
                    }
                }
            }
        }
    }

    let solution = *value_hash.get(solve_for).unwrap();

    return solution;
}

fn solve_map_for0<'a>(solve_for: &'a str, name_to_op: &'a HashMap<String, MonkeyOp>, value_hash: &'a mut HashMap<&'a str, i64>) -> i64 {
    let mut todo = Vec::new();
    todo.push(solve_for);
    while !todo.is_empty() {
        let current = todo.pop().unwrap();
        if value_hash.contains_key(current) {
            continue;
        }
        let monkey_op = name_to_op.get(current).unwrap();
        match monkey_op {
            MonkeyOp::Number(n) => {
                value_hash.insert(current, *n);
                continue;
            }
            MonkeyOp::Eq { left, op, right } => {
                if value_hash.contains_key(left.as_str()) && value_hash.contains_key(right.as_str()) {
                    let (left_v, right_v) = (value_hash.get(left.as_str()).unwrap(),
                                             value_hash.get(right.as_str()).unwrap());
                    let result = do_monkey_op(*left_v, *right_v, *op);
                    value_hash.insert(current, result);
                } else {
                    // we're missing a value, so retry current op
                    todo.push(current);
                    if !value_hash.contains_key(left.as_str()) {
                        // we need left
                        todo.push(left);
                    }
                    if !value_hash.contains_key(right.as_str()) {
                        todo.push(right);
                    }
                }
            }
        }
    }

    let solution = *value_hash.get(solve_for).unwrap();

    return solution;
}

fn test_for_depends<'a>(root: &'a str, goal: &'a str, op_map: &'a HashMap<String, MonkeyOpD>) -> bool {
    let mut stack: Vec<&str> = Vec::new();

    if root.ne(goal) {
        stack.push(root);
    }
    'todo: while !stack.is_empty() {
        let current = stack.pop().unwrap();
        let t_op = op_map.get(current).unwrap();
        match t_op {
            MonkeyOpD::Number(_) => {
                continue;
            }
            MonkeyOpD::Eq { left,right, ..} => {
                if goal.eq(left) {
                    stack.push(current);
                    break 'todo;
                }
                if goal.eq(right) {
                    stack.push(current);
                    break 'todo;
                }
                stack.push(left);
                stack.push(right);
            }
        }
    }
    return stack.len() != 0;
}


fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };
    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let mut lines: Vec<&str> = data1_s.trim().split(LINE_ENDING).collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }


    let name_to_op = parse_monkeys(&mut lines);
    let mut value_hash: HashMap<&str, i64> = HashMap::new();
    let final_value = solve_map_for0("root", &name_to_op, &mut value_hash);

    let answer1 = final_value.to_string();
    return answer1;
}


fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));
    let mut lines: Vec<&str> = data2_s.trim().split(LINE_ENDING).collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }

    let name_to_op: HashMap<String, MonkeyOpD> = parse_monkeys_d(&mut lines);


    let nothing = String::from("nothing");

    let root_op: &MonkeyOpD = name_to_op.get("root").unwrap();


    let (root_left, root_right) = match root_op {
        MonkeyOpD::Number(_) => { (&nothing, &nothing) }
        MonkeyOpD::Eq { left,  right, .. } => { (left, right) }
    };


    let target;
    let solve_test;
    let l_deps = test_for_depends(root_left, PART2_MNAME, &name_to_op);
    let r_deps = test_for_depends(root_right, PART2_MNAME, &name_to_op);
    let mut value_map: HashMap<&str, Decimal> = HashMap::new();
    if l_deps {
        target = solve_map_for(root_right, &name_to_op, &mut value_map);
        solve_test = root_left;
    } else if r_deps {
        target = solve_map_for(root_left, &name_to_op, &mut value_map);
        solve_test = root_right;
    } else {
        panic!("neither branch depends on {}", PART2_MNAME);
    }


    let left = target;
    let mut inital: HashMap<&str, Decimal> = HashMap::new();
    let mut right = solve_map_for(solve_test, &name_to_op, &mut inital);
    let base: i64 = 2;

    let (mut min, mut max): (Decimal, Decimal) = (Decimal::from_i64(-1 * base.pow(44)).unwrap(), Decimal::from_i64(base.pow(44)).unwrap());
    let mut answer = dec!(0);


    let test1 = dec!(1);
    let mut new_value_map: HashMap<&str, Decimal> = HashMap::new();
    new_value_map.insert(PART2_MNAME, test1);
    let r1 = solve_map_for(solve_test, &name_to_op, &mut new_value_map);

    let test2 = dec!(100);
    let mut new_value_map: HashMap<&str, Decimal> = HashMap::new();
    new_value_map.insert(PART2_MNAME, test2);
    let r2 = solve_map_for(solve_test, &name_to_op, &mut new_value_map);

    let positive = r1 < r2;


    while left != right {
        let d_two = Decimal::from_i64(2).unwrap();
        if right > target {
            if positive {
                max = min + (max - min) / d_two;
            } else {
                min = min + (max - min) / d_two;
            }
        } else {
            if positive {
                min = min + (max - min) / d_two;
            } else {
                max = min + (max - min) / d_two;
            }
        }

        let test = min + (max - min) / d_two;

        let mut new_value_map: HashMap<&str, Decimal> = HashMap::new();
        new_value_map.insert(PART2_MNAME, test);
        let result = solve_map_for(solve_test, &name_to_op, &mut new_value_map);

        answer = test;
        right = result;
    }



    let answer2 = answer.to_string();
    return answer2;
}
