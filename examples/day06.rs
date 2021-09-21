use std::collections::HashSet;

use itertools::Itertools;

fn part2(groups: &[&str]) -> usize {
    groups
        .iter()
        .map(|&g| {
            g.lines()
                .map(|person| person.chars().collect::<HashSet<char>>())
                .reduce(|acc, person| {
                    acc.intersection(&person)
                        .copied()
                        .collect::<HashSet<_>>()
                })
                .expect("Could not calculate intersection of sets")
                .len()
        })
        .sum()
}

fn main() {
    let input_str =
        std::fs::read_to_string("inputs/day06.txt").expect("Could not read day 6 input file");
    let groups: Vec<&str> = input_str.as_str().split("\n\n").collect();

    let part1_solution: usize = groups
        .iter()
        .map(|&g| g.replace("\n", ""))
        .map(|g| g.chars().unique().count())
        .sum();
    println!("First puzzle {:?}", part1_solution);

    let part2_solution = part2(&groups);
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
