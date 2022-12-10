


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
        part1 answer: 5907
        part2 answer: 2303

 */


const TEST_ANSWER: (u32, u32) = (13, 36);
const INPUT_ANSWER: (u32, u32) = (5907,2303);



const PART1_TEST_FILENAME: &str = "data/day09/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day09/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day09/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day09/part2_input.txt";

const TEST: bool = false;
fn main() {
    print!("Advent of Code 2022, Day ");
    println!("09");


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

        let mut head_knot = &self.head;

        for knot in self.knots.iter_mut() {
            let new_knot_position = Rope::new_tail_position(knot, head_knot);

            knot.0 = new_knot_position.0;
            knot.1 = new_knot_position.1;
            head_knot = knot;
        }

        self.tail = Rope::new_tail_position(&self.tail, head_knot);

    }
    fn new_tail_position(knot: &Position, head_knot: &Position) -> Position {
        let (head_knot_x, head_knot_y) = head_knot.clone();
        let (x,y):(i32,i32) = (knot.0, knot.1);
        if(head_knot_y == y) {
            let distance = (head_knot_x - x).abs();
            if distance > 1 {
                if head_knot_x > x {
                    return (x + 1, y)
                } else {
                    return (x - 1, y)
                }
            } else {
                return (x, y);
            }
        } else if(head_knot_x == x) {
            let distance = (head_knot_y - y).abs();
            if distance > 1 {
                if head_knot_y > y {
                 return (x, y+1);
                } else {
                    return    (x,y-1);
                }
            } else {
                return (x,y);
            }
        } else if ((head_knot_x -x).abs() ==1) && ((head_knot_y -y).abs() ==1) {
            return   (x,y)
        } else {
            let delta_x = if head_knot_x > x { 1 } else { -1 };
            let delta_y = if head_knot_y > y {1} else {-1};
            return   (x + delta_x, y + delta_y)
        }
    }
}









fn run_steps(mut rope: Rope, steps: &mut Vec<Step>) -> HashSet<Position> {

    let mut visited:HashSet<Position> = HashSet::new();
    for s in steps
    {
        let repeats = match s {
            Step::Left(x) => { x }
            Step::Right(x) => { x }
            Step::Up(x) => { x }
            Step::Down(x) => { x }
        };


        for _ in 0..*repeats {
            match s {
                Step::Left(_) => { rope.left() }
                Step::Right(_) => { rope.right() }
                Step::Up(_) => { rope.up() }
                Step::Down(_) => { rope.down() }
            }
            visited.insert(rope.tail);
        }
    }
    return visited;
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
    for i in 0..lines.len() {
        lines[i] = lines[i].trim();
    }
    let mut rope = Rope::new(0);

    println!("\t {rope}");
    let mut steps:Vec<Step> = Vec::new();

    for i in 0..lines.len() {
        lines[i] = lines[i].trim();
        let s:Step = lines[i].parse().unwrap();
        steps.push(s);
    }

    let visited  = run_steps(rope, &mut steps);

    let mut answer1 = visited.len().to_string();
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

    println!("\t {rope}");
    let mut steps:Vec<Step> = Vec::new();

    for i in 0..lines.len() {
        lines[i] = lines[i].trim();
        let s:Step = lines[i].parse().unwrap();
        steps.push(s);
    }

    let visited  = run_steps(rope, &mut steps);



    let mut answer2 = visited.len().to_string();
    return answer2;
}
