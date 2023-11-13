#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]


use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs;
use std::time::Instant;

/*
    Advent of Code 2022: Day 24`
        part1 answer:
        part2 answer:


 */


const TEST_ANSWER: (i64, i64) = (20, 110);
const INPUT_ANSWER: (i64, i64) = (4195, 1069);

const PART1_TEST_FILENAME: &str = "data/day24/part1_test.txt";
const PART1B_TEST_FILENAME: &str = "data/day24/part1b_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day24/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day24/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day24/part2_input.txt";

const TEST: bool = true;


fn main() {
    print!("Advent of Code 2022, Day ");
    println!("24");                           // insert Day


    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();

    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);
    if TEST {
        if answer1 != TEST_ANSWER.0.to_string() {
            println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
                     answer1, TEST_ANSWER.0.to_string())
        }
    } else {
        if answer1 != INPUT_ANSWER.0.to_string() {
            println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
                     answer1, INPUT_ANSWER.0.to_string())
        }
    }

    // let start2 = Instant::now();
    // let answer2 = part2();
    // let duration2 = start2.elapsed();

    // println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);
    //
    //
    // if TEST {
    //     if answer2 != TEST_ANSWER.1.to_string() {
    //         println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
    //                  answer2,TEST_ANSWER.1.to_string() )
    //     }
    // } else {
    //     if answer2 != INPUT_ANSWER.1.to_string() {
    //         println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
    //                  answer2,TEST_ANSWER.1.to_string() )
    //     }
    // }


    println!("----------\ndone");
}


#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",
        match self {
            Direction::Up => {"^"}
            Direction::Down => {"v"}
            Direction::Left => {"<"}
            Direction::Right => {">"}
        })
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"@[row={}, col={}]", self.row, self.col )
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Blizz {
    row: usize,
    col: usize,
    dir: Direction
}

impl Blizz {
     fn pos_at_time(&self, t: i32, (max_row, max_col):(usize,usize) ) -> Coord {
         let mut r:i32 = -1;
         let mut c:i32 = -1;

         let (i_max_row, i_max_col) = (max_row as i32, max_col as i32);
         let (i_row, i_col) = (self.row as i32, self.col as i32);


         match self.dir{
             Direction::Up => {
                 c = i_col;
                 // messy to ignore modulo of negative
                 r =  (i_row - 1 +  (t * (i_max_row -2) -t))
                     % (i_max_row - 2) + 1 ;

             }
             Direction::Down => {
                 c = i_col;
                 r =  ((i_row - 1 + t) % (i_max_row - 2)) + 1 ;
             }
             Direction::Left => {
                 r = i_row;
                 // messy to ignore modulo of negative
                 c =   (i_col - 1 +  (t * (i_max_col -2) -t))
                     % (i_max_col - 2) + 1 ;


             }
             Direction::Right => {
                 r = i_row;
                 c= ((i_col - 1 + t) % (i_max_col - 2)) + 1 ;
             }
         }

        return Coord{row: r as usize, col: c as usize};
    }
}

impl Display for Blizz {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       write!(f, "@[row={}, col={}] dir={}", self.row, self.col, self.dir)
    }
}

impl Blizz {
    fn new(r:usize, c:usize, d:char) -> Blizz {
        let dir = match d
        {
            '^' => {Direction::Up}
            'v' => {Direction::Down}
            '<' => {Direction::Left}
            '>' => {Direction::Right}
            _ => { panic!("bad direction given in Blizz::new -> {d}")}
        };
        return Blizz{ row: r, col: c, dir, };
    }
}


fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };
    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let lines: Vec<&str> = data1_s.trim().split("\n").collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }
    let data1_ss = data1_s.trim();
    let split_lines: Vec<&str> = data1_ss.split(LINE_ENDING).collect();

    let num_rows = split_lines.len();
    let num_cols = split_lines[0].len();

    let dims = (num_rows, num_cols);
    println!("rows: {}, cols: {}", num_rows, num_cols);

    let start_coord = Coord{ row: 0, col: 1 };
    let end_coord = Coord{row: num_rows -1, col: num_cols-2};

    let dims = ( num_rows,  num_cols);

    println!("grid dimensions: {:?}", dims);
    let v1:Vec<Vec<char>> = split_lines.iter().map(|f| f.chars().collect()).collect();

    let mut wall_set:HashSet<Coord> = HashSet::new();
    let mut blizz_vec:Vec<Blizz> = Vec::new();
    let mut coord_to_blizz:HashMap<Coord, Blizz> = HashMap::new();

    for r in 0..num_rows {
        for c in 0..num_cols {
            match v1[r][c] {
                '#' => {wall_set.insert(Coord{row: r, col: c});}
                '.' => {/*ignore empty spots*/ }
                d_char => {
                    let b = Blizz::new(r,c,d_char);
                    coord_to_blizz.insert(Coord{row: r, col: c}, b);
                    blizz_vec.push(b);
                }
            }
        }
    }

    println!("found {} blizzards", blizz_vec.len());
    //
    // for b in blizz_vec.iter() {
    //     println!("start {}", b);
    //     for t in 0..10 {
    //         let c = b.pos_at_time(t,dims);
    //         println!("\t at time {t}, {c}");
    //     }
    //     println!();
    // }


    for t in 0..=5 {
        println!(" === time {t} ===");
        for r in 0..num_rows {
            for c in 0..num_cols {
                let mut empty = true;
                let co = Coord { row: r, col: c };
                if co == start_coord {
                    print!("B");
                    empty = false;
                    continue
                } else if co == end_coord {
                    print!("E");
                    empty = false;
                    continue
                } else if wall_set.contains(&co) {
                    print!("#");
                    empty = false;
                    continue
                }

                for b in &blizz_vec {
                    let t_coord = b.pos_at_time(t, dims);
                    if t_coord == co {
                        print!("{}", b.dir);
                        empty = false;
                        break;
                    }

                }
                if empty {
                    print!(".");
                }
            }
            println!();
        }
        println!("===================\n");
    }


            let answer1 = String::new();
    return answer1.to_string();
}


fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));
    let lines: Vec<&str> = data2_s.trim().split("\n").collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }
    let data2_ss = data2_s.trim();
    let split_lines: Vec<&str> = data2_ss.split(LINE_ENDING).collect();


    let answer2 = String::new();
    return answer2.to_string();
}
