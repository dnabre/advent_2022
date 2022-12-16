use std::{cmp, fmt};
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use parse_display::{Display, FromStr};

/*
    Advent of Code 2022: Day 15
        part1 answer: 5394423
        part2 answer: 11840879211051


 */

const TEST_ANSWER: (i64, i64) = (26, 56000011);
const INPUT_ANSWER: (i64, i64) = (5394423, 11840879211051);

const PART1_TEST_FILENAME: &str = "data/day15/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day15/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day15/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day15/part2_input.txt";

const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("15");

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


const PART1_TARGET_ROW: (i64, i64) = (10, 2_000_000);
const PART2_TUNING_MULTIPLER: i64 = 4000000;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Beacon {
    loc: IntegerPoint,
}





#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
struct IntegerPoint {
    x: i64,
    y: i64,
}

impl fmt::Display for IntegerPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{},{}>", self.x, self.y)
    }
}




#[derive(Clone, Copy, Debug)]
struct LineSegment {
    slope: f64,
    constant: f64,
    x_min: f64,
    x_max: f64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
    fn contains(&self, value: i64) -> bool {
        return value >= self.start && value <= self.end;
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Sensor {
    loc: IntegerPoint,
    closet_beacon: Beacon,
    m_range: i64,
    id: i64,
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

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Sensor at x={sensor_x}, y={sensor_y}: closest beacon is at x={c_beacon_x}, y={c_beacon_y}")]
struct SensorInput {
    sensor_x: i64,
    sensor_y: i64,
    c_beacon_x: i64,
    c_beacon_y: i64,
}

#[derive(Clone, Copy, Debug)]
struct Vertex {
    x: f64,
    y: f64,
}

fn beacons_in_range(sensors: &[Sensor], range: &Range, y: i64, into_vec: &mut Vec<(i64, i64)>) {
    into_vec.clear();
    for s in sensors {
        let closet_beacon = (s.closet_beacon.loc.x, s.closet_beacon.loc.y);
        if closet_beacon.1 == y && range.contains(closet_beacon.0) && !into_vec.contains(&closet_beacon) {
            into_vec.push(closet_beacon);
        }
    }
}

fn distance_to_beacon(s: IntegerPoint, b: Beacon) -> i64 {
    return man_distance(s, b.loc);
}

fn get_ranges_for_row(sensor: &Sensor, row: i64) -> Option<Range> {
    let half_size = sensor.m_range - (row - sensor.loc.y).abs();
    let range_size = half_size * 2 + 1;
    if range_size > 0 {
        return Some(Range::new(
            sensor.loc.x - half_size,
            sensor.loc.x + half_size,
        ));
    } else {
        return None;
    }
}

fn get_testgroup_for_sensor(s: Sensor) -> Vec<LineSegment> {
    let s_point = s.loc;
    let (x, y) = (s_point.x, s_point.y);
    let r = s.m_range;
    let (a1, a2) = (IntegerPoint { x: x - r, y: y - 1 }, IntegerPoint { x: x, y: y - r - 1 });
    let a = line_segment_from_ipoints(a1, a2);
    let (b1, b2) = (IntegerPoint { x: x + 1, y: y - r }, IntegerPoint { x: x + r + 1, y: y });
    let b = line_segment_from_ipoints(b1, b2);
    let (c1, c2) = (IntegerPoint { x: x + r, y: y + 1 }, IntegerPoint { x: x, y: y + r + 1 });
    let c = line_segment_from_ipoints(c1, c2);
    let (d1, d2) = (IntegerPoint { x: x - 1, y: y + r }, IntegerPoint { x: x - r, y: y + 1 });
    let d = line_segment_from_ipoints(d1, d2);
    return vec![a, b, c, d];
}


fn intersect(l: LineSegment, r: LineSegment) -> Option<Vertex> {
    let mut left = l.clone();
    let mut right = r.clone();
    left.slope -= right.slope;
    right.slope = 0.0;

    right.constant -= left.constant;
    left.constant = 0.0;

    let x = right.constant / left.slope;
    if x < left.x_min || x > left.x_max || x < right.x_min || x > right.x_max {
        return None;
    }
    let y = l.slope * x + l.constant;

    return Some(Vertex { x, y });
}

fn line_segment_from_ipoints(a: IntegerPoint, b: IntegerPoint) -> LineSegment {
    let slope_: f64 = (b.y as f64 - a.y as f64) / (b.x as f64 - a.x as f64);
    let c_t: f64 = a.y as f64 - slope_ * a.x as f64;
    let x_min_n = cmp::min(a.x, b.x) as f64;
    let x_max_x = cmp::max(a.x, b.x) as f64;

    return LineSegment { slope: slope_, constant: c_t, x_min: x_min_n, x_max: x_max_x };
}

fn man_distance(p1: IntegerPoint, p2: IntegerPoint) -> i64 {
    let x_d = p1.x.abs_diff(p2.x);
    let y_d = p1.y.abs_diff(p2.y);
    return (x_d + y_d) as i64;
}

fn merge_ranges(sorted_ranges: &Vec<Range>) -> Vec<Range> {
    let mut result: Vec<Range> = Vec::new();
    let mut index = 0;
    let mut current = sorted_ranges.get(index).copied();
    loop {
        let next = sorted_ranges.get(index);
        index += 1;
        match (current, next) {
            (Some(r1), None) => {
                result.push(r1);
                return result;
            }
            (Some(r1), Some(r2)) if r1.contains(r2.start) => {
                current = Some(Range::new(r1.start, r1.end.max(r2.end)))
            }
            (Some(r1), Some(&r2)) => {
                if r1.end + 1 == r2.start {
                    current = Some(Range::new(r1.start, r1.end.max(r2.end)))
                } else {
                    current = Some(r2);
                    result.push(r1);
                }
            }
            (None, _) => return result,
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
    let lines: Vec<&str> = data1_s.trim().split("\n").map(|t| t.trim()).collect();
    let l_num = lines.len();
    let query_row = match TEST {
        true => { PART1_TARGET_ROW.0 }
        false => { PART1_TARGET_ROW.1 }
    };
    if TEST
    {
        println!("\t read {} lines from {}", l_num, p1_file);
    }

    let mut v_sensors: Vec<Sensor> = Vec::new();
    let mut v_beacons: Vec<Beacon> = Vec::new();

    let (mut min_x, mut min_y): (i64, i64) = (i64::MAX, i64::MAX);
    let (mut max_x, mut max_y): (i64, i64) = (i64::MIN, i64::MIN);


    for i in 0..lines.len() {
        let ln = lines[i];
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
            id: i as i64,
        };

        let right = IntegerPoint { x: s.loc.x + s.m_range, y: s.loc.y };
        let left = IntegerPoint { x: s.loc.x - s.m_range, y: s.loc.y };
        let up = IntegerPoint { x: s.loc.x, y: s.loc.y + s.m_range };
        let down = IntegerPoint { x: s.loc.x, y: s.loc.y - s.m_range };

        for p in [right, left, up, down] {
            (min_x, min_y) = (cmp::min(min_x, p.x), cmp::min(min_y, p.y));
            (max_x, max_y) = (cmp::max(max_x, p.x), cmp::max(max_y, p.y));
        }

        v_beacons.push(b);
        v_sensors.push(s);
    }
    let v_sensors = v_sensors.clone();
    let mut v_range: Vec<Range> = Vec::new();

    for s in &v_sensors {
        match get_ranges_for_row(s, query_row) {
            None => {}
            Some(r) => {
                v_range.push(r);
            }
        }
    }

    v_range.sort_unstable_by_key(|r| r.start);
    let merged: Vec<Range> = merge_ranges(&v_range);
    let mut included_beacons = Vec::new();
    let answer1 = merged
        .iter()
        .map(|range| {
            beacons_in_range(&v_sensors, range, query_row, &mut included_beacons);
            (range.end - range.start + 1) as usize - included_beacons.len()
        })
        .sum::<usize>() as u64;

    let answer1 = answer1.to_string();
    return answer1;
}

fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));
    let lines: Vec<&str> = data2_s.trim().split("\n").map(|t| t.trim()).collect();
    let l_num = lines.len();

    if TEST
    {
        println!("\t read {} lines from {}", l_num, p2_file);
    }

    let mut v_sensors: Vec<Sensor> = Vec::new();
    let mut v_beacons: Vec<Beacon> = Vec::new();
    let (mut min_x, mut min_y): (i64, i64) = (i64::MAX, i64::MAX);
    let (mut max_x, mut max_y): (i64, i64) = (i64::MIN, i64::MIN);

    for i in 0..lines.len() {
        let ln = lines[i];
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
            id: i as i64,
        };
        v_beacons.push(b);
        v_sensors.push(s);
    }
    let v_sensors = v_sensors.clone();
    let mut a_vertex: Vec<Vertex> = Vec::new();
    let mut test_points: HashSet<IntegerPoint> = HashSet::new();
    for s1 in &v_sensors {
        for s2 in &v_sensors {
            if s1 == s2 {
                continue;
            }
            let l_segments: Vec<LineSegment> = get_testgroup_for_sensor(s1.clone());
            let r_segments: Vec<LineSegment> = get_testgroup_for_sensor(s2.clone());
            for l in &l_segments {
                for r in &r_segments {
                    let i = intersect(*l, *r);
                    match i {
                        None => {}
                        Some(v) => {
                            //      println!("{:?}", v);
                            if v.x.is_finite() && v.y.is_finite() {
                                a_vertex.push(v)
                            };
                        }
                    }
                }
            }
        }
    }
    for v in a_vertex {
        let v_x = v.x as i64;
        let v_y = v.y as i64;
        for xx in (v_x - 2)..=(v_x + 2) {
            for yy in (v_y - 2)..=(v_y + 2) {
                let ip = IntegerPoint { x: xx, y: yy };
                test_points.insert(ip.clone());
            }
        }
    }
    let mut answer2: i64 = -1;
    let range_max = match TEST {
        true => { 20 }
        false => { 4000000 }
    };
    for p in test_points {
        if (p.x < 0) || (p.y < 0) {
            continue;
        }
        if (p.x > range_max) || (p.y > range_max) {
            continue;
        }
        let mut unseen = true;
        for s in &v_sensors {
            let m = man_distance(s.loc, p);
            if m <= s.m_range {
                unseen = false;
            }
        }
        if unseen {
            let tf1: i64 = (p.x as i64) * PART2_TUNING_MULTIPLER;
            let tf2: i64 = tf1 + (p.y as i64);
            answer2 = tf2;
        }
    }
    let answer2 = answer2.to_string();
    return answer2;
}
