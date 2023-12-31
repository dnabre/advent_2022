#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]
#[allow(unused_variables)]

/*
    Advent of Code 2023: Day 16
        part1 answer:   2077
        part2 answer:

 */


use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::iter::zip;
use std::time::Instant;

const ANSWER: (&str, &str) = ("2077", "2741");

fn main() {
    let _filename_test1 = "data/day16/test_input_01.txt";
    let _filename_test2 = "data/day16/test_input_02.txt";

    let filename_part1 = "data/day16/part1_input.txt";
    let filename_part2 = "data/day16/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
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

type Id = usize;

fn solve(map: &HashMap<usize, Node>,  rounds: usize) -> usize {
    let start = State {
        score: 0,
        position: START,
        opened_valves: [0;50],
        valve_offset: 0,
    };

    let mut cost_map: HashMap<State, usize> = HashMap::new();
    let mut list: Vec<(State, usize)> = Vec::new();

    list.push((start, 0));

    while let Some((state, cost)) = list.pop() {
        if let Some(valve) = &map.get(&state.position) {
            for i in 0..valve.edges.len() {
                let connection = valve.edges[i];

                if state.opened_valves.contains(&connection.id) {
                    continue;
                }

                let next_cost = cost + connection.cost;

                if let Some(remaining) = rounds.checked_sub(next_cost) {
                    if let Some(next_value) = &map.get(&connection.id) {
                        let mut opened_valves = state.opened_valves.clone();
                        opened_valves[state.valve_offset] = connection.id;


                        let next_state = State {
                            score: state.score + next_value.flow * remaining,
                            position: connection.id,
                            opened_valves: opened_valves,
                            valve_offset: state.valve_offset + 1
                        };

                        if let Some(cached_cost) = cost_map.get(&next_state) {
                            if cached_cost <= &next_cost {
                                continue;
                            }
                        }
                        list.push((next_state, next_cost));
                        cost_map.insert(next_state, next_cost);
                    }
                }
            }
        } else {
            dbg!(state,cost,map.keys());
        }
    }
    let mut vect: Vec<(&State, &usize)> = cost_map.iter().collect();
    vect.sort_unstable();
    if let Some((result, _)) = vect.last() {
        return result.score
    }
    0
}




const START: usize = 0;

// fn compact_graph2(graph_node_list:&Vec<Node>) -> HashMap<usize, Node> {
//     println!("compacting graph");
//     let mut map: HashMap<usize, Node> = HashMap::new();
//    let mut node_list: Vec<usize> = Vec::new();
//    let mut node_set: HashSet<usize> = HashSet::new();
//
//
//     graph_node_list.iter().filter(|n| n.flow > 0)
//         .for_each(|n|{
//             node_list.push(n.id);
//             node_set.insert(n.id);
//     });
//     for n in node_list.iter() {
//         println!("{n:3}\t {}", graph_node_list[*n]);
//     }
//
//     node_set.insert(START);
//     node_list.push(START);
//     for i in 0..node_list.len() {
//         add_paths_for_value(&mut map, node_list[i], &node_set, &graph_node_list);
//     }
//     return map;
// }


fn compact_graph(valves: HashMap<Id, Node>) -> HashMap<Id, Node> {
    let mut map: HashMap<Id, Node> = HashMap::new();
    let mut node_list: Vec<Id> = Vec::new();
    let mut node_set: HashSet<Id> = HashSet::new();

    valves
        .iter()
        .filter(|(_, valve)| valve.flow > 0)
        .for_each(|(id, _)| {
            node_list.push(*id);
            node_set.insert(*id);
        });

    node_set.insert(START);
    node_list.push(START);

    for i in 0..node_list.len() {
        add_paths_for_valve(&mut map, node_list[i], &node_set, &valves);
    }

    map
}



#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Connection {
    id: Id,
    cost: usize,
}

impl Display for Connection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}@{}]", self.id, self.cost)
    }
}




fn add_paths_for_value2(map: &mut HashMap<usize, Node>, id: usize, points_of_interest: &HashSet<usize>, valves: &HashMap<Id, Node>) {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut list: VecDeque<Connection> = VecDeque::new();
    let mut connections : Vec<Connection> = Vec::new();

    list.push_back(Connection{ id: id, cost: 0});

    while let Some(current) = list.pop_back() {
        let valve = &valves[&current.id];
        for i in 0..valve.edges.len() {
            let connection = valve.edges[i];

            if !visited.contains(&connection.id) {
                visited.insert(connection.id);
                list.push_back(Connection { id: connection.id, cost: current.cost + connection.cost });

                if connection.id != id && points_of_interest.contains(&connection.id) {
                    connections.push(Connection { id: connection.id, cost: current.cost + connection.cost + 1 });
                }
            }
        }
    }


    map.insert(id, Node {
        id: id,
        name: String::from(""),
        flow: valves[&id].flow,
        edges: connections
    }.clone());



            }
fn add_paths_for_valve(map: &mut HashMap<Id, Node>, id: Id, points_of_interest: &HashSet<Id>, valves: &HashMap<Id, Node>, ) {
    let mut visited: HashSet<Id> = HashSet::new();
    let mut list: VecDeque<Connection> = VecDeque::new();
    let mut connections: Vec<Connection> = Vec::new();

    list.push_back(Connection { id: id, cost: 0 });

    while let Some(current) = list.pop_front() {
        let valve = &valves[&current.id];
        for i in 0..valve.edges.len() {
            let connection = valve.edges[i];

            if !visited.contains(&connection.id) {
                visited.insert(connection.id);
                list.push_back(Connection {
                    id: connection.id,
                    cost: current.cost + connection.cost,
                });

                if connection.id != id && points_of_interest.contains(&connection.id) {
                    connections.push(Connection {
                        id: connection.id,
                        cost: current.cost + connection.cost + 1,
                    });
                }
            }
        }
    }

    map.insert(
        id, Node {
            id,
            name: "".to_string(),
            flow: valves[&id].flow,
            edges: connections
        }
    );



}





fn part1(input_file: &str) -> String {
    let lines = advent_2022::file_to_lines(input_file);
    let (node_list, valve_name_list, id_lookup_by_name, edge_lists) =
    parse_valves(&lines);

    // for n in node_list.iter() {
    //     println!("{}", n );
    // }
    advent_2022::bar();
    // let mut r:HashMap<usize,Node> = HashMap::with_capacity(node_list.len());
    // for i in 0..node_list.len() {
    //     r.insert(i, node_list[i].clone());
    // }

    let mut valves:HashMap<Id,Node> = HashMap::with_capacity(node_list.len());
    for i in 0..node_list.len() {
        valves.insert(i,node_list[i].clone());
    }


 let r = compact_graph(valves) ;
    let mut keys:Vec<usize> = r.keys().map(|k| *k).collect();
    keys.sort();
println!("keys: {}", keys.len());
    println!("{:?}",r);

     for k in keys.iter(){
         let v = &r[k];
         println!("[{}]{k:3}: \t {}",valve_name_list[*k]
             , v);

     }


  let q = solve(&r,30);





    let answer = q;
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
        let mut cons: Vec<Connection> = Vec::new();
        for i in 0..l_edges.len() {
            let n_c = Connection {
                id: l_edges[i],
                cost: 1,
            };
            cons.push(n_c)
        };

        let node = Node {
            id: u,
            name: v_name.to_string(),
            flow: u_flow,
            edges: cons
        };
        node_list.push(node);
    }
    return (node_list, valve_name_list, id_lookup_by_name, edge_lists);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone,Copy,  Debug)]
struct State {
    score: usize,
    position: Id,
    opened_valves: [Id; 50],
    valve_offset:usize
}



#[derive(Debug,  Clone)]
struct Node {
    id:usize,
    name: String,
    flow: usize,
    edges: Vec<Connection>
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {


        write!(f,"Valve[{}] {} has flow: {}, with edges ({}) [{}]", self.id, self.name, self.flow,
               self.edges.len(), advent_2022::list_displayables_to_string(&self.edges))
    }
}

fn part2(input_file: &str) -> String {
    let lines = advent_2022::file_to_lines(input_file);

    let answer = 0;
    return answer.to_string();
}
