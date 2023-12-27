use std::collections::{BTreeMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs;
use std::time::Instant;

use ndarray::{Array2, ArrayBase, Ix2, OwnedRepr};
use priority_queue::DoublePriorityQueue;

/*
    Advent of Code 2022: Day 24`
        part1 answer:   301
        part2 answer:   859

 */

const TEST_ANSWER: (i64, i64) = (18, 54);
const INPUT_ANSWER: (i64, i64) = (301, 859);

const PART1_TEST_FILENAME: &str = "data/day24/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day24/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day24/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day24/part2_input.txt";

const TEST: bool = false;
const MAX_GEN_SIZE: usize = 1000;

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


#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Blizzard {
    row: usize,
    col: usize,
    dir: Direction,
    i_max_row: i32,
    i_max_col: i32,
}

impl Display for Blizzard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@[row={}, col={}] dir={}", self.row, self.col, self.dir)
    }
}
impl Blizzard {
    fn new(r: usize, c: usize, d: char, num_rows: usize, num_cols: usize) -> Blizzard {
        let dir = match d {
            '^' => { Direction::Up }
            'v' => { Direction::Down }
            '<' => { Direction::Left }
            '>' => { Direction::Right }
            _ => { panic!("bad direction given in Blizzard::new -> |{d}|") }
        };
        return Blizzard { row: r, col: c, dir, i_max_row: num_rows as i32, i_max_col: num_cols as i32 };
    }
    fn pos_at_time(&self, t: usize) -> Coord {
        let  r:i32;
        let  c:i32;
        let i_t = t as i32;
        let (i_max_row, i_max_col) = (self.i_max_row, self.i_max_col);
        let (i_row, i_col) = (self.row as i32, self.col as i32);


        match self.dir {
            Direction::Up => {
                c = i_col;
                // messy to ignore modulo of negative
                r = (i_row - 1 + (i_t * (i_max_row - 2) - i_t)) % (i_max_row - 2) + 1;
            }
            Direction::Down => {
                c = i_col;
                r = ((i_row - 1 + i_t) % (i_max_row - 2)) + 1;
            }
            Direction::Left => {
                r = i_row;
                // messy to ignore modulo of negative
                c = (i_col - 1 + (i_t * (i_max_col - 2) - i_t)) % (i_max_col - 2) + 1;
            }
            Direction::Right => {
                r = i_row;
                c = ((i_col - 1 + i_t) % (i_max_col - 2)) + 1;
            }
        }

        return Coord { row: (r + 1) as usize, col: c as usize };
    }
}



#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Block {
    Empty,
    Wall,
    Blizzard(Direction),
    MBlizzard(usize),
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Empty => { write!(f, ".") }
            Block::Wall => { write!(f, "#") }
            Block::Blizzard(d) => { write!(f, "{d}") }
            Block::MBlizzard(n) => { write! {f, "{:1$}", n, 1} }
        }
    }
}


#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   Direction::Up => { "^" }
                   Direction::Down => { "v" }
                   Direction::Left => { "<" }
                   Direction::Right => { ">" }
               })
    }
}

impl Direction {
    fn iterator() -> impl Iterator<Item=Direction> {
        [Direction::Up, Direction::Down, Direction::Right, Direction::Left].iter().copied()
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@[row={}, col={}]", self.row, self.col)
    }
}

impl Coord {
    fn get_neighbors(&self) -> [Option<Coord>; 5] {
        let mut result: [Option<Coord>; 5] = Default::default();
        let mut i = 0;
        for d in Direction::iterator() {
            result[i] = self.step(d);
            i += 1;
        }
        result[i] = Some(self.clone());
        return result;
    }
    fn step(&self, d: Direction) -> Option<Coord> {
        let (r, c) = (self.row, self.col);
        return match d {
            Direction::Up => {
                if r == 0 {
                    None
                } else {
                    Some(Coord { row: r - 1, col: c })
                }
            }
            Direction::Down => {
                Some(Coord { row: r + 1, col: c })
            }
            Direction::Left => {
                if c == 0 {
                    None
                } else {
                    Some(Coord { row: r, col: c - 1 })
                }
            }
            Direction::Right => {
                Some(Coord { row: r, col: c + 1 })
            }
        };
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    pos: Coord,
    t: usize,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "time {} {}", self.t, self.pos)
    }
}

fn generate_map_index(num_rows: usize, num_cols: usize, wall_set: &HashSet<Coord>, blizzard_vec: &Vec<Blizzard>)
                      -> BTreeMap<usize, ArrayBase<OwnedRepr<Block>, Ix2>> {
    let map_vector_init: Vec<Block> = vec![Block::Empty; num_cols * num_rows];
    let mut map_index: BTreeMap<usize, ArrayBase<OwnedRepr<Block>, Ix2>> = BTreeMap::new();

    for min in 0..=MAX_GEN_SIZE {
        let mut a_map = Array2::from_shape_vec((num_rows, num_cols), map_vector_init.clone()).unwrap();

        for w in wall_set.iter() {
            a_map[[w.row, w.col]] = Block::Wall;
        }
        for b in blizzard_vec.iter() {
            let p = b.pos_at_time(min);
            let d = b.dir;
            match a_map[[p.row, p.col]] {
                Block::Empty => { a_map[[p.row, p.col]] = Block::Blizzard(b.dir) }
                Block::Wall => {
                    println!("trying to put blizzard ({d}) at [{},{}], put Wall is already there", p.row, p.col);
                    panic!("invalid placement");
                }
                Block::Blizzard(_) => { a_map[[p.row, p.col]] = Block::MBlizzard(2); }
                Block::MBlizzard(n) => { a_map[[p.row, p.col]] = Block::MBlizzard(n + 1); }
            }
        }

        map_index.insert(min, a_map);
    }
    map_index
}

fn map_string(a_map: ArrayBase<OwnedRepr<Block>, Ix2>) -> String {
    #![allow(dead_code)]
    let d0 = a_map.dim().0;
    let d1 = a_map.dim().1;
    let mut sb = String::with_capacity((d0 * 1) * d1);


    for r in 0..d0 {
        for c in 0..d1 {
            let b = a_map[[r, c]];
            //print!("{b}");
            sb.push_str(&*b.to_string());
        }
        sb.push('\n');
    }
    return sb.to_string();
}

fn parse(split_lines: Vec<&str>) -> (HashSet<Coord>, Vec<Blizzard>, Coord, Coord, (usize, usize)) {
    let v1: Vec<Vec<char>> = split_lines.iter().map(|f| f.chars().collect()).collect();

    let num_rows = split_lines.len();
    let num_cols = split_lines[0].len();


    let mut wall_set: HashSet<Coord> = HashSet::new();
    let mut blizzard_vec: Vec<Blizzard> = Vec::new();


    let mut point_b: Option<Coord> = None;
    let mut point_e: Option<Coord> = None;
    for r in 0..num_rows {
        for c in 0..num_cols {
            match v1[r][c] {
                '#' => { wall_set.insert(Coord { row: r, col: c }); }
                '.' => { /*ignore empty spots*/ }
                'B' => { //beginning point
                    point_b = Some(Coord { row: r, col: c });
                }
                'E' => { //ending point
                    point_e = Some(Coord { row: r, col: c });
                }
                d_char => {
                    let b = Blizzard::new(r - 1, c, d_char, num_rows - 2, num_cols);
                    blizzard_vec.push(b);
                }
            }
        }
    }
    let start_point = point_b.unwrap();
    let end_point = point_e.unwrap();
    (wall_set, blizzard_vec, start_point, end_point, (num_rows, num_cols))
}

fn print_map(a_map: ArrayBase<OwnedRepr<Block>, Ix2>) {
    #![allow(dead_code)]
    let d0 = a_map.dim().0;
    let d1 = a_map.dim().1;
    println!("dimensions {} by {}\n ", d0, d1);

    for r in 0..d0 {
        for c in 0..d1 {
            let b = a_map[[r, c]];
            print!("{b}");
        }
        println!();
    }
}

fn search(start_time: usize, wall_set: &HashSet<Coord>, start_point: Coord, end_point: Coord,
          map_index: &BTreeMap<usize, ArrayBase<OwnedRepr<Block>, Ix2>>) -> Option<usize> {
    let mut pq: DoublePriorityQueue<State, usize> = DoublePriorityQueue::new();
    pq.push(State { pos: start_point, t: start_time }, start_time);
    let mut steps: Option<usize> = None;
    let mut visited: HashSet<State> = HashSet::new();
    while  !pq.is_empty() {
        let (current_state, current_t) = pq.pop_min().unwrap();
        if current_state.pos == end_point {
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
    return steps;
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

    let (wall_set, blizzard_vector, start_point, end_point, (num_rows, num_cols)) = parse(split_lines);

    let map_index = generate_map_index(num_rows, num_cols, &wall_set, &blizzard_vector);
    let steps = search(0, &wall_set, start_point, end_point, &map_index);

    let answer1 = steps.unwrap();
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

    let (wall_set, blizzard_vec, start_point, end_point, (num_rows, num_cols)) = parse(split_lines);
    let map_index = generate_map_index(num_rows, num_cols, &wall_set, &blizzard_vec);

    let mut total_minutes = 0;
    let first_trip = search(total_minutes, &wall_set, start_point, end_point, &map_index).unwrap();
    total_minutes += first_trip;

    let second_trip = search(total_minutes, &wall_set, end_point, start_point, &map_index).unwrap() - first_trip;
    total_minutes += second_trip;


    let third_trip = search(total_minutes, &wall_set, start_point, end_point, &map_index).unwrap() - total_minutes;


    let total_minutes = first_trip + second_trip + third_trip;
    let answer2 = total_minutes.to_string();
    return answer2.to_string();
}