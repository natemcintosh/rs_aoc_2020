use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    instr: Instruction,
    arg: i64,
}

impl Rule {
    fn new(input_str: &str) -> Self {
        let mut parts = input_str.split_whitespace();
        let instr = match parts.next().expect("Could not get Instruction") {
            "nop" => Instruction::Nop,
            "jmp" => Instruction::Jmp,
            "acc" => Instruction::Acc,
            _ => panic!("Instruction was not one of nop, jmp, acc"),
        };

        let arg: i64 = parts
            .next()
            .expect("Could not get argument")
            .parse()
            .expect("Could not convert string into i64");

        Rule { instr, arg }
    }
}

#[derive(Debug)]
struct LoopResult {
    accumulator: i64,
    success: bool,
}

fn main_loop(input: &HashMap<i64, Rule>) -> LoopResult {
    // Create some useful variables
    let last_idx = (input.len() - 1) as i64;
    let mut accumulator: i64 = 0;
    let mut curr_idx: i64 = 0;
    let mut seen_idx: HashSet<i64> = HashSet::new();

    loop {
        if curr_idx > last_idx {
            return LoopResult {
                accumulator,
                success: true,
            };
        } else if seen_idx.contains(&curr_idx) {
            return LoopResult {
                accumulator,
                success: false,
            };
        }

        match input.get(&curr_idx).expect("Gone to a bad index") {
            Rule {
                instr: Instruction::Acc,
                arg,
            } => {
                // add to the accumulator and go back to top of loop
                seen_idx.insert(curr_idx);
                accumulator += arg;
                curr_idx += 1;
                continue;
            }
            Rule {
                instr: Instruction::Jmp,
                arg,
            } => {
                // We jump to a new location
                seen_idx.insert(curr_idx);
                curr_idx += arg;
                continue;
            }
            Rule {
                instr: Instruction::Nop,
                arg: _,
            } => {
                seen_idx.insert(curr_idx);
                curr_idx += 1;
                continue;
            }
        }
    }
}

fn main() {
    // Read in the day 8 input
    let read_time = std::time::Instant::now();
    let rules: Vec<Rule> = std::fs::read_to_string("inputs/day08.txt")
        .expect("Could not read day 8 input file")
        .lines()
        .map(Rule::new)
        .collect();
    let rules_mapped = (0_i64..).zip(rules.into_iter()).collect::<HashMap<_, _>>();
    println!(
        "Constructing operations {:.6} microseconds",
        read_time.elapsed().as_micros()
    );

    // Do part 1
    let part1_time = std::time::Instant::now();
    let part1_solution = main_loop(&rules_mapped);
    println!(
        "Part 1 took {:.6} microseconds",
        part1_time.elapsed().as_micros()
    );

    println!("First Puzzle: {:?}", part1_solution.accumulator);
}

#[test]
fn test_parse_str() {
    let input_str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let got: Vec<_> = input_str.lines().map(Rule::new).collect();
    let expected = [
        Rule {
            instr: Instruction::Nop,
            arg: 0,
        },
        Rule {
            instr: Instruction::Acc,
            arg: 1,
        },
        Rule {
            instr: Instruction::Jmp,
            arg: 4,
        },
        Rule {
            instr: Instruction::Acc,
            arg: 3,
        },
        Rule {
            instr: Instruction::Jmp,
            arg: -3,
        },
        Rule {
            instr: Instruction::Acc,
            arg: -99,
        },
        Rule {
            instr: Instruction::Acc,
            arg: 1,
        },
        Rule {
            instr: Instruction::Jmp,
            arg: -4,
        },
        Rule {
            instr: Instruction::Acc,
            arg: 6,
        },
    ];

    for (goti, expecti) in got.iter().zip(expected.iter()) {
        assert_eq!(goti, expecti);
    }
}

#[test]
fn test_part1() {
    let input_str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let input: Vec<Rule> = input_str.lines().map(Rule::new).collect();
    let input_mapped = (0_i64..).zip(input.into_iter()).collect::<HashMap<_, _>>();
    let got = main_loop(&input_mapped);
    assert_eq!(got.accumulator, 5);
}
