use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn all_pairs_from_list<T: Clone>(list: Vec<T>) -> Vec<(T, T)> {
    let mut pair_list: Vec<(T, T)> = Vec::new();
    for i in 0..list.len() {
        for j in i + 1..list.len() {
            pair_list.push((list[i].clone(), list[j].clone()));
        }
    }
    return pair_list;
}

pub fn file_to_lines(input_file: &str) -> Vec<String> {
    let file = File::open(input_file).expect(&*format!("error opening file {}", input_file));
    let bfile = BufReader::new(file);
    let lines: Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();
    return lines;
}

pub fn file_to_single_line(input_file: &str, merge_on: Option<char>) -> String {
    let file = File::open(input_file).expect(&*format!("error opening file {}", input_file));
    let bfile = BufReader::new(file);
    let lines: Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();
    if lines.len() == 1 {
        return lines[0].clone();
    } else {
        let mut sb = String::new();
        for i in 0..(lines.len() - 1) {
            sb.push_str(lines[i].as_str());
            match merge_on {
                None => {}
                Some(ch) => {
                    sb.push(ch);
                }
            }
        }
        sb.push_str(lines.last().unwrap().as_str());
        return sb.to_string();
    }
}

pub fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn get_diffs(series: &Vec<i32>) -> Vec<i32> {
    let mut diffs: Vec<i32> = Vec::new();
    if series.len() < 2 {
        return series.clone();
    }
    let mut left = series[0];
    for i in 1..series.len() {
        let d = series[i] - left;
        diffs.push(d);
        left = series[i];
    }
    return diffs;
}

pub fn get_distance_m1((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    return x1.abs_diff(x2) + y1.abs_diff(y2);
}

pub fn get_neighbor_points((x, y): (i32, i32), diag: bool) -> Vec<(i32, i32)> {
    let r = y;
    let c = x;

    let mut neighs: Vec<(i32, i32)> = Vec::new();
    for i in 0..CARD_DELTA.len() {
        let (dr, dc) = CARD_DELTA[i];
        neighs.push((r + dr, c + dc));
    }
    if diag {
        for i in 0..DIAG_DELTA.len() {
            let (dr, dc) = DIAG_DELTA[i];
            neighs.push((c + dc, r + dr));
        }
    }
    return neighs;
}

pub static CARD_DELTA: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
pub static DIAG_DELTA: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

pub fn checked_neighbor_points(
    (x, y): (usize, usize),
    n_rows: usize,
    n_cols: usize,
    diag: bool,
) -> Vec<(usize, usize)> {
    let r = y;
    let c = x;

    let mut neighs: Vec<(usize, usize)> = Vec::new();
    for i in 0..CARD_DELTA.len() {
        let (dr, dc) = CARD_DELTA[i];
        let f1: i32 = (c as i32) + dc;
        let f2: i32 = (r as i32) + dr;

        if (f1 < 0) || (f1 >= n_cols as i32) || (f2 < 0) || (f2 >= n_rows as i32) {
            continue;
        } else {
            neighs.push((f1 as usize, f2 as usize));
        }
    }
    if diag {
        for i in 0..DIAG_DELTA.len() {
            let (dr, dc) = DIAG_DELTA[i];
            let f1: i32 = (c as i32) + dc;
            let f2: i32 = (r as i32) + dr;

            if (f1 < 0) || (f1 >= n_cols as i32) || (f2 < 0) || (f2 >= n_rows as i32) {
                continue;
            } else {
                neighs.push((f1 as usize, f2 as usize));
            }
        }
    }
    neighs.sort();

    return neighs;
}

pub fn group_newline_separated_lines(lines: &Vec<String>) -> Vec<String> {
    let mut group_vec: Vec<String> = Vec::new();
    let mut sb = String::new();
    for i in 0..lines.len() {
        let l = &lines[i];
        if l.len() == 0 {
            group_vec.push(sb);
            sb = String::new();
        } else {
            sb.push_str(l);
            sb.push('\n');
        }
    }
    group_vec.push(sb);
    group_vec
}

pub fn is_all_foo<T: std::cmp::PartialEq>(series: &Vec<T>, element: T) -> bool {
    for n in series {
        if *n != element {
            return false;
        }
    }
    return true;
}

pub fn is_all_zero(series: &Vec<i32>) -> bool {
    return is_all_foo(series, 0);
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

pub fn list_to_pairs<T: Copy>(galaxy_list: Vec<(T, T)>) -> Vec<((T, T), (T, T))> {
    let mut pair_list: Vec<((T, T), (T, T))> = Vec::new();
    for i in 0..galaxy_list.len() {
        for j in i + 1..galaxy_list.len() {
            pair_list.push((galaxy_list[i], galaxy_list[j]));
        }
    }
    return pair_list;
}

pub fn parse_number_list_comma<T: FromStr>(number_string: &str) -> Vec<T> {
    let oo = number_string.split(",").map(|s| s.trim().parse());
    let un_oo: Vec<T> = oo.map(|r| match r {
        Ok(n) => n,
        Err(_) => {
            panic!("Error parsing")
        }
    }).collect();
    return un_oo;
}

pub fn parse_number_list_whitespace<T: FromStr>(number_string: &str) -> Vec<T> {
    let oo = number_string.split_whitespace().map(|s| s.trim().parse());
    let un_oo: Vec<T> = oo.map(|r| match r {
        Ok(n) => n,
        Err(_) => {
            panic!("Error parsing")
        }
    }).collect();
    return un_oo;
}

pub fn str_to_char_vec(s: &str) -> Vec<char> {
    let mut r_vec: Vec<char> = Vec::with_capacity(s.len());
    let b_array = s.as_bytes();
    for i in 0..s.len() {
        let ch: char = b_array[i] as char;
        r_vec.push(ch);
    }
    return r_vec;
}


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum LeftOrRight {
    Left,
    Right,
}

impl Display for LeftOrRight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            LeftOrRight::Left => { 'L' }
            LeftOrRight::Right => { 'R' }
        })
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}


impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   Direction::Up => { "Up" }
                   Direction::Down => { "Down" }
                   Direction::Left => { "Left" }
                   Direction::Right => { "Right" }
               }
        )
    }
}


pub const DIRECTION_ARRAY: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl Direction {
    pub fn to_arrow(&self) -> char {
        match self {
            Direction::Up => {'^'}
            Direction::Down => {'v'}
            Direction::Left => {'<'}
            Direction::Right => {'>'}
        }
    }






    pub fn grid_go_in_dir_rc(&self, (r, c):(usize, usize), max_y: usize, max_x: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if r > 0 {
                    Some((r - 1, c))
                } else {
                    None
                }
            }
            Direction::Down => {
                if r + 1 < max_y {
                    Some((r + 1, c))
                } else {
                    None
                }

            }

            Direction::Left => {
                if c > 0 {
                    Some((r, c - 1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if c +1 < max_x {
                    Some((r, c + 1))
                } else {
                    None
                }

            }
        }
    }


    pub fn grid_offset_in_xy(&self) -> (i32, i32) {
        match self {
            Direction::Up => { (0, -1) }
            Direction::Down => { (0, 1) }
            Direction::Left => { (-1, 0) }
            Direction::Right => { (1, 0) }
        }
    }


    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
    pub fn turn_to(&self, turn_to: LeftOrRight) -> Direction {
        match self {
            Direction::Up => {
                match turn_to {
                    LeftOrRight::Left => { Direction::Left }
                    LeftOrRight::Right => { Direction::Right }
                }
            }
            Direction::Down => {
                match turn_to {
                    LeftOrRight::Left => { Direction::Right }
                    LeftOrRight::Right => { Direction::Left }
                }
            }
            Direction::Left => {
                match turn_to {
                    LeftOrRight::Left => { Direction::Down }
                    LeftOrRight::Right => {
                        Direction::Up
                    }
                }
            }
            Direction::Right => {
                match turn_to {
                    LeftOrRight::Left => { Direction::Up }
                    LeftOrRight::Right => { Direction::Down }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Compass {
    North,
    South,
    West,
    East,
}

impl Compass {
    pub fn opposite(dir: Compass) -> Compass {
        match dir {
            Compass::North => Compass::South,
            Compass::South => Compass::North,
            Compass::West => Compass::East,
            Compass::East => Compass::West,
        }
    }

    pub fn progress(
        (x, y): (usize, usize),
        dir: Compass,
        (max_rows, max_cols): (usize, usize),
    ) -> Option<(usize, usize)> {
        match dir {
            Compass::North if y > 0 => {
                return Some((x, y - 1));
            }
            Compass::South if y + 1 < max_rows => {
                return Some((x, y + 1));
            }
            Compass::West if x > 0 => {
                return Some((x - 1, y));
            }
            Compass::East if x + 1 < max_cols => {
                return Some((x + 1, y));
            }
            _ => {
                return None;
            }
        };
    }

    pub fn turn_to(dir: Compass, turn_to: ForwardDirection) -> Compass {
        match turn_to {
            ForwardDirection::Straight => dir,
            ForwardDirection::Left => match dir {
                Compass::North => Compass::West,
                Compass::South => Compass::East,
                Compass::West => Compass::South,
                Compass::East => Compass::North,
            },
            ForwardDirection::Right => match dir {
                Compass::North => Compass::East,
                Compass::South => Compass::West,
                Compass::West => Compass::North,
                Compass::East => Compass::South,
            },
            ForwardDirection::Back => Compass::opposite(dir),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ForwardDirection {
    Straight,
    Left,
    Right,
    Back,
}

pub fn compare_grid<T: Display>(g1: &Vec<Vec<T>>, g2: &Vec<Vec<T>>) -> () {
    for y in 0..g1.len() {
        for x in 0..g1[0].len() {
            let ch = &g1[y][x];
            print!("{}", *ch);
        }
        print!("\t  \t");
        for x in 0..g2[0].len() {
            let ch = &g2[y][x];
            print!("{}", *ch);
        }
        println!();
    }
}

pub fn equal_grid<T: std::cmp::PartialEq>(g1: &Vec<Vec<T>>, g2: &Vec<Vec<T>>) -> bool {
    if (g1.len() != g2.len()) || (g1[0].len() != g2[0].len()) {
        return false;
    }
    for y in 0..g1.len() {
        for x in 0..g1[0].len() {
            if g1[y][x] != g2[y][x] {
                false;
            }
        }
    }

    return true;
}


pub fn print_grid_with_displayer<T:Copy>(grid: &Vec<Vec<T>>, display: fn(T) -> String) {
    if grid.len() == 0 || grid[0].len() == 0 {
        println!(" Error: trying to print empty grid");
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() - 1 {
            print!("{}", display(grid[y][x]));
        }
       println!("{}", display(*grid[y].last().unwrap()));
    }

}

pub fn print_grid_window<T: std::fmt::Display>(grid: &Vec<Vec<T>>,min_row:usize, min_col:usize, max_row:usize, max_col:usize) -> () {
    if grid.len() == 0 || grid[0].len() == 0 {
        println!(" Error: trying to print empty grid");
    }

    for y in min_row..max_row {
        for x in min_col..max_col {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}

pub fn print_grid<T: std::fmt::Display>(grid: &Vec<Vec<T>>) -> () {
    if grid.len() == 0 || grid[0].len() == 0 {
        println!(" Error: trying to print empty grid");
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() - 1 {
            print!("{}", grid[y][x]);
        }
        println!("{}", grid[y].last().unwrap());
    }
}

pub fn parse_grid(lines: &Vec<String>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for l in lines {
        let line = str_to_char_vec(l);
        grid.push(line);
    }
    grid
}

pub fn convert_grid_using<T: Copy, O>(grid: &Vec<Vec<T>>, convert: fn(T) -> O) -> Vec<Vec<O>> {
    let mut o_grid = Vec::with_capacity(grid.len());
    for row in grid {
        let mut grid_row: Vec<O> = Vec::with_capacity(row.len());
        for r in row {
            grid_row.push(convert(*r));
        }
        o_grid.push(grid_row);
    }
    return o_grid;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}

pub fn list_displayables_to_string<T: Display>(parts: &Vec<T>) -> String {
    let mut sb = String::new();
    if parts.len() == 1 {
        return format!("[{}]", parts[0]);
    }
    sb.push('[');
    for i in 0..parts.len() - 1 {
        sb.push_str(format!("{}, ", parts[i]).as_str());
    }
    sb.push_str(format!("{}]", parts.last().unwrap()).as_str());

    return sb.to_string();
}
