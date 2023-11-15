#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]


use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::BufWriter;
use std::io::Write;
use std::path::Component::ParentDir;
use std::process::exit;
use std::time::Instant;
use ndarray::{Array2, ArrayBase, Ix2, OwnedRepr};
use priority_queue::{DoublePriorityQueue, PriorityQueue};
use crate::Block::{Blizzard, MBlizz};

/*
    Advent of Code 2022: Day 24`
        part1 answer: 301
        part2 answer:

p1: 303 -- too high
p2: 860 -- too high
 */


const TEST_ANSWER: (i64, i64) = (18, 110);
// 25.652ms
const INPUT_ANSWER: (i64, i64) = (301, 859);

const PART1_TEST_FILENAME: &str = "data/day24/part1_test.txt";
const PART1B_TEST_FILENAME: &str = "data/day24/part1b_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day24/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day24/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day24/part2_input.txt";

const TEST: bool = false;


fn main() {
    print!("Advent of Code 2022, Day ");
    println!("24");                           // insert Day


    // let start1 = Instant::now();
    // let answer1 = part1();
    // let duration1 = start1.elapsed();

    // println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);
    // if TEST {
    //     if answer1 != TEST_ANSWER.0.to_string() {
    //         println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
    //                  answer1, TEST_ANSWER.0.to_string())
    //     }
    // } else {
    //     if answer1 != INPUT_ANSWER.0.to_string() {
    //         println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
    //                  answer1, INPUT_ANSWER.0.to_string())
    //     }
    // }

    let start2 = Instant::now();
    let answer2 = part2();
    let duration2 = start2.elapsed();

    println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);


    if TEST {
        if answer2 != TEST_ANSWER.1.to_string() {
            println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
                     answer2,TEST_ANSWER.1.to_string() )
        }
    } else {
        if answer2 != INPUT_ANSWER.1.to_string() {
            println!("\t\t ERROR: Answer is WRONG. Got: {} , Expected {}",
                     answer2,TEST_ANSWER.1.to_string() )
        }
    }


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

fn part1() -> String {
    println!("\t start part 1");
    let p1_file = match TEST {
        true => PART1B_TEST_FILENAME,
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

    let mut row_bs:Vec<HashSet<Blizz>> = Vec::with_capacity(num_rows);
    for r in 0..num_rows {
        let mut bs:HashSet<Blizz> = HashSet::new();
        row_bs.insert(r,bs);
    }


    let mut col_bs:Vec<HashSet<Blizz>> = Vec::with_capacity(num_cols);
    for c in 0..num_cols {
        let mut bs:HashSet<Blizz> = HashSet::new();
        col_bs.insert(c,bs);
    }



    for r in 0..num_rows {
        for c in 0..num_cols {
            match v1[r][c] {
                '#' => {wall_set.insert(Coord{row: r, col: c});}
                '.' => {/*ignore empty spots*/ }
                d_char => {
                   let b = Blizz::new(r,c,d_char, num_rows, num_cols);
                    coord_to_blizz.insert(Coord{row: r, col: c}, b);
                    blizz_vec.push(b);
                    if b.dir == Direction::Left || b.dir == Direction::Right {
                        row_bs[b.row].insert(b);
                    } else {
                        col_bs[b.col].insert(b);
                    }
                }
            }
        }
    }




    println!("found {} blizzards", blizz_vec.len());


    let mut found_goal=false;

    let mut open_places:VecDeque<State> = VecDeque::new();
    let mut visited:HashSet<State> = HashSet::new();
    open_places.push_back(State{pos: start_coord, t: 0});
    let goal_loc = end_coord;
    let mut goal_time = 0;
let mut count_timeouts = 0;

    let start_neightbors =
                        [start_coord.step(Direction::Down),None, None, None, Some(start_coord)];
    println!("init open_places: {:?}", open_places);
//let mut it= 0;
   while !found_goal && !open_places.is_empty() {
       //it += 1;
        let current = open_places.pop_front().unwrap();

  //         println!("current State: {:?},  V={}, O={}", current, visited.len(), open_places.len());

      // println!("visited: {:?}", visited);
      // println!("open_places: {:?}", open_places);

        visited.insert(current);
        if current.pos == end_coord {
            println!("found goal with State: {:?}", current);
            goal_time = current.t;
            found_goal = true;
            break;
        }
        let neighbors =
            if current.pos == start_coord{start_neightbors }
            else {current.pos.get_neighbors()};
        let new_t: usize = current.t + 1;

        for n in neighbors {
            match n {
                Some(co) => {
                    if wall_set.contains(&co) {
                        continue;
                    } else if new_t > 302 {
                        count_timeouts += 1;
                        continue;
                    } else {
                        let s = State { pos: co, t: new_t };
                        if !visited.contains(&s) {
                            let mut b_check_good = true;



                            // for b in &blizz_vec {
                            //     if b.pos_at_time(new_t) == co {
                            //         b_check_good = false;
                            //         break;
                            //     }
                            // }

                            for b in &col_bs[co.col] {
                                if b.pos_at_time(new_t) == co {
                                    b_check_good = false;
                                    break;
                                }
                            }
                            if b_check_good {
                                for b in &row_bs[co.row] {
                                    if b.pos_at_time(new_t) == co {
                                        b_check_good = false;
                                        break;
                                    }
                                }
                            }


                            if b_check_good {
                                if co==current.pos {
                                    open_places.push_front(s);
                                } else {
                                    open_places.push_back(s);
                                }
                            }
                        }
                    }
                },
                None => {}
            }
        }

   }
    println!("Reached {} in {} steps", goal_loc, goal_time);

    println!("timeouts: {}", count_timeouts);


    let answer1 = goal_time.to_string();
    return answer1.to_string();
}


fn part2() -> String {
    println!("\t start part 2");
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

    let lines: Vec<&str> = data2_s.trim().split("\n").collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }
    let data1_ss = data2_s.trim();
    let split_lines: Vec<&str> = data1_ss.split(LINE_ENDING).collect();

    let num_rows = split_lines.len();
    let num_cols = split_lines[0].len();

    let dims = (num_rows, num_cols);
    println!("rows: {}, cols: {}", num_rows, num_cols);




    let dims = ( num_rows,  num_cols);

    println!("grid dimensions: {:?}", dims);
    let v1:Vec<Vec<char>> = split_lines.iter().map(|f| f.chars().collect()).collect();

    let mut wall_set:HashSet<Coord> = HashSet::new();
    let mut blizz_vec:Vec<Blizz> = Vec::new();


    let mut point_b:Option<Coord> = None;
    let mut point_e:Option<Coord> = None;
    println!();
    for r in 0..num_rows {
        for c in 0..num_cols {
            match v1[r][c] {
                '#' => {wall_set.insert(Coord{row: r, col: c});}
                '.' => {/*ignore empty spots*/ }
                'B' => { //begining point
                    point_b = Some(Coord{row: r, col: c});
                }
                'E' => { //ending point
                    point_e = Some(Coord{row: r, col: c});
                }
                d_char => {
                    let b = Blizz::new(r-1,c,d_char, num_rows-2, num_cols);
                    blizz_vec.push(b);
                }
            }
        }
    }
    let start_point = point_b.unwrap();
    let end_point = point_e.unwrap();


    println!("found {} blizzards", blizz_vec.len());
    println!("start point: {}", start_point);
    println!("end   point: {}", end_point);


    let mut found_goal=false;

    let mut open_places:VecDeque<State> = VecDeque::new();
    let mut visited:HashSet<State> = HashSet::new();

    let mut goal_time = 0;
    let mut count_timeouts = 0;

    let mut time=0;



    let mut map_vector_init:Vec<Block> = vec![Block::Empty; num_cols*num_rows];
    println!("allocated map_vector_init of length : {}", map_vector_init.len());

   let mut map_vec:Vec<String> = Vec::with_capacity(1001);

    const MAX_GEN_SIZE:usize = 1000;


    let mut map_index:BTreeMap<usize,ArrayBase<OwnedRepr<Block>, Ix2>> = BTreeMap::new();

 for min in 0..=MAX_GEN_SIZE {

        let mut a_map = Array2::from_shape_vec((num_rows,num_cols), map_vector_init.clone()).unwrap();
      //  a_map[[start_point.row, start_point.col]] = Block::EndPoint(0);
      //  a_map[[end_point.row, end_point.col]] = Block::EndPoint(1);

        for w in &wall_set {
            a_map[[w.row,w.col]] = Block::Wall;
        }
        for b in blizz_vec.iter() {
            let p = b.pos_at_time2(min);
            let d = b.dir;
            match a_map[[p.row, p.col]] {
                Block::Empty => {a_map[[p.row, p.col]] = Blizzard(b.dir)}
                Block::Wall => {
                    println!("trying to put blizzard ({d}) at [{},{}], put Wall is already there", p.row, p.col);
                    panic!("invalid placement");
                }
                Block::EndPoint(_) => {panic!("invalid block");}
                Block::Blizzard(_) => { a_map[[p.row, p.col]] = MBlizz(2); }
                Block::MBlizz(n) => {a_map[[p.row, p.col]] = MBlizz(n+1); }
            }
        }

        map_index.insert(min, a_map);
     }

    let map_index = map_index;
    println!("map_index has {} elements stored", map_index.len());
    let mut pq:DoublePriorityQueue<State,usize> = DoublePriorityQueue::new();

    pq.push(State{pos:start_point,t: 0}, 0);

    let mut found_goal = false;

    let mut steps:Option<usize> = None;
    let mut visited:HashSet<State> = HashSet::new();
    while !found_goal && !pq.is_empty()  {
        let (current_state,current_t) =pq.pop_min().unwrap();
        if current_state.pos == end_point {
            found_goal = true;
            steps = Some(current_t-1);
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
                        let b = c_map[[c.row,c.col]];
                        if let Block::Empty = b {
                            //empty spot we can move.
                            let new_state = State{pos: c, t: current_t + 1};
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








    let answer2 = String::new();
    return answer2.to_string();
}
