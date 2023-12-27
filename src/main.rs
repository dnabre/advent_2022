use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;
use std::ops::Range;
use std::time::{Instant, SystemTime};

/*
    Advent of Code 2023: Day 25
        part1 answer:   514786
        part2 answer:

*/
const ANSWER: (&str, &str) = ("514786", "Button Pressed");

fn main() {
    let _filename_test = "data/day25/test_input_01.txt";
    let _filename_test2 = "data/day25/test_input_02.txt";

    let filename_part1 = "data/day25/part1_input.txt";

    println!("Advent of Code, Day 25");
    println!("    ---------------------------------------------");
    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!(
            "\t\t ERROR: Answer is WRONG. Got: {}, Expected {}",
            answer1, ANSWER.0
        );
    }

    let start2 = Instant::now();
    let answer2 = part2();
    let duration2 = start2.elapsed();

    println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    if ANSWER.1 != answer2 {
        println!(
            "\t\t ERROR: Answer is WRONG. Got: {}, Expected {}",
            answer2, ANSWER.1
        );
    }
    println!("    ---------------------------------------------");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    id: usize,
    adjacent: BTreeSet<usize>,
}

type Graph = (V, E);
type V = HashSet<String>;
type E = Vec<(String, String)>;

fn get_random(seed: &mut u128, range: Range<usize>) -> usize {
    const M: u128 = 9223372036854775808u128;
    const A: u128 = 1103515245u128;
    const C: u128 = 12345u128;

    *seed = (A * *seed + C) % M;
    let b = range.end - range.start;
    let offset = (*seed % b as u128) as usize;
    return range.start + offset;
}

fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let (vertices, edges) = parse_input(&lines);

    let mut prng_seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    loop {
        let mut vertices = vertices.clone();
        let mut edges = edges.clone();
        while vertices.len() > 2 {
            let i = get_random(&mut prng_seed, 0..edges.len());
            let (v1, v2) = edges[i].clone();

            edges.swap_remove(i);
            vertices.remove(&v1);
            vertices.remove(&v2);

            let new_v = format!("{}:{}", v1, v2);
            vertices.insert(new_v.clone());
            for (e1, e2) in edges.iter_mut() {
                if *e1 == v1 || *e1 == v2 {
                    *e1 = new_v.clone()
                }
                if *e2 == v1 || *e2 == v2 {
                    *e2 = new_v.clone()
                }
            }

            let mut j = 0;
            while j < edges.len() {
                let (e1, e2) = &edges[j];
                if e1 == e2 {
                    edges.swap_remove(j);
                } else {
                    j += 1;
                }
            }
        }
        if edges.len() == 3 {
            return vertices
                .iter()
                .map(|s| s.split(':').count())
                .product::<usize>()
                .to_string();
        }
    }
}

fn parse_input(lines: &Vec<String>) -> Graph {
    let n = lines.len();
    let mut vertices: HashSet<String> = HashSet::with_capacity(n);
    let mut edges: Vec<(String, String)> = Vec::with_capacity(n * n);

    for i in 0..n {
        let line = &lines[i];
        let (left, right_side) = line.split_once(":").unwrap();
        vertices.insert(left.to_string());
        for right in right_side.split_whitespace() {
            vertices.insert(right.to_string());
            edges.push((left.to_string(), right.to_string()));
        }
    }
    return (vertices, edges);
}

fn part2() -> String {
    let answer = "Button Pressed";
    return answer.to_string();
}
