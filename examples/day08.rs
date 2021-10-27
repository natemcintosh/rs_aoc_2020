use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

fn flip_code(code: &HashMap<i64, Rule>, idx: i64) -> HashMap<i64, Rule> {
    let r = code.get(&idx).expect("Gone to a bad index");
    let new_instr = match r.instr {
        Instruction::Jmp => Instruction::Nop,
        Instruction::Nop => Instruction::Jmp,
        Instruction::Acc => panic!("Shouldn't swap an acc"),
    };
    let new_rule = Rule {
        instr: new_instr,
        arg: r.arg,
    };

    let mut new = clone_hashmap(code);
    if let Some(x) = new.get_mut(&idx) {
        *x = new_rule;
    }

    new
}

fn clone_hashmap<K: Clone, V: Clone>(data: &HashMap<K, V>) -> HashMap<K, V> {
    data.clone()
}

fn part2(code: &HashMap<i64, Rule>) -> i64 {
    // Find the keys of `code` that are nop or jmp
    let nops_or_jmps: Vec<i64> = code
        .iter()
        .filter(|(_, &v)| (v.instr == Instruction::Jmp) || (v.instr == Instruction::Nop))
        .map(|(&k, _)| k)
        .collect();

    // For each location, flip the instruction and run the main loop
    nops_or_jmps
        .iter()
        .map(|&idx| flip_code(code, idx))
        .map(|x| main_loop(&x))
        .find(|x| x.success)
        .expect("Could not find a successful loop")
        .accumulator
}

fn main() {
    // Read in the day 8 input
    let setup_time = std::time::Instant::now();
    let rules_mapped = (0_i64..)
        .zip(
            std::fs::read_to_string("inputs/day08.txt")
                .expect("Could not read day 8 input file")
                .lines()
                .map(Rule::new),
        )
        .collect::<HashMap<_, _>>();
    println!(
        "Constructing operations {:.6} microseconds",
        setup_time.elapsed().as_micros()
    );

    // Do part 1
    let part1_time = std::time::Instant::now();
    let part1_solution = main_loop(&rules_mapped);
    println!(
        "Part 1 took {:.6} microseconds",
        part1_time.elapsed().as_micros()
    );

    // Time and run part 2
    let part2_time = std::time::Instant::now();
    let part2_solution = part2(&rules_mapped);
    println!(
        "Part 2 took {:.6} microseconds",
        part2_time.elapsed().as_micros()
    );

    println!();
    println!("First Puzzle: {:?}", part1_solution.accumulator);
    println!("Second Puzzle: {:?}", part2_solution);
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

#[test]
fn test_part2() {
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
    let got = part2(&input_mapped);
    assert_eq!(got, 8);
}
