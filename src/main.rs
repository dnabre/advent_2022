
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

/*
    Advent of Code 2022: Day 12
        part1 answer: 383
        part2 answer: 377

 */


const TEST_ANSWER: (i32, i32) = (31, 29);
const INPUT_ANSWER: (i32, i32) = (383, 377);


const PART1_TEST_FILENAME: &str = "data/day12/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day12/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day12/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day12/part2_input.txt";

const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("12");

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



fn bfs(grid: &HashMap<(i16, i16), char>, start: (i16, i16), end: (i16, i16), best: i32) -> i32 {
    // this might be closer to an A* than BFS, but I'm basing it on BFS
    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    let mut search_queue: VecDeque<(i16, i16)> = VecDeque::new();
    // delta from x,y for possible neighbors,
    //                          left , right,  up  ,  down
    let deltas = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    // since we're using a HashMap<(x,y),char> neighbors off the edge will just not be found.
    let mut distance_from_start: HashMap<(i16, i16), i32> = HashMap::new();
    distance_from_start.insert(start, 0);
    visited.insert(start);
    search_queue.push_back(start);
    while search_queue.len() > 0 {
        let (x, y) = search_queue.pop_front().unwrap();
        //   println!("visitng ({x},{y}):");
        let mut neighbors: Vec<(i16, i16)> = Vec::new();
        let x_y_dist = *distance_from_start.get(&(x, y)).unwrap();
        if (best > 0) && (x_y_dist+1 > best) {
            return -1;
        }

        for (dx, dy) in &deltas {
            let new_x = x + dx;
            let new_y = y + dy;

            if !visited.contains(&(new_x, new_y)) {
                if grid.contains_key(&(new_x, new_y)) {
                    neighbors.push((new_x, new_y));
                }
            }
        }


        let c_c = *grid.get(&(x, y)).unwrap() as i16;

        //    println!("visitng ({x},{y}), grid: {}, height: {}",d_c_c,c_c );
        //    println!("found {} viable neighbors", neighbors.len());
        for (n_x, n_y) in neighbors {

            let n_c = *grid.get(&(n_x, n_y)).unwrap() as i16; // we've check above if this exists
            //        println!("\t neighbor ({n_x},{n_y}) in grid is: {} (height = {})", d_n_c, n_c);

            if n_c - c_c <= 1 {
                //height of neighbor above current square isn't too high
                search_queue.push_back((n_x, n_y));
                visited.insert((n_x, n_y));
                distance_from_start.insert((n_x, n_y),x_y_dist + 1);

                if (n_x, n_y) == end {
                    return *distance_from_start.get(&(n_x, n_y)).unwrap() as i32;
                }
            }
        }
    }
    return -1;
}

fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };

    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let  lines: Vec<&str> = data1_s.trim().split("\n").map(|t| t.trim()).collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
    }
    let width = lines[0].len();
    let height = lines.len();
    println!("input wxh = {width}x{height}");
    let mut grid: HashMap<(i16, i16), char> = HashMap::new();


    let mut start: (i16, i16) = (-1, -1);
    let mut end: (i16, i16) = (-1, -1);


    for h in 0..height {
        let row: Vec<char> = lines[h].chars().collect();
        for w in 0..width {
            let ch = row[w];
            grid.insert((w as i16, h as i16), ch);
            if ch == 'S' { start = (w as i16, h as i16); }
            if ch == 'E' { end = (w as i16, h as i16); }
        }
    }
    // override grid S and E with min and max height accordingly
    grid.insert(start, 'a');
    grid.insert(end, 'z');

    let aa1 = bfs(&grid, start, end, -1);
    let  answer1 = aa1.to_string();
    return answer1;
}


fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));

    let  lines: Vec<&str> = data2_s.trim().split("\n").map(|t| t.trim()).collect();
    let l_num = lines.len();

    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
    }
    let width = lines[0].len();
    let height = lines.len();
    println!("input wxh = {width}x{height}");
    let mut grid: HashMap<(i16, i16), char> = HashMap::new();


    let mut start: (i16, i16) = (-1, -1);
    let mut end: (i16, i16) = (-1, -1);


    let mut possible_start: Vec<(i16, i16)> = Vec::new();
    for h in 0..height {
        let row: Vec<char> = lines[h].chars().collect();
        for w in 0..width {
            let ch = row[w];
            grid.insert((w as i16, h as i16), ch);
            if ch == 'S' { start = (w as i16, h as i16); }
            if ch == 'E' { end = (w as i16, h as i16); }
            if ch == 'a' {
                possible_start.push((w as i16, h as i16));
            }
        }
    }
    // override grid S and E with min and max height accordingly
    grid.insert(start, 'a');
    grid.insert(end, 'z');
    let mut best =  bfs(&grid, start, end, -1);


    //  for ((s_x,s_y),ch) in grid.iter() {
    //     if *ch == 'a' {
    for (s_x, s_y) in possible_start {
        let d = bfs(&grid, (s_x, s_y), end, -1);
        if  (d>0) &&(best > d) {
            best = std::cmp::min(best,d);

        }
    }

    let aa2 = best;

    let  answer2 = aa2.to_string();
    return answer2;
}
