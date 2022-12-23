use std::cmp;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs;
use std::time::Instant;
use std::fmt;

/*
    Advent of Code 2022: Day 17
        part1 answer: 3059
        part2 answer: 1500874635587

 */

const TEST_ANSWER: (i64, i64) = (3068, 1514285714288);
const INPUT_ANSWER: (i64, i64) = (3059, 1500874635587);

const PART1_TEST_FILENAME: &str = "data/day17/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day17/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day17/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day17/part2_input.txt";

const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("17");                           // insert Day

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

const PART1_ITERATIONS: u64 = 2022;
const PART2_ITERATIONS: u64 = 1_000_000_000_000;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Block {
    HLine,
    Cross,
    Corner,
    VLine,
    Square,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Block::HLine => { write!(f, "hort-bar") }
            Block::Cross => { write!(f, "cross") }
            Block::Corner => { write!(f, "corner") }
            Block::VLine => { write!(f, "vert-bar") }
            Block::Square => { write!(f, "square") }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    // Up isn't need for this problem
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::Down => write!(f, "🠗"),
            Direction::Left => write!(f, "🠔"),
            Direction::Right => write!(f, "➝"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    jet_index: usize,
    block_index: usize,
    field_value: u8,
}

// Test if piece can move in the given direction
fn can_move(field: &[u8], r: Block, dir: Direction, row: usize, col: usize) -> bool {
    match dir {
        Direction::Down => {
            0 < row
                && match r {
                Block::HLine => {
                    field[row - 1] & (0xF << (7 - col - 3)) == 0
                }
                Block::Cross => {
                    field[row - 1] & (1 << (7 - col - 1)) | field[row] &
                        (0x5 << (7 - col - 2)) == 0
                }
                Block::Corner => {
                    field[row - 1] & (0x7 << (7 - col - 2)) == 0
                }
                Block::VLine => {
                    field[row - 1] & (1 << (7 - col)) == 0
                }
                Block::Square => {
                    field[row - 1] & (0x3 << (7 - col - 1)) == 0
                }
            }
        }
        Direction::Left => {
            0 < col &&
                match r {
                    Block::HLine => {
                        field[row] & (1 << (7 - col + 1)) == 0
                    }
                    Block::Cross => {
                        (field[row] | field[row + 2])
                            & (1 << (7 - col))
                            | field[row + 1]
                            & (1 << (7 - col + 1))
                            == 0
                    }
                    Block::Corner => {
                        field[row] & (1 << (7 - col + 1)) |
                            (field[row + 1] | field[row + 2]) &
                                (1 << (7 - col - 1)) == 0
                    }
                    Block::VLine => {
                        (field[row] | field[row + 1] | field[row + 2] | field[row + 3])
                            & (1 << (7 - col + 1))
                            == 0
                    }
                    Block::Square => {
                        (field[row] | field[row + 1]) & (1 << (7 - col + 1)) == 0
                    }
                }
        }
        Direction::Right => {
            match r {
                Block::HLine => col + 4 < 7 && field[row] & (1 << (7 - col - 4)) == 0,
                Block::Cross => {
                    col + 3 < 7
                        && (field[row] | field[row + 2]) & (1 << (7 - col - 2))
                        | field[row + 1] & (1 << (7 - col - 3))
                        == 0
                }
                Block::Corner => {
                    col + 3 < 7
                        && (field[row] | field[row + 1] | field[row + 2]) & (1 << (7 - col - 3)) == 0
                }
                Block::VLine => {
                    col + 1 < 7
                        && (field[row] | field[row + 1] | field[row + 2] | field[row + 3])
                        & (1 << (7 - col - 1))
                        == 0
                }
                Block::Square => {
                    col + 2 < 7 && (field[row] | field[row + 1]) & (1 << (7 - col - 2)) == 0
                }
            }
        }
    }
}

// Piece can no longer move, come up with new board state for next iteration
fn freeze(field: &mut [u8], r: Block, row: usize, col: usize) -> usize {
    match r {
        Block::HLine => {
            field[row] |= 0xF << (7 - col - 3);
            row
        }
        Block::Cross => {
            field[row] |= 1 << (7 - col - 1);
            field[row + 1] |= 0x7 << (7 - col - 2);
            field[row + 2] |= 1 << (7 - col - 1);
            row + 2
        }
        Block::Corner => {
            field[row] |= 0x7 << (7 - col - 2);
            field[row + 1] |= 1 << (7 - col - 2);
            field[row + 2] |= 1 << (7 - col - 2);
            row + 2
        }
        Block::VLine => {
            field[row] |= 1 << (7 - col);
            field[row + 1] |= 1 << (7 - col);
            field[row + 2] |= 1 << (7 - col);
            field[row + 3] |= 1 << (7 - col);
            row + 3
        }
        Block::Square => {
            field[row] |= 3 << (7 - col - 1);
            field[row + 1] |= 3 << (7 - col - 1);
            row + 1
        }
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

    let mut field: Vec<u8> = vec![0u8; 10];
    let mut highest_row: usize = 0;
    let jets = lines[0].trim_end().as_bytes();
    let mut jet_index: usize = 0;
    let mut block_index: usize = 0;

    let last_turn = PART1_ITERATIONS;

    'turn: for current_turn in 0..last_turn {
        let mut bottom = highest_row + 3;
        for _ in field.len()..bottom + 4 {
            field.push(0);
        }
        let mut left = 2;

        let r = match current_turn % 5 {
            0 => Block::HLine,
            1 => Block::Cross,
            2 => Block::Corner,
            3 => Block::VLine,
            4 => Block::Square,
            x => panic!("invalid block in match, {:?}", x),
        };
        block_index += 1;
        block_index = block_index % 5;
        bottom -= 3;
        for _ in 0..3 {
            match jets[jet_index % jets.len()] {
                60 => { //left
                    if can_move(&field, r, Direction::Left, bottom, left) {
                        left -= 1;
                    }
                }
                62 => { //right
                    if can_move(&field, r, Direction::Right, bottom, left) {
                        left += 1;
                    }
                }
                _ => unreachable!(),
            }
            jet_index += 1;
            jet_index = jet_index % jets.len();
        }

        loop {
            match jets[jet_index % jets.len()] {
                60 => { //left
                    if can_move(&field, r, Direction::Left, bottom, left) {
                        left -= 1
                    }
                }
                62 => { //right
                    if can_move(&field, r, Direction::Right, bottom, left) {
                        left += 1
                    }
                }
                _ => unreachable!(),
            }
            jet_index += 1;

            let dir = Direction::Down;
            if !can_move(&field, r, dir, bottom, left) {
                let row = freeze(&mut field, r, bottom, left);
                highest_row = cmp::max(row + 1, highest_row);


                continue 'turn;
            } else {
                bottom -= 1;
            }
        }
    }

    let answer1 = highest_row as u32;
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
    }

    let mut field = vec![0u8; 10];
    let mut shape = [0usize; 7];
    let mut highest_row = 0usize;
    let jets = data2_s.trim_end().as_bytes();
    let mut jet_index = 0usize;
    let mut previously_seen_on: HashMap<([usize; 7], usize, Block), (u64, u64)> = HashMap::default();
    let mut extra_part = 0u64;

    let last_turn = PART2_ITERATIONS;

    let mut current_turn = 0;
    'turn: while current_turn < last_turn {
        let mut bottom = highest_row + 3;
        for _ in field.len()..bottom + 4 {
            field.push(0);
        }
        let mut left = 2;
        let r = match current_turn % 5 {
            0 => Block::HLine,
            1 => Block::Cross,
            2 => Block::Corner,
            3 => Block::VLine,
            4 => Block::Square,
            x => panic!("invalid block in match, {:?}", x),
        };

        bottom -= 3;
        for _ in 0..3 {
            match jets[jet_index % jets.len()] {
                60 => {
                    if can_move(&field, r, Direction::Left, bottom, left) {
                        left -= 1
                    }
                }
                62 => {
                    if can_move(&field, r, Direction::Right, bottom, left) {
                        left += 1
                    }
                }
                x => panic!("invalid jet: {}", x)
            }
            jet_index += 1;
        }
        loop {
            if jets[jet_index % jets.len()] == 60 {
                if can_move(&field, r, Direction::Left, bottom, left) {
                    left -= 1
                }
            } else {
                if can_move(&field, r, Direction::Right, bottom, left) {
                    left += 1
                }
            }
            jet_index += 1;

            let dir = Direction::Down;
            if can_move(&field, r, dir, bottom, left) {
                bottom -= 1;
                continue;
            }
            let row = freeze(&mut field, r, bottom, left);
            match r {
                Block::HLine => {
                    shape[left] = shape[left].max(bottom + 1);
                    shape[left + 1] = shape[left + 1].max(bottom + 1);
                    shape[left + 2] = shape[left + 2].max(bottom + 1);
                    shape[left + 3] = shape[left + 3].max(bottom + 1);
                }
                Block::Cross => {
                    shape[left] = shape[left].max(bottom + 2);
                    shape[left + 1] = shape[left + 1].max(bottom + 3);
                    shape[left + 2] = shape[left + 2].max(bottom + 2);
                }
                Block::Corner => {
                    shape[left] = shape[left].max(bottom + 1);
                    shape[left + 1] = shape[left + 1].max(bottom + 1);
                    shape[left + 2] = shape[left + 2].max(bottom + 3);
                }
                Block::VLine => {
                    shape[left] = shape[left].max(bottom + 4);
                }
                Block::Square => {
                    shape[left] = shape[left].max(bottom + 2);
                    shape[left + 1] = shape[left + 1].max(bottom + 2);
                }
            }

            let mut k = shape;
            let m = shape.iter().min().unwrap();
            for v in k.iter_mut() {
                *v -= m;
            }
            if row + 1 > highest_row {
                highest_row = row + 1;
            }
            if extra_part == 0 {
                let e = previously_seen_on.entry((k, jet_index % jets.len(), r));
                e.and_modify(|(j, h)| {
                    let step = current_turn - *j;
                    let count = (last_turn - current_turn) / step;
                    current_turn += step * count;
                    extra_part = (highest_row as u64 - *h) * count;
                })
                    .or_insert((current_turn, highest_row as u64));
            }
            current_turn += 1;
            continue 'turn;
        }
    }

    let answer2 = highest_row as u64 + extra_part;
    let answer2 = answer2.to_string();
    return answer2;
}
