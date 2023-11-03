use std::fmt;
use std::fmt::Formatter;
use std::fs;
use std::time::Instant;

use array2d::Array2D;

/*
    Advent of Code 2022: Day 14
        part1 answer:   843
        part2 answer:   27625
 */

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Block {
    Empty,
    Stone,
    Sand,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Block::Empty => { write!(f, ".") }
            Block::Sand => { write!(f, "o") }
            Block::Stone => { write! {f, "#"} }
        }
    }
}

impl Block {
    fn is_open(&self) -> bool {
        match self {
            Block::Empty => { true }
            Block::Stone => { false }
            Block::Sand => { false }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, )]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn one_step(&self, grid: &Array2D<Block>) -> Point {
        let mut look_block;

        // Check below
        look_block = grid_get_by_delta(grid, self, 0, 1);
        if look_block.is_open() {
            let new_pos = Point { x: self.x + 0, y: self.y + 1 };
            return new_pos;
        }

        //Check down and left
        look_block = grid_get_by_delta(grid, self, -1, 1);
        if look_block.is_open() {
            let new_pos = Point { x: self.x - 1, y: self.y + 1 };
            return new_pos;
        }

        //Check down and right
        look_block = grid_get_by_delta(grid, self, 1, 1);
        if look_block.is_open() {
            let new_pos = Point { x: self.x + 1, y: self.y + 1 };
            return new_pos;
        }
        return self.clone();
    }

    fn one_step2(&self, grid: &Array2D<Block>, void_level: usize) -> Point {
        let mut look_block;
        // Check below
        look_block = grid_get_by_delta2(grid, self, 0, 1, void_level);
        if look_block.is_open() {
            let new_pos = Point { x: self.x + 0, y: self.y + 1 };
            return new_pos;
        }
        //Check down and left
        look_block = grid_get_by_delta2(grid, self, -1, 1, void_level);
        if look_block.is_open() {
            let new_pos = Point { x: self.x - 1, y: self.y + 1 };
            return new_pos;
        }

        //Check down and right
        look_block = grid_get_by_delta2(grid, self, 1, 1, void_level);
        if look_block.is_open() {
            let new_pos = Point { x: self.x + 1, y: self.y + 1 };
            return new_pos;
        }
        return self.clone();
    }

    fn parse(s: &str) -> Self {
        let mut tokens = s.split(',');
        let (x, y) = (tokens.next().unwrap(), tokens.next().unwrap());
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }

    fn compare(&self, other: Point) -> PointCompare {
        return if other.x == self.x {
            if other.y == self.y {
                return PointCompare::Same;
            }
            PointCompare::Vert
        } else {
            PointCompare::Hort
        };
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PointCompare {
    Same,
    Hort,
    Vert,
}

struct PolyLine<'a>(&'a Vec<Point>);

impl fmt::Display for PolyLine<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for element in &self.0[0..self.0.len() - 1] {
            result.push_str(&*element.to_string());
            result.push_str(" -> ")
        }
        result.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "{}", result)
    }
}


const TEST_ANSWER: (i64, i64) = (24, 93);
const INPUT_ANSWER: (i64, i64) = (843, 27625);

const PART1_TEST_FILENAME: &str = "data/day14/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day14/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day14/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day14/part2_input.txt";

const TEST: bool = false;

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

    println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);

    if TEST {
        if answer2 != TEST_ANSWER.1.to_string() {
            println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
                     answer2, TEST_ANSWER.1.to_string())
        }
    } else {
        if answer2 != INPUT_ANSWER.1.to_string() {
            println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
                     answer2, TEST_ANSWER.1.to_string())
        }
    }
    println!("----------\n");
}

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

// sands starts at "500,0" or row 0 and col =500 const SAND_START_ROW: usize = 0;
const SAND_START_COL: usize = 500;
const SAND_START_ROW: usize = 0;

fn build_grid(poly_lines: Vec<Vec<Point>>, default_block_value: Block, rows: usize, cols: usize) -> Array2D<Block> {
    let mut grid: Array2D<Block> = Array2D::filled_with(default_block_value, rows, cols);
    for pl in poly_lines {
        for p_i in 1..pl.len() {
            let mut p_s = pl[p_i - 1];
            let mut p_e = pl[p_i];
            let c = p_e.compare(p_s);
            match c {
                PointCompare::Same => { panic!("p_s and p_e shouldn't be the same, p_s: {p_s} \t p_e: {p_e}"); }
                PointCompare::Hort => {
                    if p_e.x < p_s.x {
                        (p_s, p_e) = (p_e, p_s);
                    }
                }
                PointCompare::Vert => {
                    if p_e.y < p_s.y {
                        (p_s, p_e) = (p_e, p_s);
                    }
                }
            }
            let (mut x, mut y) = (p_s.x, p_s.y);
            while x != p_e.x {
                grid_set(&mut grid, x, y, Block::Stone);
                x += 1;
            }
            while y != p_e.y {
                grid_set(&mut grid, x, y, Block::Stone);
                y += 1;
            }
            grid_set(&mut grid, p_e.x, p_e.y, Block::Stone);
        }
    }
    return grid;
}

fn grid_get(grid: &Array2D<Block>, col: usize, row: usize) -> Block {
    let res = grid.get(row, col);
    match res {
        None => { panic!("unable to read grid at [{row}, {col}]"); }
        Some(b) => { return *b; }
    }
}
fn grid_get2(grid: &Array2D<Block>, col: usize, row: usize, void_level: usize) -> Block {
    if row >= void_level {
        return Block::Stone;
    }
    let res = grid.get(row, col);
    match res {
        None => { panic!("unable to read grid at [{row}, {col}]"); }
        Some(b) => { return *b; }
    }
}

fn grid_get_by_delta(grid: &Array2D<Block>, p: &Point, col_delta: i32, row_delta: i32) -> Block {
    let ip_x = p.x as i32;
    let ip_y = p.y as i32;
    let new_x = (ip_x + col_delta) as usize;
    let new_y = (ip_y + row_delta) as usize;
    return grid_get(grid, new_x, new_y);
}
fn grid_get_by_delta2(grid: &Array2D<Block>, p: &Point, col_delta: i32, row_delta: i32, void_level: usize) -> Block {
    let ip_x = p.x as i32;
    let ip_y = p.y as i32;
    let new_x = (ip_x + col_delta) as usize;
    let new_y = (ip_y + row_delta) as usize;
    return grid_get2(grid, new_x, new_y, void_level);
}

fn grid_set(grid: &mut Array2D<Block>, col: usize, row: usize, block: Block) {
    let res = grid.set(row, col, block);
    match res {
        Ok(_) => { return; }
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}

fn parse_polylines(split_lines: Vec<&str>) -> (Vec<Vec<Point>>, (usize, usize)) {
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    let mut poly_lines: Vec<Vec<Point>> = Vec::new();
    for l in split_lines {
        let poly_line: Vec<Point> = l.split(" -> ").map(Point::parse).collect();
        for p in &poly_line {
            if p.x > max_x {
                max_x = p.x;
            }
            if p.y > max_y {
                max_y = p.y;
            }
        }
        poly_lines.push(poly_line);
    }
    return (poly_lines, (max_x, max_y));
}

fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };
    let data1_s = fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
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
    let (poly_lines, (max_x, max_y)) = parse_polylines(split_lines);
    let void = max_y + 2;

    let mut grid = build_grid(poly_lines, Block::Empty, max_y + 3, max_x * 20);

    let mut sand_dropped = 0;
    let mut reach_void = false;

    while !reach_void {
        sand_dropped += 1;
        let initial_sand = Point { x: SAND_START_COL, y: SAND_START_ROW };
        let mut sand_current = initial_sand;
        let mut last_point = initial_sand;
        let mut next_point;

        loop {
            next_point = sand_current.one_step(&grid);
            if next_point.y >= void {
                reach_void = true;
                break;
            }
            if next_point == last_point {
                grid_set(&mut grid, next_point.x, next_point.y, Block::Sand);
                break;
            } else {
                sand_current = next_point;
                last_point = sand_current;
            }
        }
    }
    let answer1 = (sand_dropped - 1).to_string();
    return answer1.to_string();
}

fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s = fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));
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

    let (poly_lines, (max_x, max_y)) = parse_polylines(split_lines);
    let void_level = max_y + 2;
    let mut grid = build_grid(poly_lines, Block::Empty, max_y + 3, max_x * 20);

    let mut sand_dropped = 0;
    let mut reach_void = false;
    let mut sand_filled = false;

    while !reach_void && !sand_filled {
        sand_dropped += 1;
        let initial_sand = Point { x: SAND_START_COL, y: SAND_START_ROW };
        let mut sand_current = initial_sand;
        let mut last_point = initial_sand;
        let mut next_point;

        loop {
            next_point = sand_current.one_step2(&grid, void_level);
            if next_point.y >= void_level {
                reach_void = true;
                break;
            }
            if next_point == last_point {
                if next_point.y == 0 {
                    sand_filled = true;
                    break;
                } else {
                    grid_set(&mut grid, next_point.x, next_point.y, Block::Sand);
                }
                break;
            } else {
                sand_current = next_point;
                last_point = sand_current;
            }
        }
    }
    let answer2 = (sand_dropped).to_string();
    return answer2.to_string();
}