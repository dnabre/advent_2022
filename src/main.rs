#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::collections::{HashMap, VecDeque};
use std::collections::HashSet;
use std::{cmp, fmt};
use std::fmt::{Display, Formatter};
use std::fs;
use std::time::{Instant,Duration};
use array2d::{Array2D, Error};


/*
    Advent of Code 2022: Day 14
        part1 answer:
        part2 answer:


 */


#[derive(Debug, Copy, Clone,PartialEq,Eq, )]
enum Block {
    Empty,
    Stone,
    Sand,
    Void
}
impl fmt::Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Block::Empty => {write!(f,"_")}
            Block::Sand  => {write!(f,"*")}
            Block::Stone => {write!{f,"#"}}
            Block::Void => {write!(f,"x")}
        }
    }
}
#[derive(Debug, Copy, Clone,PartialEq,Eq,)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn parse(s: &str) -> Self {
        let mut tokens = s.split(',');
        let (x, y) = (tokens.next().unwrap(), tokens.next().unwrap());
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
    fn compare(&self, other:Point) -> PointCompare{
        if other.x == self.x {
            if other.y == self.y {
                return PointCompare::Same;
            }
            return PointCompare::Vert;
        } else {
            assert_eq!(other.y, self.y);
            return PointCompare::Hort;
        }
    }
}
#[derive(Debug)]
enum PointCompare {
    Same,
    Hort,
    Vert
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       write!(f,"{},{}", self.x, self.y)
    }
}

struct PolyLine<'a>(&'a Vec<Point>);

impl std::fmt::Display for PolyLine<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for element in &self.0[0..self.0.len() -1] {
            result.push_str(&*element.to_string());
            result.push_str(" -> ")
        }
        result.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f,"{}",result)
    }
}


const TEST_ANSWER: (i64, i64) = (64, 58);
const INPUT_ANSWER: (i64, i64) = (4282, 2452);

const PART1_TEST_FILENAME: &str = "data/day14/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day14/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day14/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day14/part2_input.txt";

const TEST: bool = true;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("14");                           // insert Day


    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();

    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);
    if TEST {
        if answer1 != TEST_ANSWER.0.to_string() {
            println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
                     answer1,TEST_ANSWER.0.to_string() )
        }
    } else {
        if answer1 != INPUT_ANSWER.0.to_string() {
            println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
                     answer1,TEST_ANSWER.0.to_string() )
        }
    }
    //
    // let start2 = Instant::now();
    // let answer2 = part2();
    // let duration2 = start2.elapsed();
    //
    // println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);
    //
    //
    // if TEST {
    //     if answer2 != TEST_ANSWER.0.to_string() {
    //         println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
    //                  answer2,TEST_ANSWER.1.to_string() )
    //     }
    // } else {
    //     if answer2 != INPUT_ANSWER.0.to_string() {
    //         println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
    //                  answer2,TEST_ANSWER.1.to_string() )
    //     }
    // }



    println!("----------\ndone");
}

#[cfg(windows)]
const D_LINE_ENDING: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const D_LINE_ENDING: &'static str = "\n\n";

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

const MAX_DIM:usize = 501;



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
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }
    let data1_ss = data1_s.trim();
    let split_lines:Vec<&str> = data1_ss.split(LINE_ENDING).collect();
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;


    let mut poly_lines:Vec<Vec<Point>> = Vec::new();
    for l in split_lines {
        let poly_line:Vec<Point> = l.split(" -> ").map(Point::parse).collect();
        for p in &poly_line {
            if p.x > max_x {
                max_x = p.x;
            }
            if p.y > max_y {
            max_y = p.y;
            }
        }
        println!("{}", PolyLine(&poly_line));
        poly_lines.push(poly_line);

    }

    println!("max x,y: ({max_x}, {max_y})");


    let mut grid = Array2D::filled_with(Block::Empty, 15,600);
    for pl in poly_lines {
        println!("begin poly_line: {:?}", pl);
        let  (mut current_x, mut current_y) = (pl[0].x, pl[0].y);
        for p_i in 1..pl.len() {
            let p_s = pl[p_i - 1];
            let p_e = pl[p_i];
            let c = p_e.compare(p_s);
            println!("doing {p_s} -> {p_e} for p_i={p_i} c={:?}",c);


            match c {
                PointCompare::Same => {
                    panic!("p_s and p_e shouldn't be the same, p_s: {p_s} \t p_e: {p_e}");
                }
                PointCompare::Hort => {
                    let x = p_e.x;
                    for y in p_s.y ..=p_e.y{
                        println!("({x},{y}) = {}  {p_s} -> {p_e}", Block::Stone);
                        let r = grid.set(y, x, Block::Stone);
                        check(r);
                    }
                }
                PointCompare::Vert => {
                    let x = p_e.x;
                    for y in p_s.y ..=p_e.y {

                        println!("({x},{y}) = {}  {p_s} -> {p_e}", Block::Stone);
                            let r = grid.set(y,x, Block::Stone);
                            check(r);
                    }
                }
            }
        }
        println!("end poly_line: {:?}", pl);
    }

    let mut count_stone =0;
    let mut count_sand =0;
    let mut count_empty =0;
    let mut count_void =0;

     for row_iter in grid.rows_iter() {
         for element in row_iter {
            match element {
                Block::Empty => {count_empty += 1;}
                Block::Stone => {count_stone += 1;}
                Block::Sand => {count_sand += 1;}
                Block::Void => {count_void +=1 ;}
            }

         }
     }

    println!( "count_empty = {count_empty}");
    println!( "count_stone = {count_stone}");
    println!( "count_sand  = {count_sand}");
    println!( "count_void  = {count_void}");




    print_grid(grid);





    let answer1 = String::new();
    return answer1.to_string();
}

fn check(res: Result<(), Error>) {
    match res {
        Ok(_) => {return;}
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}


fn print_grid(grid: Array2D<Block>) {
    // let (min_x, min_y) = (0, 0);
    // let (max_x, max_y) = ( grid.row_len() ,grid.column_len());

    // let (min_x, min_y) = (450, 0);
    // let (max_x, max_y) = ( grid.row_len() ,grid.column_len());
    //
    //
    //
    //
    // println!("from [{min_x},{min_y}] to [{max_x},{max_y}]");
    // for x in min_x..max_x {
    //     for y in min_y..max_y {
    //         let c = grid.get(y,x).unwrap();
    //         print!("{}", c);
    //
    //     }
    //     println!();
    // }


    for row_iter in grid.rows_iter() {
        for element in row_iter {
            print!("{}", element);
        }
        println!();
    }
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
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }
    let data2_ss = data2_s.trim();
    let split_lines:Vec<&str> = data2_ss.split(LINE_ENDING).collect();

    for l in split_lines {
        println!("{l}");
    }

    let answer2 = String::new();


    return answer2;
}
