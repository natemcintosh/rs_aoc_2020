use itertools::Itertools;

fn number_is_bad(window: &[usize], number: usize) -> Option<usize> {
    if window
        .iter()
        .combinations(2)
        .all(|combo| combo.into_iter().sum::<usize>() != number)
    {
        return Some(number);
    } else {
        return None;
    }
}

fn part1(ints: &[usize], preamble_size: usize) -> usize {
    ints.windows(preamble_size + 1)
        .map(|w| number_is_bad(&w[..preamble_size], *w.last().unwrap()))
        .find(|&o| o.is_some())
        .expect("Could not find a bad number")
        .expect("Could not find a bad number")
}

fn part2(ints: &[usize], to_sum_to: usize) -> usize {
    // Iterate over all the possible sizes of the window
    for window_size in 2..ints.len() {
        // Search for some continuous set of numbers that sums to `to_sum_to`
        let good_window = ints
            .windows(window_size)
            .find(|&w| w.iter().sum::<usize>() == to_sum_to);

        // If they did sum to `to_sum_to`, then find the min, and the max, and return
        // their sum
        if let Some(winow) = good_window {
            let min = winow
                .iter()
                .reduce(|a, b| a.min(b))
                .expect("Could not find a minimum");
            let max = winow
                .iter()
                .reduce(|a, b| a.max(b))
                .expect("Could not find a minimum");
            return min + max;
        }
    }
    panic!("Could not find a solution");
}

fn main() {
    println!("Hello world");
    let ints: Vec<usize> = std::fs::read_to_string("inputs/day09.txt")
        .expect("Could not read day 9 input")
        .lines()
        .map(|l| l.parse().expect("Could not parse into usize"))
        .collect();

    let part1_time = std::time::Instant::now();
    let part1_soln = part1(&ints, 25);
    println!(
        "Part 1 took {:.6} microseconds",
        part1_time.elapsed().as_micros()
    );

    let part2_time = std::time::Instant::now();
    let part2_soln = part2(&ints, part1_soln);
    println!(
        "Part 2 took {:.6} microseconds",
        part2_time.elapsed().as_micros()
    );

    println!("First Puzzle: {:?}", part1_soln);
    println!("Second Puzzle: {:?}", part2_soln);
}

#[test]
fn test_part1() {
    let ints: Vec<usize> = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    let preamble_size: usize = 5;
    let got = part1(&ints, preamble_size);
    let expected: usize = 127;
    assert_eq!(got, expected);
}

#[test]
fn test_part2() {
    let ints: Vec<usize> = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    let to_sum_to: usize = 127;
    let got = part2(&ints, to_sum_to);
    let expected: usize = 62;
    assert_eq!(got, expected);
}
