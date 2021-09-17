use itertools::Itertools;

fn solve(inputs: &[u32], n_entries: usize) -> u32 {
    // Go through all length `n_entries` combinations of inputs, if the sum is 2020,
    // return the product of those numbers.
    for combo in inputs.iter().combinations(n_entries) {
        if combo.iter().fold(0u32, |acc, &&item| acc + item) == 2020 {
            return combo.iter().map(|&&i| i as u32).product();
        }
    }
    panic!("Did not find numbers summing to 2020")
}

fn main() {
    // Read in the file of ints, which has one int per line
    let input_ints: Vec<u32> = std::fs::read_to_string("inputs/day01.txt")
        .expect("Could not read day01 input file")
        .lines()
        .map(|line| line.parse().expect("Could not parse line into int"))
        .collect();

    // Solve for the first puzzle
    println!("First puzzle: {}", solve(&input_ints, 2));

    // Solve for the second puzzle
    println!("Second puzzle: {}", solve(&input_ints, 3));
}

#[test]
fn part1() {
    // Test that in the vec [1721, 979, 366, 299, 675, 1456], the sum of the two numbers
    // that sum to 2020 is 514579
    assert_eq!(solve(&[1721, 979, 366, 299, 675, 1456], 2), 514579);
}

#[test]
fn part2() {
    // Test that in the vec [1721, 979, 366, 299, 675, 1456], the sum of the three numbers
    // that sum to 2020 is 241861950
    assert_eq!(solve(&[1721, 979, 366, 299, 675, 1456], 3), 241861950);
}
