


#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;


use std::time::Instant;

use parse_display::FromStr;



/*
    Advent of Code 2022: Day 07
        part1 answer:
        part2 answer:

 */


const TEST_ANSWER: (u32, u32) = (13, 36);
const INPUT_ANSWER: (u32, u32) = (5907, 0);



const PART1_TEST_FILENAME: &str = "data/day09/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day09/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day09/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day09/part2_input.txt";

const TEST: bool = true;


#[derive(FromStr, PartialEq, Debug)]
enum Step {
    #[display("L {0}")]
    Left(i32),
    #[display("R {0}")]
    Right(i32),
    #[display("U {0}")]
    Up(i32),
    #[display("D {0}")]
    Down(i32)

}
impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Step::Left(x) => write!(f, "Left {x}"),
            Step::Right(x) => write!(f, "Right {x}"),
            Step::Up(x) => write!(f, "Up {x}"),
            Step::Down(x)=> write!(f, "Down {x}"),
        }
    }
}

type Position = (i32,i32);


struct Rope {
    head: Position,
    tail: Position,
    knots: Vec<Position>
}
impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "head: <{},{}>, tail: <{},{}> # knots: {}",
               self.head.0,self.head.1, self.tail.0, self.tail.1, self.knots.len()+2)
    }
}

impl Rope {
    fn new(number_of_knots: usize) -> Self {
        Self {
            head: (0,0),
            tail: (0,0),
            knots: vec![(0,0); number_of_knots]
        }
    }

    fn left(&mut self) {
        self.move_by(-1,0);
    }
    fn right(&mut self) {
        self.move_by(1,0);
    }
    fn up(&mut self) {
        self.move_by(0,1);
    }
    fn down(&mut self) {
        self.move_by(0,-1);
    }
    fn move_by(&mut self, x:i32, y:i32) {
        self.head = (self.head.0 + x, self.head.1 + y);

        let mut lead_position = &self.head;

        for knot in self.knots.iter_mut() {
            let new_knot_position = Rope::new_tail_position(knot, lead_position);

            knot.0 = new_knot_position.0;
            knot.1 = new_knot_position.1;
            lead_position = knot;
        }

        self.tail = Rope::new_tail_position(&self.tail, lead_position);

    }
    fn new_tail_position(knot: &Position, lead_position: &Position) -> Position {
        let (cur_lead_x, cur_lead_y) = lead_position.clone();

        match knot.clone() {
            (x, y) if y == cur_lead_y => {
                if (cur_lead_x - x).abs() > 1 {
                    let step = if cur_lead_x > x { 1 } else { -1 };
                    (x + step, y)
                } else {
                    (x, y)
                }
            }
            (x, y) if x == cur_lead_x => {
                if (cur_lead_y - y).abs() > 1 {
                    let step = if cur_lead_y > y { 1 } else { -1 };
                    (x, y + step)
                } else {
                    (x, y)
                }
            }
            (x, y) if (x - cur_lead_x).abs() == 1 && (y - cur_lead_y).abs() == 1 => (x, y),
            (x, y) => {
                let step_x = if cur_lead_x > x { 1 } else { -1 };
                let step_y = if cur_lead_y > y { 1 } else { -1 };
                (x + step_x, y + step_y)
            }
        }
    }
}







fn main() {
    print!("Advent of Code 2022, Day ");
    println!("09");


    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();

    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);

    // if TEST {
    //     assert_eq!(answer1, TEST_ANSWER.0.to_string());
    // } else {
    //     assert_eq!(answer1, INPUT_ANSWER.0.to_string());
    // }

    let start2 = Instant::now();
    let answer2 = part2();
    let duration2 = start2.elapsed();
    println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);
    // if TEST {
    //     assert_eq!(answer2, TEST_ANSWER.1.to_string());
    // } else {
    //     assert_eq!(answer2, INPUT_ANSWER.1.to_string());
    // }


    println!("----------\ndone");
}



fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };
    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let mut lines: Vec<&str> = data1_s.trim().split("\n").collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
    }

    let mut rope = Rope::new(0);
    let mut visited:HashSet<Position> = HashSet::new();
    println!("\t {rope}");
    for i in 0..lines.len() {
        lines[i] = lines[i].trim();
        // println!("input: {}", lines[i]);
        let s:Step = lines[i].parse().unwrap();
        // println!("step: {s}");
        match s {
            Step::Left(x) => {
                for i in 0..x {
                    rope.left();
                    visited.insert(rope.tail);
                    // println!("\t head: {:?} tail: {:?}", rope.head, rope.tail);
                }
            }
            Step::Right(x) => {
                for i in 0..x {
                    rope.right();
                    visited.insert(rope.tail);
                    // println!("\t head: {:?} tail: {:?}", rope.head, rope.tail);
                }
            }
            Step::Up(x) => {
                for i in 0..x {
                    rope.up();
                    visited.insert(rope.tail);
                    // println!("\t head: {:?} tail: {:?}", rope.head, rope.tail);
                }
            }
            Step::Down(x) => {
                for i in 0..x {
                    rope.down();
                    visited.insert(rope.tail);
                    // println!("\t head: {:?} tail: {:?}", rope.head, rope.tail);
                }
            }
        }
        // println!("\texecuted step:");
        // println!("\t\t head: {:?} tail: {:?}", rope.head, rope.tail);


    }
    let num_visited_spots = visited.len();

    // println!("{:?}", visited);


    let mut answer1 = num_visited_spots.to_string();
    return answer1;
}


fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));

    let mut lines: Vec<&str> = data2_s.trim().split("\n").collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
    }

    for i in 0..lines.len() {
        lines[i] = lines[i].trim();
    }
    let mut rope = Rope::new(8);
    let mut visited:HashSet<Position> = HashSet::new();
    println!("\t {rope}");
    for i in 0..lines.len() {
        lines[i] = lines[i].trim();

        let s:Step = lines[i].parse().unwrap();
        let repeats = match s {
            Step::Left(x) => {x}
            Step::Right(x) => {x}
            Step::Up(x) => {x}
            Step::Down(x) => {x}
        };

        println!("repeats: {}", repeats);
        match s {
            Step::Left(x) => {
                for i in 0..x {
                    rope.left();
                    visited.insert(rope.tail);
                    // println!("\t head: {:?} tail: {:?}", rope.head, rope.tail);
                }
            }
            Step::Right(x) => {
                for i in 0..x {
                    rope.right();
                    visited.insert(rope.tail);
                    // println!("\t head: {:?} tail: {:?}", rope.head, rope.tail);
                }
            }
            Step::Up(x) => {
                for i in 0..x {
                    rope.up();
                    visited.insert(rope.tail);
                    // println!("\t head: {:?} tail: {:?}", rope.head, rope.tail);
                }
            }
            Step::Down(x) => {
                for i in 0..x {
                    rope.down();
                    visited.insert(rope.tail);
                    // println!("\t head: {:?} tail: {:?}", rope.head, rope.tail);
                }
            }
        }
        // println!("\texecuted step:");
        // println!("\t\t head: {:?} tail: {:?}", rope.head, rope.tail);


    }
    let num_visited_spots = visited.len();

    // println!("{:?}", visited);



    let mut answer2 = num_visited_spots.to_string();
    return answer2;
}
