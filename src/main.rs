use std::fs;
use std::time::Instant;

/*
    Advent of Code 2022: Day 20
        part1 answer: 13289
        part2 answer: 2865721299243
 */

const TEST_ANSWER: (i64, i64) = (3, 1623178306);
const INPUT_ANSWER: (i64, i64) = (13289, 2865721299243);

const PART1_TEST_FILENAME: &str = "data/day20/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day20/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day20/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day20/part2_input.txt";

const DECRYPTION_KEY: (i64, i64) = (1, 811589153);
const ITERATIONS: (usize, usize) = (1, 10);

const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("20");                           // insert Day


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


#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";


#[derive(Clone)]
struct Number {
    original_index: usize,
    move_by: i64,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.original_index == other.original_index
    }
}

//generic is used so that the iter mess works properly
fn shift_element<T>(vec: &Vec<T>,
                    index: usize,
                    offset: i64) -> Vec<T>
    where T: Clone + PartialEq
{
    let len = vec.len() as i64;

    // concat three copies of the list
    let mut tripled: Vec<T> = Vec::with_capacity(vec.len() * 3);
    for _ in 0..3 {
        for v in vec {
            tripled.push(v.clone());
        }
    }
    let tripled = tripled; //drop mutability

    //wrap offset into tripled lists
    // just mods by len-1, but does witchcraft to handle mod of negative value
    let offset = (offset % (len - 1) + len) % len
        + if offset > 0 { 1 } else { 0 };

    //build return vector
    [
        // make vec of target value
        vec![vec[index].clone()],
        tripled.into_iter()
            .skip(index + offset as usize)
            .filter(|el| *el != vec[index])
            .take(vec.len() - 1)
            .collect()
    ].concat()
}


fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };
    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));

    let lines: Vec<&str> = data1_s.trim().split(LINE_ENDING).collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}\n", lines[0].len());
        }
    }

    let decryption_key: i64 = DECRYPTION_KEY.0;
    let iterations: usize = ITERATIONS.0;
    let orig_numbers: Vec<i64> = lines.into_iter().map(|n| n.parse::<i64>().unwrap()).collect();

    let mut seq: Vec<Number> = orig_numbers.iter()
        .enumerate()
        .map(|(original_index, &move_by)|
            Number {
                original_index,
                move_by: move_by * decryption_key,
            })
        .collect();
    //  println!("seq: {:?}", seq);

    let len = seq.len();

    for _ in 0..iterations {
        for i in 0..len {
            let index = seq.iter().position(
                |num| num.original_index == i).unwrap();

            let offset = seq[index].move_by;
            let shifted = shift_element(&seq, index, offset);
            seq = shifted;
        }
    }

    let index_zero = seq.iter()
        .position(|num| num.move_by == 0).unwrap();

    let t = seq[(index_zero + 1000) % len].move_by
        + seq[(index_zero + 2000) % len].move_by
        + seq[(index_zero + 3000) % len].move_by;


    let answer1 = t.to_string();
    return answer1.to_string();
}

fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));

    let lines: Vec<&str> = data2_s.trim().split(LINE_ENDING).collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }


    let decryption_key: i64 = DECRYPTION_KEY.1;
    let iterations: usize = ITERATIONS.1;
    let orig_numbers: Vec<i64> = lines.into_iter().map(|n| n.parse::<i64>().unwrap()).collect();

    let mut seq: Vec<Number> = orig_numbers.iter()
        .enumerate()
        .map(|(original_index, &move_by)|
            Number {
                original_index,
                move_by: move_by * decryption_key,
            })
        .collect();
    //  println!("seq: {:?}", seq);

    let len = seq.len();

    for _ in 0..iterations {
        for i in 0..len {
            let index = seq.iter().position(
                |num| num.original_index == i).unwrap();

            let offset = seq[index].move_by;
            let shifted = shift_element(&seq, index, offset);
            seq = shifted;
        }
    }

    let index_zero = seq.iter()
        .position(|num| num.move_by == 0).unwrap();

    let t = seq[(index_zero + 1000) % len].move_by
        + seq[(index_zero + 2000) % len].move_by
        + seq[(index_zero + 3000) % len].move_by;


    let answer2 = t.to_string();
    return answer2;
}
