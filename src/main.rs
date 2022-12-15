#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::{cmp, fmt};
use std::fmt::Formatter;
use std::fs;
use std::io::ErrorKind::Interrupted;
use std::time::Instant;
use geo::point;
use parse_display::{Display, FromStr};

use rstar::Point;

/*
    Advent of Code 2022: Day 15
        part1 answer: 5394423
        part2 answer:


 */


// test (-8,-10) to (28,26), input: (-911_068, -910_981) to (5_534_348,5_396_743)

const TEST_ANSWER: (i32, i32) = (26, 5394423);
const INPUT_ANSWER: (i32, i32) = (56000011, 0);


const PART1_TEST_FILENAME: &str = "data/day15/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day15/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day15/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day15/part2_input.txt";

const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("15");

    // let start1 = Instant::now();
    // let answer1 = part1();
    // let duration1 = start1.elapsed();
    //
    // println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);

    // if TEST {
    //     assert_eq!(answer1, TEST_ANSWER.0.to_string());
    // } else {
    //     assert_eq!(answer1, INPUT_ANSWER.0.to_string());
    // }

    let start2 = Instant::now();
    let answer2 = part2();
    let duration2 = start2.elapsed();

    println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);
    //
    // if TEST {
    //     assert_eq!(answer2, TEST_ANSWER.1.to_string());
    // } else {
    //     assert_eq!(answer2, INPUT_ANSWER.1.to_string());
    // }
    //
    // println!("----------\ndone");
}


const PART1_TARGET_ROW: (i32, i32) = (10, 2_000_000);
const PART2_TUNING_MULTIPLER: i32 = 4000000;
const PART2_TEST_RANGES: (i32, i32) = (20, 20);
const PART2_INPUT_RANGES: (i32, i32) = (4000000, 4000000);

#[derive(Copy, Clone, PartialEq, Debug)]
struct Sensor {
    loc: IntegerPoint,
    closet_beacon: Beacon,
    m_range: i32,
    id: i32,
}


impl Default for Sensor {
    fn default() -> Sensor {
        Sensor {
            loc: IntegerPoint { x: 0, y: 0 },
            closet_beacon: Beacon { loc: IntegerPoint { x: 0, y: 0 } },
            m_range: 0,
            id: 0,
        }
    }
}


#[derive(Copy, Clone, PartialEq, Debug)]
struct Beacon {
    loc: IntegerPoint,
}


#[derive(Copy, Clone, PartialEq, Debug)]
struct IntegerPoint {
    x: i32,
    y: i32,
}

impl fmt::Display for IntegerPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{},{}>", self.x, self.y)
    }
}


impl Point for IntegerPoint
{
    type Scalar = i32;
    const DIMENSIONS: usize = 2;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self
    {
        IntegerPoint {
            x: generator(0),
            y: generator(1),
        }
    }

    fn nth(&self, index: usize) -> Self::Scalar
    {
        match index {
            0 => self.x,
            1 => self.y,
            _ => unreachable!()
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar
    {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => unreachable!()
        }
    }
}

//Input Line :Sensor at x=2, y=18: closest beacon is at x=-2, y=15

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Sensor at x={sensor_x}, y={sensor_y}: closest beacon is at x={c_beacon_x}, y={c_beacon_y}")]
struct SensorInput {
    sensor_x: i32,
    sensor_y: i32,
    c_beacon_x: i32,
    c_beacon_y: i32,
}

fn distance_to_beacon(s: IntegerPoint, b: Beacon) -> i32 {
    return man_distance(s, b.loc);
}

fn man_distance(p1: IntegerPoint, p2: IntegerPoint) -> i32 {
    let x_d = p1.x.abs_diff(p2.x);
    let y_d = p1.y.abs_diff(p2.y);
    return (x_d + y_d) as i32;
}

fn print_char_vec(v: &Vec<char>) {
    print!("[");
    for ch in v {
        print!("{}", ch);
    }
    println!("]");
}


fn render_for_space(v_sensors: &Vec<Sensor>, v_beacons: &Vec<Beacon>, min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> i32 {
    println!("rendering from ({},{}) -> ({},{})", min_x, min_y, max_x, max_y);
    let mut beacon_count = 0;
    let mut drew_something = false;
    let mut s_count = 0;
    let mut b_count = 0;
    let mut row_count = 0;
    let mut row_vector: Vec<IntegerPoint> = Vec::new();
    let target_row = match TEST {
        true => { PART1_TARGET_ROW.0 }
        false => { PART1_TARGET_ROW.1 }
    };
    for j in min_y..=max_y {
//        print!("{j:>3}");
        for i in min_x..=max_x {
            let p = IntegerPoint { x: i, y: j };
            let mut ch = '.';
            let mut row_trigger = false;
            for s in v_sensors {
                let b = s.closet_beacon;
                if s.loc == p {
                    ch = 'S';
                    s_count += 1;

                    //     println!("S @ {p}, {:?}", s);
                } else if b.loc == p {
                    ch = 'B';
                    b_count += 1;

                    //   println!("B @ {p}, {:?}", s);
                } else if man_distance(s.loc, p) <= s.m_range {
                    ch = '#';

                    if p.y == target_row {
                        row_trigger = true;
                    }
                }
            }

            if row_trigger {
                row_count += 1;
                row_trigger = false;
            }

            //         print!("{ch}");
        }
        // println!();
    }

    println!("\n s_count: {s_count} b_count: {b_count}");
    println!("sensors: {} beacons: {}", v_sensors.len(), v_beacons.len());

    println!("row_count: {}", row_count);


    return row_count;
}


fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };

    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let mut lines: Vec<&str> = data1_s.trim().split("\n").map(|t| t.trim()).collect();
    let l_num = lines.len();
    let query_row = match TEST {
        true => { PART1_TARGET_ROW.0 }
        false => { PART1_TARGET_ROW.1 }
    };
    // if TEST
    {
        println!("\t read {} lines from {}", l_num, p1_file);
    }

    let mut v_sensors: Vec<Sensor> = Vec::new();
    let mut v_beacons: Vec<Beacon> = Vec::new();
    let mut sensor_count = 0;
    let (mut min_x, mut min_y): (i32, i32) = (i32::MAX, i32::MAX);
    let (mut max_x, mut max_y): (i32, i32) = (i32::MIN, i32::MIN);

    let mut sensor_input_vec: Vec<SensorInput> = Vec::new();
    for i in 0..lines.len() {
        let ln = lines[i];
        //  println!("line {i:>2}:\t {}", ln);
        let s_input: SensorInput = ln.parse().unwrap();
        (min_x, min_y) = (cmp::min(min_x, s_input.c_beacon_x), cmp::min(min_y, s_input.c_beacon_y));
        (max_x, max_y) = (cmp::max(max_x, s_input.c_beacon_x), cmp::max(max_y, s_input.c_beacon_y));
        (min_x, min_y) = (cmp::min(min_x, s_input.sensor_x), cmp::min(min_y, s_input.sensor_y));
        (max_x, max_y) = (cmp::max(max_x, s_input.sensor_x), cmp::max(max_y, s_input.sensor_y));
        let b: Beacon = Beacon { loc: IntegerPoint { x: s_input.c_beacon_x, y: s_input.c_beacon_y } };
        let s: Sensor = Sensor {
            loc: IntegerPoint { x: s_input.sensor_x, y: s_input.sensor_y }
            ,
            closet_beacon: b,
            m_range: distance_to_beacon(IntegerPoint { x: s_input.sensor_x, y: s_input.sensor_y }, b),
            id: i as i32,
        };

        // sensor ranges
        let right = IntegerPoint { x: s.loc.x + s.m_range, y: s.loc.y };
        let left = IntegerPoint { x: s.loc.x - s.m_range, y: s.loc.y };
        let up = IntegerPoint { x: s.loc.x, y: s.loc.y + s.m_range };
        let down = IntegerPoint { x: s.loc.x, y: s.loc.y - s.m_range };

        for p in [right, left, up, down] {
            (min_x, min_y) = (cmp::min(min_x, p.x), cmp::min(min_y, p.y));
            (max_x, max_y) = (cmp::max(max_x, p.x), cmp::max(max_y, p.y));
        }


        // println!("space: <{min_x},{min_y}> to <{max_x}{max_y}>");

        v_beacons.push(b);
        v_sensors.push(s);
        sensor_count += 1;
    }
    let v_sensors = v_sensors.clone();
    let v_beacons = v_beacons.clone();
    println!("locations range from ({min_x},{min_y}) to ({max_x},{max_y})");

    let mut row_vec: Vec<char> = vec!['.'; (max_x - min_x) as usize];

    println!("target row (y={}): length = {}, x_delta = {}", query_row, row_vec.len(), min_x);
    //  println!("   {:?}", row_vec);
    //  print_char_vec(&row_vec);


    //   print_char_vec(&row_vec);
    println!();
    //full render
    // let ans = render_for_space(&v_sensors, &v_beacons, min_x, max_x, min_y, max_y);

    // just target
    let ans = render_for_space(&v_sensors, &v_beacons, min_x, max_x, query_row, query_row);


    println!("ans: {ans}");
    let mut answer1 = String::new();
    return answer1;
}


fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));

    let mut lines: Vec<&str> = data2_s.trim().split("\n").map(|t| t.trim()).collect();
    let l_num = lines.len();

    // if TEST
    {
        println!("\t read {} lines from {}", l_num, p2_file);
    }

    let mut v_sensors: Vec<Sensor> = Vec::new();
    let mut v_beacons: Vec<Beacon> = Vec::new();
    let mut sensor_count = 0;
    let (mut min_x, mut min_y): (i32, i32) = (i32::MAX, i32::MAX);
    let (mut max_x, mut max_y): (i32, i32) = (i32::MIN, i32::MIN);

    let mut sensor_input_vec: Vec<SensorInput> = Vec::new();
    for i in 0..lines.len() {
        let ln = lines[i];
        //  println!("line {i:>2}:\t {}", ln);
        let s_input: SensorInput = ln.parse().unwrap();
        (min_x, min_y) = (cmp::min(min_x, s_input.c_beacon_x), cmp::min(min_y, s_input.c_beacon_y));
        (max_x, max_y) = (cmp::max(max_x, s_input.c_beacon_x), cmp::max(max_y, s_input.c_beacon_y));
        (min_x, min_y) = (cmp::min(min_x, s_input.sensor_x), cmp::min(min_y, s_input.sensor_y));
        (max_x, max_y) = (cmp::max(max_x, s_input.sensor_x), cmp::max(max_y, s_input.sensor_y));
        let b: Beacon = Beacon { loc: IntegerPoint { x: s_input.c_beacon_x, y: s_input.c_beacon_y } };
        let s: Sensor = Sensor {
            loc: IntegerPoint { x: s_input.sensor_x, y: s_input.sensor_y }
            ,
            closet_beacon: b,
            m_range: distance_to_beacon(IntegerPoint { x: s_input.sensor_x, y: s_input.sensor_y }, b),
            id: i as i32,
        };

        // sensor ranges
        let right = IntegerPoint { x: s.loc.x + s.m_range, y: s.loc.y };
        let left = IntegerPoint { x: s.loc.x - s.m_range, y: s.loc.y };
        let up = IntegerPoint { x: s.loc.x, y: s.loc.y + s.m_range };
        let down = IntegerPoint { x: s.loc.x, y: s.loc.y - s.m_range };

        for p in [right, left, up, down] {
            (min_x, min_y) = (cmp::min(min_x, p.x), cmp::min(min_y, p.y));
            (max_x, max_y) = (cmp::max(max_x, p.x), cmp::max(max_y, p.y));
        }


        // println!("space: <{min_x},{min_y}> to <{max_x}{max_y}>");

        v_beacons.push(b);
        v_sensors.push(s);
        sensor_count += 1;
    }
    let v_sensors = v_sensors.clone();
    let v_beacons = v_beacons.clone();

    let range = match TEST {
        true => { PART2_TEST_RANGES }
        false => { PART2_INPUT_RANGES }
    };


    let (r_x, r_y) = range;
    let mut s_counter = 0;

    let mut p_counter = 0;
    let mut point_unseen = true;

    let mut v_points: Vec<IntegerPoint> = Vec::new();


    println!("Looking from ({},{}) to ({},{})", 0, 0, r_x, r_y);
    let mut loop_counter:i64 = 0;

    let mut seen = false;
    let mut ans = -1;

    'x_loop: for x in 0..=range.0 {
        for y in 0..range.1 {
            loop_counter +=1;
            if (loop_counter % 500_000) == 0 {
                println!("loop: {loop_counter} at <x,y> = {x}{y}");
            }
            let p = IntegerPoint { x: x, y: y };

            seen = false;
            for s in &v_sensors {
                let md = man_distance(s.loc, p);
                if md <= s.m_range {
                    //point seen
                    //     println!("point ({x}, {y} seen by sensor: {:>2}", s.id);
                    seen = true;
                    break;
                }
            }
            if !seen {
                println!("nobody saw <{x},{y}>");
                ans = x * PART2_TUNING_MULTIPLER + y;
                break 'x_loop
            }
        }
    }



    let mut answer2 = ans.to_string();
    return answer2;
}
