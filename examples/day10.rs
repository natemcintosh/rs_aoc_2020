use counter::Counter;

fn put_in_world(starting_ints: &mut Vec<usize>) {
    let max = starting_ints
        .iter()
        .reduce(|a, b| &a.max(b))
        .expect("Could not find a maximum");

    let to_add = vec![0, *max + 3];
    starting_ints.extend(to_add.iter());
    starting_ints.sort_unstable();
}

fn part1(ints: &[usize]) -> usize {
    ints.windows(2)
        .map(|w| {
            w.last().expect("Could not get last") - w.iter().next().expect("Could not get first")
        })
        .collect::<Counter<_>>()
        .iter()
        .map(|(_, &val)| val)
        .product()
}

fn main() {
    let mut ints: Vec<usize> = std::fs::read_to_string("inputs/day10.txt")
        .expect("Could not read day 10 input")
        .lines()
        .map(|l| l.parse().expect("Could not parse to usize"))
        .collect();
    put_in_world(&mut ints);

    let part1_time = std::time::Instant::now();
    let part1_soln = part1(&ints);
    println!(
        "Part 1 took {:.6} microseconds",
        part1_time.elapsed().as_micros()
    );

    println!("First Puzzle: {:?}", part1_soln);
}

#[test]
fn test_part1_small() {
    let mut ints: Vec<usize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    put_in_world(&mut ints);

    let got = part1(&ints);
    let expected: usize = 35;
    assert_eq!(got, expected);
}

#[test]
fn test_part1_big() {
    let mut ints: Vec<usize> = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    put_in_world(&mut ints);

    let got = part1(&ints);
    let expected: usize = 220;
    assert_eq!(got, expected);
}
