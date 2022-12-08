
use std::fs;
use std::time::Instant;
use ndarray::{Array, Array2, ArrayBase, OwnedRepr, Dim};

/*
    Advent of Code 2022: Day 08
        part1 answer: 1538
        part2 answer: 496125
 */



const TEST: bool = false;

const PART1_TEST_FILENAME: &str = "data/day08/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day08/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day08/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day08/part2_input.txt";

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("08");                           // insert Day

    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();
    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);

    let start2 = Instant::now();
    let answer2 = part2();
    let duration2 = start2.elapsed();
    println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);

    println!("----------\ndone");
}

fn get_scenic_score(tree_array: &Array2<i32>) -> i32 {
    let mut scenic_score: i32 = 0;
    let size = tree_array.dim().0;
    let (mut up, mut down, mut left, mut right): (i32, i32, i32, i32);
    for i in 1..size - 1 {
        for j in 1..size - 1 {
            let tree_height = tree_array[[i, j]];

            up = scenic_direction(&tree_array, Direction::Up, i, j, tree_height);
            down = scenic_direction(&tree_array, Direction::Down, i, j, tree_height);
            left = scenic_direction(&tree_array, Direction::Left, i, j, tree_height);
            right = scenic_direction(&tree_array, Direction::Right, i, j, tree_height);

            if up * right * down * left > scenic_score {
                scenic_score = up * right * down * left;
            }
        }
    }
    return scenic_score;
}

fn scenic_direction(tree_array: &Array2<i32>, dir: Direction, i: usize, j: usize, tree_height: i32) -> i32 {
    let size = tree_array.dim().0;
    let mut s_value = 0;
    match dir {
        Direction::Left => {
            for n in (0..j).rev() {
                s_value += 1;
                if tree_array[[i, n]] < tree_height {
                    continue;
                } else {
                    break;
                }
            }
        }
        Direction::Right => {
            for n in j + 1..size {
                s_value += 1;
                if tree_array[[i, n]] < tree_height {
                    continue;
                } else {
                    break;
                }
            }
        }
        Direction::Up => {
            for n in (0..i).rev() {
                s_value += 1;
                if tree_array[[n, j]] < tree_height {
                    continue;
                } else {
                    break;
                }
            }
        }
        Direction::Down => {
            for n in i + 1..size {
                s_value += 1;
                if tree_array[[n, j]] < tree_height {
                    continue;
                } else {
                    break;
                }
            }
        }
    }
    return s_value;
}

fn sweep_horizontal(tree_array: &Array2<i32>, vis: &mut ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>) {
    let mut local_max_left: i32;
    let mut local_max_right: i32;
    let size = tree_array.dim().0;
    // sweep left -> right
    for i in 0..size {
        local_max_left = tree_array[[i, 0]];
        local_max_right = tree_array[[i, size - 1]];
        vis[[i, 0]] = 1;
        vis[[i, size - 1]] = 1;
        //left->right
        for j in 1..size {
            if tree_array[[i, j]] > local_max_left {
                local_max_left = tree_array[[i, j]];
                vis[[i, j]] = 1;
            }
        }
        //right->left
        for j in (0..size).rev() {
            if tree_array[[i, j]] > local_max_right {
                local_max_right = tree_array[[i, j]];
                vis[[i, j]] = 1;
            }
        }
    }
}

fn sweep_vertical(tree_array: &Array2<i32>, vis: &mut ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>) {
    let mut local_max_up: i32;
    let mut local_max_down: i32;
    let size = tree_array.dim().0;

    for i in 0..size {
        local_max_up = tree_array[[0, i]];
        vis[[0, i]] = 1;
        local_max_down = tree_array[[size - 1, i]];
        vis[[size - 1, i]] = 1;
        // sweep top -> bottom
        for j in 0..size {
            if tree_array[[j, i]] > local_max_up {
                local_max_up = tree_array[[j, i]];
                vis[[j, i]] = 1;
            }
        }
        // sweep bottom -> top
        for j in (0..size).rev() {
            if tree_array[[j, i]] > local_max_down {
                local_max_down = tree_array[[j, i]];
                vis[[j, i]] = 1;
            }
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
    let mut lines: Vec<&str> = data1_s.trim().split("\n").collect();
    lines = lines.iter().map(|l| l.trim()).collect();
    let l_num = lines.len();

    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
    }

    let size = lines[0].len();
    let mut sbuffer = String::new();

    for ln in lines {
        sbuffer.push_str(ln);
    }
    let mut n_array: Vec<i32> = Vec::new();
    let b_chars = sbuffer.chars();
    for ch in b_chars {
        if let Some(digit) = ch.to_digit(10) {
            n_array.push(digit as i32);
        }
    }
    let mut tree_array: Array2<i32> = Array2::from_shape_vec((size, size), n_array).unwrap();

    let mut vis: ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>> = Array::zeros((size, size));

    sweep_horizontal(&tree_array, &mut vis);
    sweep_vertical(&mut tree_array, &mut vis);

    let mut vis_count = 0;
    for e in &vis {
        if *e == 1 {
            vis_count += 1;
        }
    }

    let answer1 = vis_count.to_string();
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
    lines = lines.iter().map(|l| l.trim()).collect();
    let l_num = lines.len();

    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
    }

    let size = lines[0].len();
    let mut sbuffer = String::new();
    for ln in lines {
        sbuffer.push_str(ln);
    }
    let mut n_array: Vec<i32> = Vec::new();
    let b_chars = sbuffer.chars();
    for ch in b_chars {
        if let Some(digit) = ch.to_digit(10) {
            n_array.push(digit as i32);
        }
    }

    let tree_array: Array2<i32> = Array2::from_shape_vec((size, size), n_array).unwrap();
    let scenic = get_scenic_score(&tree_array);
    let answer2 = scenic.to_string();

    return answer2;
}
