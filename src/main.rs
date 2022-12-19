#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::collections::{HashMap, VecDeque};
use std::collections::HashSet;
use std::{cmp, fmt};
use std::fmt::{Display, Formatter};
use std::fs;
use std::time::{Instant,Duration};

use ndarray::{Array, Array2, Array3, ArrayBase, OwnedRepr, Dim};
use parse_display::FromStr;
use crate::Voxel::Outside;


/*
    Advent of Code 2022: Day 18
        part1 answer: 4282
        part2 answer: 2452

part2,  4036 is too high
 */

const TEST_ANSWER: (i64, i64) = (64, 58);
const INPUT_ANSWER: (i64, i64) = (4282, 2452);

const PART1_TEST_FILENAME: &str = "data/day18/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day18/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day18/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day18/part2_input.txt";

const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("18");                           // insert Day


    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();

    println!("\t Part 1: {answer1} ,\t time: {:?}", duration1);
    //
    if TEST {
        assert_eq!(answer1, TEST_ANSWER.0.to_string());
    } else {
        assert_eq!(answer1, INPUT_ANSWER.0.to_string());
    }

    let start2 = Instant::now();
    let answer2 = part2();
    let duration2 = start2.elapsed();

    println!("\t Part 2: {answer2} ,\t time: {:?}", duration2);

    // if TEST {
    //     assert_eq!(answer2, TEST_ANSWER.1.to_string());
    // } else {
    //     assert_eq!(answer2, INPUT_ANSWER.1.to_string());
    // }

    println!("----------\ndone");
}

#[cfg(windows)]
const D_LINE_ENDING: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const D_LINE_ENDING: &'static str = "\n\n";

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

const ARRAY_SIZE:usize = 25;

#[derive(FromStr, Debug, Copy, Clone,Eq, Hash)]
#[display("{x},{y},{z}")]
struct Point3D {
    x:usize,
    y:usize,
    z:usize
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
    }
}

impl Display for Point3D{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"<{},{},{}>", self.x,self.y,self.z)
    }
}
#[derive(Debug, Copy, Clone,PartialEq,Eq, Hash)]
enum Voxel  {
    Unknown=0,
    Outside,
    Lava
}

impl Display for Voxel{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Voxel::Lava => {write!(f,"L")}
            Voxel::Outside => {write!(f,"O")}
            Voxel::Unknown => {write!(f,"?")}
        }
    }
}

impl Default for Voxel {
    fn default() -> Voxel {
        return Voxel::Unknown
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
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }
    let data1_ss = data1_s.trim();
    let split_lines:Vec<&str> = data1_ss.split(LINE_ENDING).collect();

    let mut v_points:Vec<Point3D> = Vec::new();
    for i in 0..split_lines.len() {
        let pp = split_lines[i].parse::<Point3D>().unwrap();
        let s_pp = Point3D{x:pp.x+1, y:pp.y+1, z:pp.z+1};
        v_points.push(s_pp);
    }

    let v_points = v_points.clone();

    let v_points = v_points.clone();

    let mut p_cloud:HashSet<Point3D> = HashSet::new();
    for p in v_points.clone() {
        p_cloud.insert(p.clone());
    }
    let mut cube_count =0;

   for p in &v_points {
       let mut sides:i32=0;
       let mut h_sides:HashSet<Point3D> = HashSet::new();

       let mut pp = Point3D{x:p.x+1, y:p.y, z:p.z};
       h_sides.insert(pp.clone());
       let mut pp = Point3D{x:p.x-1  , y:p.y, z:p.z};
       h_sides.insert(pp.clone());
       let mut pp = Point3D{x:p.x, y:p.y+1, z:p.z};
       h_sides.insert(pp.clone());
       let mut pp = Point3D{x:p.x, y:p.y-1, z:p.z};
       h_sides.insert(pp.clone());
       let mut pp = Point3D{x:p.x, y:p.y, z:p.z+1};
       h_sides.insert(pp.clone());
       let mut pp = Point3D{x:p.x, y:p.y, z:p.z-1};
       h_sides.insert(pp.clone());

        for h in h_sides {
            if !p_cloud.contains(&h) {
                cube_count +=1;
            }
        }
   }


    let answer1 = cube_count.to_string();
    return answer1.to_string();
}

fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));

    let mut lines: Vec<&str> = data2_s.trim().split("\n").collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
        if l_num == 1 {
            println!("\t\t line read has length: {}", lines[0].len());
        }
    }
    let data2_ss = data2_s.trim();
    let split_lines:Vec<&str> = data2_ss.split(LINE_ENDING).collect();

    let mut v_points:Vec<Point3D> = Vec::new();
    for i in 0..split_lines.len() {
        let pp = split_lines[i].parse::<Point3D>().unwrap();
        let s_pp = Point3D{x:pp.x+2, y:pp.y+2, z:pp.z+2};
        v_points.push(s_pp);
    }

    let v_points = v_points.clone();


    let v_points = v_points.clone();

    let  (mut x_min,mut x_max) = (usize::MAX, usize::MIN);
    let  (mut y_min,mut y_max) = (usize::MAX, usize::MIN);
    let  (mut z_min,mut z_max) = (usize::MAX, usize::MIN);

    let mut p_cloud:HashSet<Point3D> = HashSet::new();
    for p in v_points.clone() {
        x_min = cmp::min(p.x, x_min);
        y_min = cmp::min(p.y, y_min);
        z_min = cmp::min(p.z, z_min);

        x_max = cmp::max(p.x, x_max);
        y_max = cmp::max(p.y, y_max);
        z_max = cmp::max(p.z, z_max);

        p_cloud.insert(p.clone());
    }


    let mut cube_count =0;

    for p in &v_points {
        let mut sides:usize=0;
        let mut h_sides:HashSet<Point3D> = HashSet::new();

        let mut pp = Point3D{x:p.x+1, y:p.y, z:p.z};
        h_sides.insert(pp.clone());
        let mut pp = Point3D{x:p.x-1, y:p.y, z:p.z};
        h_sides.insert(pp.clone());
        let mut pp = Point3D{x:p.x, y:p.y+1, z:p.z};
        h_sides.insert(pp.clone());
        let mut pp = Point3D{x:p.x, y:p.y-1, z:p.z};
        h_sides.insert(pp.clone());
        let mut pp = Point3D{x:p.x, y:p.y, z:p.z+1};
        h_sides.insert(pp.clone());
        let mut pp = Point3D{x:p.x, y:p.y, z:p.z-1};
        h_sides.insert(pp.clone());

        for h in h_sides {
            if !p_cloud.contains(&h) {
                cube_count +=1;
            }
        }
    }

    let mut hole_count:usize = 0;
    let mut considered_holes:usize = 0;
    // v_holes: count of single voxel holes
    let mut v_holes:HashSet<Point3D> = HashSet::new();
    for xx in (x_min-1)..=(x_max+1) {
        for yy in (y_min-1)..=(y_max+1) {
            for zz in (z_min-1)..=(z_max+1) {
                let p = Point3D{x:xx,y:yy,z:zz};
                if p_cloud.contains(&p) {
                    // point is lava, ignore
                    continue;
                }
                considered_holes +=1;
                let mut sides =0;
                // check point on each side, and count how many are lava

                let mut pp = Point3D{x:p.x+1, y:p.y, z:p.z};
                if p_cloud.contains(&pp) {
                    sides += 1;
                }
                let mut pp = Point3D{x:p.x-1, y:p.y, z:p.z};
                if p_cloud.contains(&pp) {
                    sides += 1;
                }
                let mut pp = Point3D{x:p.x, y:p.y+1, z:p.z};
                if p_cloud.contains(&pp) {
                    sides += 1;
                }
                let mut pp = Point3D{x:p.x, y:p.y-1, z:p.z};
                if p_cloud.contains(&pp) {
                    sides += 1;
                }
                let mut pp = Point3D{x:p.x, y:p.y, z:p.z+1};
                if p_cloud.contains(&pp) {
                    sides += 1;
                }
                let mut pp = Point3D{x:p.x, y:p.y, z:p.z-1};
                if p_cloud.contains(&pp) {
                    sides += 1;
                }
                if sides == 6 {
                    //println!("\tquery point {p} has lava on {} sides", sides);
                    hole_count +=1;
                    v_holes.insert(p.clone());
                }
            }
        }
    }

    let mut previous = v_holes.len();

    let mut longest_dir = cmp::max(x_max, y_max);
    longest_dir = cmp::max(longest_dir, z_max);
    longest_dir +=1;
    let a_size:usize = longest_dir as usize;

    let v = vec![Voxel::Unknown; a_size * a_size * a_size];


    let mut grid: [[[Voxel; ARRAY_SIZE]; ARRAY_SIZE]; ARRAY_SIZE] = Default::default();
let mut inital_markings = 0;
    for xx in 0..ARRAY_SIZE {
        for yy in 0..ARRAY_SIZE {
            for zz in 0..ARRAY_SIZE {
                grid[xx][yy][0] = Voxel::Outside; // bottom side
                grid[xx][yy][ARRAY_SIZE - 1] = Voxel::Outside;// top side

                grid[0][yy][zz] = Voxel::Outside;
                grid[ARRAY_SIZE - 1][yy][zz] = Voxel::Outside;

                grid[xx][0][zz] = Voxel::Outside;
                grid[xx][ARRAY_SIZE - 1][zz] = Voxel::Outside;
                inital_markings += 6;
            }
        }
    }

    for p in p_cloud {
        let (x,y,z) =(p.x, p.y, p.z);
        grid[x][y][z] = Voxel::Lava;
    }


    let mut flood_loops =-1;
    // array has outside marked, and lava quares mark. flood-fill outside
    let mut progress = true;
    while progress {
        progress = false;
        for xx in 0..ARRAY_SIZE {
            for yy in 0..ARRAY_SIZE {
                for zz in 0..ARRAY_SIZE {
                    if grid[xx][yy][zz] != Voxel::Unknown {
                        continue;
                    }


                    let (dx, dy, dz) = (xx + 1, yy, zz);
                    if grid[dx][dy][dz] == Voxel::Outside {
                        progress = true;
                        grid[xx][yy][zz] = Voxel::Outside;
                        continue;
                    }
                    let (dx, dy, dz) = ((xx as i64 + -1) as usize, yy, zz);
                    if grid[dx][dy][dz] == Voxel::Outside {
                        progress = true;
                        grid[xx][yy][zz] = Voxel::Outside;
                        continue;
                    }

                    let (dx, dy, dz) = (xx, yy + 1, zz);
                    if grid[dx][dy][dz] == Voxel::Outside {
                        progress = true;
                        grid[xx][yy][zz] = Voxel::Outside;
                        continue;
                    }

                    let (dx, dy, dz) = (xx, (yy as i64 + -1) as usize, zz);

                    if grid[dx][dy][dz] == Voxel::Outside {
                        progress = true;
                        grid[xx][yy][zz] = Voxel::Outside;
                        continue;
                    }
                    let (dx, dy, dz) = (xx, yy, zz + 1);
                    if grid[dx][dy][dz] == Voxel::Outside {
                        progress = true;
                        grid[xx][yy][zz] = Voxel::Outside;
                        continue;
                    }

                    let (dx, dy, dz) = (xx, yy, (zz as i64 + -1) as usize);
                    if grid[dx][dy][dz] == Voxel::Outside {
                        progress = true;
                        grid[xx][yy][zz] = Voxel::Outside;
                        continue;
                    }
                }
            }
        }
    }
    let mut out_count = 0;
    let mut unknown_count = 0;
    let mut lava_count = 0;
    for xx in 0..ARRAY_SIZE {
        for yy in 0..ARRAY_SIZE {
            for zz in 0..ARRAY_SIZE {
                let q = grid[xx][yy][zz];
                match q {
                    Voxel::Unknown => {unknown_count += 1 ; }
                    Outside => { out_count += 1;}
                    Voxel::Lava => {lava_count +=1;}
                }
            }
        }
    }

    let mut outside_lava_count = 0;
    for xx in 0..ARRAY_SIZE {
        for yy in 0..ARRAY_SIZE {
            for zz in 0..ARRAY_SIZE {
                if grid[xx][yy][zz] != Voxel::Lava {
                    continue;
                }


                let (dx, dy, dz) = (xx + 1, yy, zz);
                if grid[dx][dy][dz] == Voxel::Outside {
                    outside_lava_count += 1;
                }
                let (dx, dy, dz) = ((xx as i64 + -1) as usize, yy, zz);
                if grid[dx][dy][dz] == Voxel::Outside {
                    outside_lava_count += 1;
                }

                let (dx, dy, dz) = (xx, yy + 1, zz);
                if grid[dx][dy][dz] == Voxel::Outside {
                    outside_lava_count += 1;
                }

                let (dx, dy, dz) = (xx, (yy as i64 + -1) as usize, zz);

                if grid[dx][dy][dz] == Voxel::Outside {
                    outside_lava_count += 1;
                }
                let (dx, dy, dz) = (xx, yy, zz + 1);
                if grid[dx][dy][dz] == Voxel::Outside {
                    outside_lava_count += 1;
                }

                let (dx, dy, dz) = (xx, yy, (zz as i64 + -1) as usize);
                if grid[dx][dy][dz] == Voxel::Outside {
                    outside_lava_count += 1;
                }
            }
        }
    }


    let mut answer2 = outside_lava_count.to_string();
    return answer2;
}
