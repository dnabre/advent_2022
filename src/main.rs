#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

use std::collections::{HashMap, VecDeque};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::{cmp, fmt};
use std::fs;
use std::time::{Instant, Duration};

use enum_display_derive::Display as Derived_Display;

/*
    Advent of Code 2022: Day 23
        part1 answer:
        part2 answer:


 */


const TEST_ANSWER: (i64, i64) = (110, 93);
const INPUT_ANSWER: (i64, i64) = (843, 27625);

const PART1_TEST_FILENAME: &str = "data/day23/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day23/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day23/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day23/part2_input.txt";

const TEST: bool = true;


fn main() {
    print!("Advent of Code 2022, Day ");
    println!("23");                           // insert Day


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

    let start2 = Instant::now();
    let answer2 = part2();
    let duration2 = start2.elapsed();

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
const D_LINE_ENDING: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const D_LINE_ENDING: &'static str = "\n\n";

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    row: i32,
    col: i32,
}

impl Coord {
    fn add(&self, dir: Direction) -> Coord {
        match dir {
            Direction::North => { self.plus(-1, 0) }
            Direction::NorthEast => { self.plus(-1, 1) }
            Direction::East => { self.plus(0, 1) }
            Direction::SouthEast => { self.plus(1, 1) }
            Direction::South => { self.plus(1, 0) }
            Direction::SouthWest => { self.plus(1, -1) }
            Direction::West => { self.plus(0, -1) }
            Direction::NorthWest => { self.plus(-1, -1) }
        }
    }
    fn plus(&self, row: i32, col: i32) -> Coord {
        return Coord { row: self.row + row, col: self.col + col };
    }
}

impl Coord {}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.col, self.row)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Elf {
    loc: Coord,
    proposed: Option<Coord>,
}

impl Elf {
    fn new(start: Coord) -> Self {
        Self { loc: start, proposed: None }
    }
    fn look(&self, dir: Direction) -> [Coord; 3] {
        match dir {
            Direction::North => {
                [self.loc.plus(-1, 0),
                    self.loc.plus(-1, 1),
                    self.loc.plus(-1, -1)]
            }
            Direction::NorthEast => { panic!("got {dir}, not cardinal direction") }
            Direction::East => {
                [self.loc.plus(0, 1),
                    self.loc.plus(-1, 1),
                    self.loc.plus(1, 1)]
            }
            Direction::SouthEast => { panic!("got {dir}, not cardinal direction") }
            Direction::South => {
                [self.loc.plus(1, 0),
                    self.loc.plus(1, 1),
                    self.loc.plus(1, -1)]
            }
            Direction::SouthWest => { panic!("got {dir}, not cardinal direction") }
            Direction::West => {
                [self.loc.plus(0, -1),
                    self.loc.plus(-1, 1),
                    self.loc.plus(1, -1)]
            }
            Direction::NorthWest => { panic!("got {dir}, not cardinal direction") }
        }
    }
    fn get_neighbors(&self) -> [Coord; 8] {
        let result = [
            self.loc.add(Direction::North),
            self.loc.add(Direction::NorthEast),
            self.loc.add(Direction::East),
            self.loc.add(Direction::SouthEast),
            self.loc.add(Direction::South),
            self.loc.add(Direction::SouthWest),
            self.loc.add(Direction::West),
            self.loc.add(Direction::NorthWest),
        ];
        return result;
    }
}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "elf @{} proposed: {:?}", self.loc, self.proposed)
    }
}


#[derive(Hash, PartialEq, Eq, Derived_Display, Debug, Copy, Clone)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn print_set<T: Display>(set: HashSet<T>) {
    let mut c = 0;
    let l = set.len();

    let v: Vec<T> = set.into_iter().collect();

    let Some((last, elements)) = v.split_last()
        else {
            panic!("split_last on vector of hashset wonky");
        };
    print!("[");
    for elem in elements {
        print!("{}, ", elem);
    }
    println!("{}]", last);
}


fn rotate_elf_rules(rules: &mut VecDeque<Direction>) {
    let top = rules.pop_front().unwrap();
    rules.push_back(top);
}

fn gen_init_rules() -> VecDeque<Direction> {
    let mut elf_rules: VecDeque<Direction> = VecDeque::with_capacity(4);
    elf_rules.push_back(Direction::North);
    elf_rules.push_back(Direction::South);
    elf_rules.push_back(Direction::West);
    elf_rules.push_back(Direction::East);
    return elf_rules;
}


fn part1() -> String {
    let p1_file = match TEST {
        true => PART2_TEST_FILENAME,
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
    let split_lines: Vec<&str> = data1_ss.split(LINE_ENDING).collect();

    let mut c_col;
    let mut c_row;

    let mut elf_rules: VecDeque<Direction> = gen_init_rules();

    let mut elf_locations: HashSet<Coord> = HashSet::new();

    let mut elves: Vec<Elf> = Vec::new();
    c_row = 1;
    for l in split_lines {
        c_col = 1;
        for c in l.chars() {
            if c == '#' {
                let c = Coord { row: c_row, col: c_col };
                let mut e = Elf::new(c);
                elves.push(e);
                elf_locations.insert(c);
                //println!("new-> {}", &e);
            }
            c_col += 1;
        }
        c_row += 1;
    }
    let mut max_row = elf_locations.iter().map(|e| e.row).max().unwrap();
    let mut max_col = elf_locations.iter().map(|e| e.col).max().unwrap();
    let mut min_row = elf_locations.iter().map(|e| e.row).min().unwrap();
    let mut min_col = elf_locations.iter().map(|e| e.col).min().unwrap();
    //  (min_row,min_col) = (-2,-2); (max_row,max_col) = (8,8);
    for r in min_row..=max_row {
        for c in min_col..=max_col {
            if elf_locations.contains(&Coord { row: r, col: c }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    // First Half
    let mut target_loc: HashSet<Coord> = HashSet::with_capacity(elves.len());
    let mut target_count: HashMap<Coord, u32> = HashMap::new();
    for r in 1..10 {
        for e_i in 0..elves.len() {
            let neighbors: [Coord; 8] = elves[e_i].get_neighbors();
            let mut num_neigh = 0;
            for c in neighbors {
                if elf_locations.contains(&c) {
                    num_neigh += 1;
                }
            }
            print!("elf@{} ", elves[e_i].loc);
            if num_neigh == 0 {
                elves[e_i].proposed = None;
            } else {
                let mut go: Option<Direction> = None;
                for d in &elf_rules {
                    let dir_coords = elves[e_i].look(*d);
                    println!(" looks {} at {:?} ", d, dir_coords);
                    if !elf_locations.contains(&dir_coords[0]) ||
                        !elf_locations.contains(&dir_coords[1]) ||
                        !elf_locations.contains(&dir_coords[2]) {
                        go = Some(*d);
                        break;
                    }
                }
                match go {
                    None => {
                        elves[e_i].proposed = None;
                        println!("elf @ {}, wants to move None  proposed: None", elves[e_i].loc);
                    }
                    Some(d) => {
                        elves[e_i].proposed = Some(elves[e_i].loc.add(d));

                        println!("elf @ {}, wants to move {d}  proposed: {}", elves[e_i].loc, elves[e_i].proposed.unwrap());
                    }
                }
            }
            if let Some(d) = elves[e_i].proposed {
                if target_count.contains_key(&d) {
                    let mut c = *target_count.get(&d).unwrap();
                    c += 1;
                    target_count.insert(d, c);
                } else {
                    target_count.insert(d, 1);
                }
            }
        }
        // Second Half
        elf_locations = HashSet::new();
        for e_i in 0..elves.len() {
            print!("elf @ {}, proposed: ", elves[e_i].loc);
            match elves[e_i].proposed {
                None => {
                    print!(" None ");
                }
                Some(p) => {
                    print!(" {} ", p);
                    let t = target_count.get(&p);
                    match t {
                        None => { elves[e_i].loc = p; }
                        Some(1) => { elves[e_i].loc = p; }
                        Some(x) => {
                            println!("elf@{} can't moved to proposed {} because {} want to", elves[e_i].loc, elves[e_i].proposed.unwrap(), x);
                        }
                    }
                }
            }
            println!();
            elf_locations.insert(elves[e_i].loc);
            elves[e_i].proposed = None;
        }

        max_row = elf_locations.iter().map(|e| e.row).max().unwrap();
        max_col = elf_locations.iter().map(|e| e.col).max().unwrap();
        min_row = elf_locations.iter().map(|e| e.row).min().unwrap();
        min_col = elf_locations.iter().map(|e| e.col).min().unwrap();

        //   println!("({min_row}, {min_col}) -> ({max_row},{max_col})");

        //   (min_row,min_col) = (-2,-2); (max_row,max_col) = (8,8);
        println!("End of Round {r}");
        for r in min_row..=max_row {
            for c in min_col..=max_col {
                if elf_locations.contains(&Coord { row: r, col: c }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        rotate_elf_rules(&mut elf_rules);
    }

    println!("\n\n\n done");
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
    let mut lines: Vec<&str> = data2_s.trim().split("\n").collect();
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
