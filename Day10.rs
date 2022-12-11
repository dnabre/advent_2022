use std::fmt;
use std::fs;
use std::time::Instant;
use parse_display::FromStr;

/*
    Advent of Code 2022: Day 10
        part1 answer: 15220
        part2 answer: RFZEKBFA

 */


const TEST_ANSWER: (u32, &str) = (13140, concat!(
"██..██..██..██..██..██..██..██..██..██..\n",
"███...███...███...███...███...███...███.\n",
"████....████....████....████....████....\n",
"█████.....█████.....█████.....█████.....\n",
"██████......██████......██████......████\n",
"███████.......███████.......███████.....\n"));

const INPUT_ANSWER: (u32, &str) = (15220, concat!(
"███..████.████.████.█..█.███..████..██..\n",
"█..█.█.......█.█....█.█..█..█.█....█..█.\n",
"█..█.███....█..███..██...███..███..█..█.\n",
"███..█.....█...█....█.█..█..█.█....████.\n",
"█.█..█....█....█....█.█..█..█.█....█..█.\n",
"█..█.█....████.████.█..█.███..█....█..█.\n"));

const PART1_TEST_FILENAME: &str = "data/day10/part1_test.txt";
const PART1_INPUT_FILENAME: &str = "data/day10/part1_input.txt";

const PART2_TEST_FILENAME: &str = "data/day10/part2_test.txt";
const PART2_INPUT_FILENAME: &str = "data/day10/part2_input.txt";

const TEST: bool = false;

fn main() {
    print!("Advent of Code 2022, Day ");
    println!("10");

    let start1 = Instant::now();
    let answer1 = part1();
    let duration1 = start1.elapsed();

    println!("\t Part 1: {answer1:>20}, time: {:?}", duration1);

    if TEST {
        assert_eq!(answer1, TEST_ANSWER.0.to_string());
    } else {
        assert_eq!(answer1, INPUT_ANSWER.0.to_string());
    }

    let start2 = Instant::now();
    let answer2 = part2();
    let duration2 = start2.elapsed();
    println!("\t Part 2:  {:>20}, time: {:?}", "<>", duration2);
    println!("\t\t\t{}", answer2.replace("\n", "\n\t\t\t"));

    if TEST {
        assert_eq!(answer2, TEST_ANSWER.1.to_string());
    } else {
        assert_eq!(answer2, INPUT_ANSWER.1.to_string());
    }

    println!("----------\ndone");
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Machine {
    register: i32,
    cycle_counter: usize,
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\t machine, cycle {:>12}, register {:>12}", self.cycle_counter, self.register)
    }
}

impl Default for Machine {
    fn default() -> Machine {
        Machine { cycle_counter: 0, register: 1 }
    }
}

#[derive(FromStr, PartialEq, Debug, Copy, Clone)]
enum Instruction {
    #[display("noop")]
    Noop,
    #[display("addx {0}")]
    Addx(i32),
    #[display("<wait>")]
    WaitState,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Noop => { write!(f, "noop") }
            Instruction::Addx(x) => { write!(f, "addx {x}") }
            Instruction::WaitState => { write!(f, "<wait>") }
        }
    }
}

fn build_code_vector(lines: Vec<&str>) ->  Vec<Instruction>{
    let mut code: Vec<Instruction> = Vec::new();
    for ln in lines {
        let i: Instruction = ln.parse().unwrap();
        match i {
            Instruction::Noop => { code.push(i) }
            Instruction::Addx(x) => {
                code.push(Instruction::WaitState);
                code.push(Instruction::Addx(x))
            }
            Instruction::WaitState => {}
        }
    }
    return code;
}

fn part1() -> String {
    let p1_file = match TEST {
        true => PART1_TEST_FILENAME,
        false => PART1_INPUT_FILENAME
    };

    let data1_s =
        fs::read_to_string(p1_file).expect(&*format!("error opening file {}", p1_file));
    let lines: Vec<&str> = data1_s.trim().split("\n").map(|t| t.trim_end()).collect();
    let l_num = lines.len();
    if TEST {
        println!("\t read {} lines from {}", l_num, p1_file);
    }

    let mut code = build_code_vector(lines);
    code.insert(0,Instruction::WaitState); //fix timing mismatched between part 1 & 2;

    let mut m: Machine = Default::default();
    let mut total_ss = 0;

    for i in code {
        match i {
            Instruction::Noop => {}
            Instruction::Addx(x) => { m.register = m.register + x; }
            Instruction::WaitState => {}
        }
        m.cycle_counter += 1;
        if m.cycle_counter % 40 == 20 {
            let ss = m.cycle_counter * (m.register as usize);
            total_ss += ss;
        }
    }

    let answer1 = total_ss.to_string();
    return answer1;
}



fn part2() -> String {
    let p2_file = match TEST {
        true => PART2_TEST_FILENAME,
        false => PART2_INPUT_FILENAME
    };
    let data2_s =
        fs::read_to_string(p2_file).expect(&*format!("error opening file {}", p2_file));

    let lines: Vec<&str> = data2_s.trim().split("\n").map(|t| t.trim_end()).collect();
    let l_num = lines.len();

    if TEST {
        println!("\t read {} lines from {}", l_num, p2_file);
    }

    let  code: Vec<Instruction> = build_code_vector(lines);
    let mut m: Machine = Default::default();
    let mut crt = String::new();

    for i in code {
        let inner_sprite_index = ((m.cycle_counter as i32) % 40) - m.register;
        let new_crt_char;
        if [-1, 0, 1].contains(&inner_sprite_index) {
            new_crt_char = '█';
        } else {
            new_crt_char = '.';
        }
        crt.push(new_crt_char);

        match i {
            Instruction::Noop => {}
            Instruction::Addx(x) => { m.register = m.register + x; }
            Instruction::WaitState => {}
        }

        m.cycle_counter += 1;
        if m.cycle_counter % 40 == 0 {
            crt.push('\n');
        }
    }

    let answer2 = crt;
    return answer2;
}
