use std::collections::HashSet;

use itertools::Itertools;

fn part2(groups: &[&str]) -> usize {
    groups
        .iter()
        .map(|&g| {
            g.lines()
                .map(|person| person.chars().collect::<HashSet<char>>())
                .reduce(|acc, person| acc.intersection(&person).copied().collect::<HashSet<_>>())
                .expect("Could not calculate intersection of sets")
                .len()
        })
        .sum()
}

fn main() {
    let setup_time = std::time::Instant::now();
    let input_str =
        std::fs::read_to_string("inputs/day06.txt").expect("Could not read day 6 input file");
    let groups: Vec<&str> = input_str.as_str().split("\n\n").collect();
    println!(
        "Setup took {:.6} microseconds",
        setup_time.elapsed().as_micros()
    );

    let part1_time = std::time::Instant::now();
    let part1_solution: usize = groups
        .iter()
        .map(|&g| g.replace("\n", ""))
        .map(|g| g.chars().unique().count())
        .sum();
    println!(
        "Part 1 took {:.6} microseconds",
        part1_time.elapsed().as_micros()
    );

    let part2_time = std::time::Instant::now();
    let part2_solution = part2(&groups);
    println!(
        "Part 2 took {:.6} microseconds",
        part2_time.elapsed().as_micros()
    );

    println!();
    println!("First puzzle {:?}", part1_solution);
    println!("Second puzzle: {}", part2_solution);
}

#[test]
fn test_part1() {
    let input = "abc

a
b
c

ab
ac

a
a
a
a

b
";
    let groups: Vec<&str> = input.split("\n\n").collect();

    let part1_solution: usize = groups
        .iter()
        .map(|&g| g.replace("\n", "").chars().unique().count())
        .sum();

    assert_eq!(part1_solution, 11);
}

#[test]
fn test_part2() {
    let input = "abc

a
b
c

ab
ac

a
a
a
a

b
";
    let groups: Vec<&str> = input.split("\n\n").collect();

    let part2_solution: usize = part2(&groups);

    assert_eq!(part2_solution, 6);
}
