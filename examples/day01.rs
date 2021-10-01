use itertools::Itertools;

fn solve(inputs: &[usize], n_entries: usize) -> usize {
    // Go through all length `n_entries` combinations of inputs, if the sum is 2020,
    // return the product of those numbers.
    for combo in inputs.iter().combinations(n_entries) {
        if combo.iter().copied().sum::<usize>() == 2020 {
            return combo.iter().copied().product();
        }
    }
    panic!("Did not find numbers summing to 2020")
}

fn main() {
    let setup_time = std::time::Instant::now();

    // Read in the file of ints, which has one int per line
    let input_ints: Vec<usize> = std::fs::read_to_string("inputs/day01.txt")
        .expect("Could not read day01 input file")
        .lines()
        .map(|line| line.parse().expect("Could not parse line into int"))
        .collect();
    println!(
        "Setup took {:.6} microseconds",
        setup_time.elapsed().as_micros()
    );

    let part1_time = std::time::Instant::now();
    // Solve for the first puzzle
    let part1_soln = solve(&input_ints, 2);
    println!(
        "Part 1 took {:.6} microseconds",
        part1_time.elapsed().as_micros()
    );

    let part2_time = std::time::Instant::now();
    // Solve for the second puzzle
    let part2_soln = solve(&input_ints, 3);
    println!(
        "Part 2 took {:.6} microseconds",
        part2_time.elapsed().as_micros()
    );

    println!();
    println!("First puzzle: {}", part1_soln);
    println!("Second puzzle: {}", part2_soln);
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
