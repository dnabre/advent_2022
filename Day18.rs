use std::collections::HashSet;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs;
use std::time::Instant;

use parse_display::FromStr;

/*
    Advent of Code 2022: Day 18
        part1 answer: 4282
        part2 answer: 2452

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

const ARRAY_SIZE: usize = 25;

#[derive(FromStr, Debug, Copy, Clone, Eq, Hash)]
#[display("{x},{y},{z}")]
struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{},{},{}>", self.x, self.y, self.z)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Voxel {
    Unknown = 0,
    Outside,
    Lava,
}

impl Display for Voxel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Voxel::Lava => { write!(f, "L") }
            Voxel::Outside => { write!(f, "O") }
            Voxel::Unknown => { write!(f, "?") }
        }
    }
}

impl Default for Voxel {
    fn default() -> Voxel {
        return Voxel::Unknown;
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
    let data1_ss = data1_s.trim();
    let split_lines: Vec<&str> = data1_ss.split(LINE_ENDING).collect();

    let mut v_points: Vec<Point3D> = Vec::new();
    for i in 0..split_lines.len() {
        let pp = split_lines[i].parse::<Point3D>().unwrap();
        let s_pp = Point3D { x: pp.x + 1, y: pp.y + 1, z: pp.z + 1 };
        v_points.push(s_pp);
    }

    let v_points = v_points.clone();

    let v_points = v_points.clone();

    let mut p_cloud: HashSet<Point3D> = HashSet::new();
    for p in v_points.clone() {
        p_cloud.insert(p.clone());
    }
    let mut cube_count = 0;

    for p in &v_points {
        let mut h_sides: HashSet<Point3D> = HashSet::new();

        let pp = Point3D { x: p.x + 1, y: p.y, z: p.z };
        h_sides.insert(pp.clone());
        let pp = Point3D { x: p.x - 1, y: p.y, z: p.z };
        h_sides.insert(pp.clone());
        let pp = Point3D { x: p.x, y: p.y + 1, z: p.z };
        h_sides.insert(pp.clone());
        let pp = Point3D { x: p.x, y: p.y - 1, z: p.z };
        h_sides.insert(pp.clone());
        let pp = Point3D { x: p.x, y: p.y, z: p.z + 1 };
        h_sides.insert(pp.clone());
        let pp = Point3D { x: p.x, y: p.y, z: p.z - 1 };
        h_sides.insert(pp.clone());

        for h in h_sides {
            if !p_cloud.contains(&h) {
                cube_count += 1;
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

    let mut v_points: Vec<Point3D> = Vec::new();
    for i in 0..split_lines.len() {
        let pp = split_lines[i].parse::<Point3D>().unwrap();
        let s_pp = Point3D { x: pp.x + 2, y: pp.y + 2, z: pp.z + 2 };
        v_points.push(s_pp);
    }

    let v_points = v_points.clone();
    let v_points = v_points.clone();

    let mut p_cloud: HashSet<Point3D> = HashSet::new();
    for p in v_points.clone() {
        p_cloud.insert(p.clone());
    }

    let mut grid: [[[Voxel; ARRAY_SIZE]; ARRAY_SIZE]; ARRAY_SIZE] = Default::default();

    for xx in 0..ARRAY_SIZE {
        for yy in 0..ARRAY_SIZE {
            for zz in 0..ARRAY_SIZE {
                grid[xx][yy][0] = Voxel::Outside; // bottom side
                grid[xx][yy][ARRAY_SIZE - 1] = Voxel::Outside;// top side

                grid[0][yy][zz] = Voxel::Outside;
                grid[ARRAY_SIZE - 1][yy][zz] = Voxel::Outside;

                grid[xx][0][zz] = Voxel::Outside;
                grid[xx][ARRAY_SIZE - 1][zz] = Voxel::Outside;
            }
        }
    }

    for p in p_cloud {
        let (x, y, z) = (p.x, p.y, p.z);
        grid[x][y][z] = Voxel::Lava;
    }


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
    let answer2 = outside_lava_count.to_string();
    return answer2;
}
