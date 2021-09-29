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
    println!("First Puzzle: {:?}", part1_soln);
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
