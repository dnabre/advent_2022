#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]



use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::{fmt, fs};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::io::Write;
use std::path::Component::ParentDir;
use std::process::exit;
use std::time::Instant;
use itertools::Itertools;
use ndarray::{Array2, ArrayBase, Ix2, OwnedRepr};
use priority_queue::{DoublePriorityQueue, PriorityQueue};
use crate::Block::{Blizzard, MBlizz};

/*
    Advent of Code 2022: Day 16`
        part1 answer: 301
        part2 answer:

p1: 303 -- too high
p2: 860 -- too high
 */


const TEST_ANSWER: (i64, i64) = (1651, 54);
// 25.652ms
const INPUT_ANSWER: (i64, i64) = (2077, 859);


const PART1_TEST_FILENAME: &str = "data/day16/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day16/part1_input.txt";
const PART2_TEST_FILENAME: &str = "data/day16/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day16/part2_input.txt";




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
    println!("----------\ndone");
}












const MAX_GEN_SIZE:usize = 1000;



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
impl Direction {
    fn iterator() -> impl Iterator<Item = Direction> {
        [Direction::Up, Direction::Down, Direction::Right, Direction::Left].iter().copied()
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn get_neighbors(&self) -> [Option<Coord>; 5] {
        let mut result:[Option<Coord>;5] = Default::default();
        let mut i =0 ;
        for d in Direction::iterator() {
            result[i] = self.step(d);
            i += 1;
        }
        result[i] = Some(self.clone());
        return result;
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"@[row={}, col={}]", self.row, self.col )
    }
}

impl Coord {
    fn step(&self, d:Direction) -> Option<Coord> {
        let (r,c) = (self.row, self.col);
        match d {
            Direction::Up => {
                if r == 0 {
                    return None
                } else {
                    return Some(Coord { row: r - 1, col: c });
                }
            }
            Direction::Down => {
                return Some(Coord { row: r+1,col: c});
            }
            Direction::Left => {
                if c == 0 {
                    return None;
                } else {
                    return Some(Coord { row: r, col: c - 1 });
                }
            }
            Direction::Right => {
                return Some(Coord { row: r, col: c+1});
            }
        }
    }

}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    pos: Coord,
    t: usize
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "time {} {}", self.t, self.pos)
    }
}



#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Blizz {
    row: usize,
    col: usize,
    dir: Direction,
    i_max_row: i32,
    i_max_col: i32
}

impl Blizz {
     fn pos_at_time(&self, t: usize ) -> Coord {
         let mut r:i32 = -1;
         let mut c:i32 = -1;
            let i_t = t as i32;
         let (i_max_row, i_max_col) = (self.i_max_row, self.i_max_col);
         let (i_row, i_col) = (self.row as i32, self.col as i32);


         match self.dir{
             Direction::Up => {
                 c = i_col;
                 // messy to ignore modulo of negative
                 r =  (i_row - 1 +  (i_t * (i_max_row -2) -i_t))
                     % (i_max_row - 2) + 1 ;

             }
             Direction::Down => {
                 c = i_col;
                 r =  ((i_row - 1 + i_t) % (i_max_row - 2)) + 1 ;
             }
             Direction::Left => {
                 r = i_row;
                 // messy to ignore modulo of negative
                 c =   (i_col - 1 +  (i_t * (i_max_col -2) -i_t))
                     % (i_max_col - 2) + 1 ;


             }
             Direction::Right => {
                 r = i_row;
                 c= ((i_col - 1 + i_t) % (i_max_col - 2)) + 1 ;
             }
         }

        return Coord{row: r as usize, col: c as usize};
    }
    fn pos_at_time2(&self, t: usize ) -> Coord {
        let mut r:i32 = -1;
        let mut c:i32 = -1;
        let i_t = t as i32;
        let (i_max_row, i_max_col) = (self.i_max_row, self.i_max_col);
        let (i_row, i_col) = (self.row as i32, self.col as i32);


        match self.dir{
            Direction::Up => {
                c = i_col;
                // messy to ignore modulo of negative
                r =  (i_row - 1 +  (i_t * (i_max_row -2) -i_t))
                    % (i_max_row - 2) + 1 ;

            }
            Direction::Down => {
                c = i_col;
                r =  ((i_row - 1 + i_t) % (i_max_row - 2)) + 1 ;
            }
            Direction::Left => {
                r = i_row;
                // messy to ignore modulo of negative
                c =   (i_col - 1 +  (i_t * (i_max_col -2) -i_t))
                    % (i_max_col - 2) + 1 ;


            }
            Direction::Right => {
                r = i_row;
                c= ((i_col - 1 + i_t) % (i_max_col - 2)) + 1 ;
            }
        }

        return Coord{row: (r + 1)  as usize, col: c as usize};
    }
}

impl Display for Blizz {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       write!(f, "@[row={}, col={}] dir={}", self.row, self.col, self.dir)
    }
}

impl Blizz {
    fn new(r:usize, c:usize, d:char, num_rows: usize, num_cols: usize) -> Blizz {
        let dir = match d
        {
            '^' => {Direction::Up}
            'v' => {Direction::Down}
            '<' => {Direction::Left}
            '>' => {Direction::Right}
            _ => { panic!("bad direction given in Blizz::new -> |{d}|")}
        };
        return Blizz{ row: r, col: c, dir, i_max_row:  num_rows as i32, i_max_col: num_cols as i32 };
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Block {
    Empty,
    Wall,
    EndPoint(usize),
    Blizzard(Direction),
    MBlizz(usize)
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Empty => { write!(f, ".") }
            Block::Wall => { write!(f, "#") }
            Block::EndPoint(n) => { write! {f, "{:1$}", n, 1} }
            Block::Blizzard(d) => { write!(f, "{d}") }
            Block::MBlizz(n) => { write! {f, "{:1$}", n, 1} }
        }
    }
}


fn print_map(a_map: ArrayBase<OwnedRepr<Block>, Ix2>) {
    let d0 = a_map.dim().0;
    let d1 = a_map.dim().1;
    println!("dimensions {} by {}\n ",d0, d1);

    for r in 0..d0 {
        for c in 0..d1 {
            let b = a_map[[r,c]];
            print!("{b}");
        }
        println!();
    }
}
fn map_string(a_map: ArrayBase<OwnedRepr<Block>, Ix2>) -> String {
    let d0 = a_map.dim().0;
    let d1 = a_map.dim().1;
    let mut sb = String::with_capacity((d0*1)* d1);


    for r in 0..d0 {
        for c in 0..d1 {
            let b = a_map[[r,c]];
            //print!("{b}");
            sb.push_str(&*b.to_string());
        }
        sb.push('\n');
    }
    return sb.to_string();
}


fn search(start_time:usize, wall_set: &HashSet<Coord>, start_point: Coord, end_point: Coord, map_index: &BTreeMap<usize, ArrayBase<OwnedRepr<Block>, Ix2>>) -> usize {
    let mut pq: DoublePriorityQueue<State, usize> = DoublePriorityQueue::new();
    pq.push(State { pos: start_point, t: start_time }, start_time);
    let mut found_goal = false;
    let mut steps: Option<usize> = None;
    let mut visited: HashSet<State> = HashSet::new();
    while !found_goal && !pq.is_empty() {
        let (current_state, current_t) = pq.pop_min().unwrap();
        if current_state.pos == end_point {
            found_goal = true;
            steps = Some(current_t - 1);
            break;
        }
        visited.insert(current_state);

        let neighs = current_state.pos.get_neighbors();
        let c_map = map_index.get(&current_t).unwrap();
        for n in neighs {
            match n {
                None => {}
                Some(c) => {
                    if wall_set.contains(&c) {
                        continue;
                    } else {
                        let b = c_map[[c.row, c.col]];
                        if let Block::Empty = b {
                            //empty spot we can move.
                            let new_state = State { pos: c, t: current_t + 1 };
                            if !visited.contains(&new_state) {
                                pq.push(new_state, new_state.t);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("search stopped, found_goal: {}, steps: {:?}, pq.len(): {}", found_goal, steps, pq.len());
    if let Some(n) = steps {
        return n;
    } else {
        return 0;
    }

}

fn generate_map_index(num_rows: usize, num_cols: usize, wall_set: &HashSet<Coord>, blizz_vec: &Vec<Blizz>)
                      -> BTreeMap<usize, ArrayBase<OwnedRepr<Block>, Ix2>> {
    let mut map_vector_init:Vec<Block> = vec![Block::Empty; num_cols*num_rows];
    let mut map_index: BTreeMap<usize, ArrayBase<OwnedRepr<Block>, Ix2>> = BTreeMap::new();

    for min in 0..=MAX_GEN_SIZE {
        let mut a_map = Array2::from_shape_vec((num_rows, num_cols), map_vector_init.clone()).unwrap();

        for w in wall_set.iter() {
            a_map[[w.row, w.col]] = Block::Wall;
        }
        for b in blizz_vec.iter() {
            let p = b.pos_at_time2(min);
            let d = b.dir;
            match a_map[[p.row, p.col]] {
                Block::Empty => { a_map[[p.row, p.col]] = Blizzard(b.dir) }
                Block::Wall => {
                    println!("trying to put blizzard ({d}) at [{},{}], put Wall is already there", p.row, p.col);
                    panic!("invalid placement");
                }
                Block::EndPoint(_) => { panic!("invalid block"); }
                Block::Blizzard(_) => { a_map[[p.row, p.col]] = MBlizz(2); }
                Block::MBlizz(n) => { a_map[[p.row, p.col]] = MBlizz(n + 1); }
            }
        }

        map_index.insert(min, a_map);
    }
    map_index
}


fn parse(split_lines: Vec<&str>) -> (HashSet<Coord>, Vec<Blizz>, Coord, Coord, (usize,usize)) {
    let v1:Vec<Vec<char>> = split_lines.iter().map(|f| f.chars().collect()).collect();

    let num_rows = split_lines.len();
    let num_cols = split_lines[0].len();




    let mut wall_set: HashSet<Coord> = HashSet::new();
    let mut blizz_vec: Vec<Blizz> = Vec::new();


    let mut point_b: Option<Coord> = None;
    let mut point_e: Option<Coord> = None;
    for r in 0..num_rows {
        for c in 0..num_cols {
            match v1[r][c] {
                '#' => { wall_set.insert(Coord { row: r, col: c }); }
                '.' => { /*ignore empty spots*/ }
                'B' => { //begining point
                    point_b = Some(Coord { row: r, col: c });
                }
                'E' => { //ending point
                    point_e = Some(Coord { row: r, col: c });
                }
                d_char => {
                    let b = Blizz::new(r - 1, c, d_char, num_rows - 2, num_cols);
                    blizz_vec.push(b);
                }
            }
        }
    }
    let start_point = point_b.unwrap();
    let end_point = point_e.unwrap();
    (wall_set, blizz_vec, start_point, end_point, (num_rows,num_cols))
}




#[derive(PartialEq, Debug,Eq,Hash)]
struct Room {
    id:i32,
    value:i32,
    connected:Vec<i32>
}
//Valve LW has flow rate=0; tunnels lead to valves AA, HT

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value {} has flow rate={}; tunnels leads to values: {:?}",
               self.id, self.value, self.connected
        )
    }
}


impl Default for Room {
    fn default() -> Room {
        Room {
            id: -1,
            value: -1,
            connected: Vec::new(),
        }
    }
}



fn alpha_to_num(s:&String) -> i32{
    let ch1:u8 = s.chars().nth(0).unwrap() as u8;
    let ch2:u8 = s.chars().nth(1).unwrap() as u8;
    let mut base:i32 = 0;
    let right =  ch1 - ('A' as u8) ;
    if ch1 == ch2 {
        return right as i32;
    } else {
        base = 26;
    }
    let left:i32 =  (ch2 - ('A' as u8)) as i32;
    let p = (right as i32) + (26  * left) as i32 + base ;
    return p;
}


fn generate_alpha_mapping()->HashMap<(char,char),i32> {
    let mut n_to_a: HashMap<(char, char), i32> = HashMap::new();

    for ch1 in 'A'..='Z' {
        for ch2 in 'A'..='Z' {
            let p = (ch1, ch2);
            let mut sb = String::new();
            sb.push(ch1);
            sb.push(ch2);
            let n = alpha_to_num(&sb);
            let r = n_to_a.insert(p, n);
            if r != None {
                panic!("overwrote {sb}");
            }
        }
    }
    return n_to_a;
}


fn parse_room(ln: &mut &str) -> Room {
    let (mut l, mut r) = ln.split_once("=").unwrap();
    // println!("l=|{}|", l);
    let (ch1, ch2) = (l.chars().nth(6).unwrap(), l.chars().nth(7).unwrap());
    let mut s_id = String::with_capacity(2);
    s_id.push(ch1);
    s_id.push(ch2);
    // println!("ch1={ch1}, ch2={ch2}");
    let (mut l, mut r) = r.split_once(";").unwrap();
    let val: i32 = l.parse().unwrap();
    // println!("val= |{}|", val);
    // println!("r=|{r}| pre tunnels");
    let mut v_part;
    if r.contains("tunnels") {
        let (mut l, mut r) = r.split_at(24);
        v_part = r;
        // println!("l= |{l}| r=|{r}|");
    } else {
        let (mut l , mut r) = r.split_at(23);
        v_part = r;
        // println!("l= |{l}| r=|{r}|");
    }

    let parts: Vec<&str> = v_part.split(", ").collect();
    // println!("{:?}", parts);
    let mut v: Vec<i32> = Vec::new();

    for p in parts {
        let n = alpha_to_num(&p.to_string());
        // println!("p=|{p}|");
        // println!("p=|{p}|, n={n}");
        v.push(n);
    }
    // println!("s_id: |{}|", s_id);
    // println!("{:?}", v);

    let mut r=   Room {
        id: alpha_to_num(&s_id),
        value: val,
        connected: v.clone(),
    };
    return r;
}
pub fn factorial(num: u128) -> u128 {
    match num {
        0  => 1,
        1.. => (1..num+1).product(),
    }
}


fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };
    let file = File::open(p1_file).expect(&*format!("error opening file {}", p1_file));
    let bfile = BufReader::new(file);
    let lines:Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();

    if TEST {
        let l_num = lines.len();
        println!("\t read {} lines from {}", l_num, p1_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }









    let answer1 = String::new();
    return answer1.to_string();
}


fn part2() -> String {
    println!("   start part 2");
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };

    let file = File::open(p2_file).expect(&*format!("error opening file {}", p2_file));
    let bfile = BufReader::new(file);
    let lines:Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();

    if TEST {
        let l_num = lines.len();
        println!("\t read {} lines from {}", l_num, p2_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }



    let answer2 = String::new();
    return answer2.to_string();
}
