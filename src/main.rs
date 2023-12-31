#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]
#[allow(unused_variables)]

/*
    Advent of Code 2023: Day 16
        part1 answer:
        part2 answer:

 */


use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::time::Instant;

const ANSWER: (&str, &str) = ("2077", "2741");

fn main() {
    let _filename_test1 = "data/day16/test_input_01.txt";
    let _filename_test2 = "data/day16/test_input_02.txt";

    let filename_part1 = "data/day16/part1_input.txt";
    let filename_part2 = "data/day16/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(_filename_test1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(_filename_test2);
    let duration2 = start2.elapsed();

    println!("Advent of Code, Day 16");
    println!("    ---------------------------------------------");

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }
    //
    // println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    // if ANSWER.1 != answer2 {
    //     println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    // }
    println!("    ---------------------------------------------");
}


fn part1(input_file: &str) -> String {
    let lines = advent_2022::file_to_lines(input_file);
    let (node_list, valve_name_list, id_lookup_by_name, edge_lists) =
    parse_valves(&lines);

    for n in node_list.iter() {
        println!("{}", n );
    }



    let answer = 0;
    return answer.to_string();
}

fn parse_valves(lines:& Vec<String>) -> (Vec<Node>, Vec<&str>, HashMap<&str, usize>, Vec<Vec<usize>>) {
    let mut valve_set: HashSet<&str> = HashSet::new();
    let mut flow_rate_map: HashMap<&str, usize> = HashMap::new();
    let mut edges_by_name: HashMap<&str, Vec<&str>> = HashMap::new();

    for  l in lines.iter()
    {
        let (left, right) = l.split_once(";").unwrap();

        let first_valve = &left[6..=7];
        valve_set.insert(first_valve);

        let  n_flow: usize;


        let  edges:Vec<&str>;
        if right.contains("valves") {
            n_flow = left[23..].parse().unwrap();
            edges = right[24..].split(",").map(|s| s.trim()).collect();
        } else {
            let m_n_flow: Result<usize, _> = left[23..].parse();
            match m_n_flow {
                Ok(n) => { n_flow = n; }
                Err(_) => { panic!("error parsing for number: {:?}", m_n_flow); }
            }
            edges = right[23..].split(",").map(|s| s.trim()).collect();
        }
        edges_by_name.insert(first_valve, edges.clone());
        flow_rate_map.insert(first_valve, n_flow);
        for e in &edges {
            valve_set.insert(e);
        }
    }


    let mut valve_name_list = Vec::from_iter(valve_set);
    valve_name_list.sort();
    let valve_name_list = valve_name_list;

    let mut flow_rate: Vec<usize> = Vec::new();
    let mut id_lookup_by_name: HashMap<&str, usize> = HashMap::new();
    for i in 0..valve_name_list.len() {
        let u = i as usize;
        let n = valve_name_list[u];
        id_lookup_by_name.insert(n, u);
        let f = flow_rate_map[n];
        flow_rate.push(f);
    }


    let mut edge_lists: Vec<Vec<usize>> = Vec::new();

    for id in 0..valve_name_list.len() {
        let name = valve_name_list[id];
        let s_edges = &edges_by_name[name];
        let mut i_edges: Vec<usize> = Vec::new();
        for s_e in s_edges.iter() {
            let e_id = id_lookup_by_name[s_e];
            i_edges.push(e_id);
        }
        edge_lists.push(i_edges);
    }
    let edge_lists = edge_lists;

    let number_of_valves = lines.len();
    let mut node_list:Vec<Node> = Vec::new();
    for u in 0..number_of_valves {
        let v_name = valve_name_list[u];
        let l_edges = &edge_lists[u];
        let u_flow = flow_rate[u];
        let node = Node {
            id: u,
            name: v_name.to_string(),
            flow: u_flow,
            edges: l_edges.clone(),
        };
        node_list.push(node);
    }
    return (node_list, valve_name_list, id_lookup_by_name, edge_lists);
}

#[derive(Debug, Hash,  Clone)]
struct Node {
    id:usize,
    name: String,
    flow: usize,
    edges:Vec<usize>
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Valve[{}] {} has flow: {}, with edges {}", self.id, self.name, self.flow,
               advent_2022::list_displayables_to_string(&self.edges))
    }
}

fn part2(input_file: &str) -> String {
    let lines = advent_2022::file_to_lines(input_file);

    let answer = 0;
    return answer.to_string();
}

