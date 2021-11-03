use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seat {
    Ground,
    Empty,
    Occupied,
}

impl Seat {
    fn new(input: char) -> Seat {
        match input {
            '.' => Seat::Ground,
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            _ => panic!("input character was not one of '.', 'L', '#'"),
        }
    }
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn parse_input(grid: &str) -> HashMap<(i32, i32), Seat> {
    let mut result = HashMap::new();
    for (row, line) in grid.lines().enumerate() {
        for (col, seat_char) in line.chars().enumerate() {
            result.insert((row as i32, col as i32), Seat::new(seat_char));
        }
    }
    result
}

fn count_adjacent_occupied(loc: (i32, i32), grid: &HashMap<(i32, i32), Seat>) -> usize {
    DIRECTIONS
        .iter()
        // Generate the locations adjacent to `loc`
        .map(|&dir| (dir.0 + loc.0, dir.1 + loc.1))
        // Remove any that are not in the grid
        .map(|dir| grid.get(&dir))
        .filter_map(|seat| match seat {
            Some(_) => seat,
            None => None,
        })
        // Check if the seat is occupied at that location
        .filter(|&&seat| seat == Seat::Occupied)
        // Count
        .count()
}

fn seat_iter(
    layout: &HashMap<(i32, i32), Seat>,
    adjacency_fn: fn((i32, i32), &HashMap<(i32, i32), Seat>) -> usize,
    tolerance: usize,
) -> (HashMap<(i32, i32), Seat>, usize) {
    let occupied_adjacent_count: HashMap<(i32, i32), usize> = layout
        .iter()
        .map(|(&loc, _)| (loc, adjacency_fn(loc, &layout)))
        .collect();

    // At empty spots, check for 0 occupied seats adjacent to it
    let all_empty_adjacents: HashSet<(i32, i32)> = layout
        .iter()
        .filter(|(_, &seat)| seat == Seat::Empty)
        .filter(|(loc, _)| occupied_adjacent_count.get(loc).unwrap().eq(&0))
        .map(|(&loc, _)| loc)
        .collect();

    // At occupied spots, check for four or more occupied adjacent seats
    let four_or_more: HashSet<(i32, i32)> = layout
        .iter()
        .filter(|(_, &seat)| seat == Seat::Occupied)
        .filter(|(loc, _)| occupied_adjacent_count.get(loc).unwrap().ge(&tolerance))
        .map(|(&loc, _)| loc)
        .collect();

    // Make a copy to return
    let mut new_layout = layout.clone();

    // Carry out the changes
    for (loc, seat) in new_layout.iter_mut() {
        if all_empty_adjacents.contains(loc) {
            *seat = Seat::Occupied;
        } else if four_or_more.contains(loc) {
            *seat = Seat::Empty;
        }
    }

    let nchanges: usize = all_empty_adjacents.len() + four_or_more.len();

    return (new_layout, nchanges);
}

fn solve(
    layout: &HashMap<(i32, i32), Seat>,
    adjacency_fn: fn((i32, i32), &HashMap<(i32, i32), Seat>) -> usize,
    tolerance: usize,
) -> usize {
    let mut n_changes: usize = 1;
    let mut changing_layout = layout.clone();
    while n_changes > 0 {
        let results = seat_iter(&changing_layout, adjacency_fn, tolerance);
        changing_layout = results.0;
        n_changes = results.1;
    }
    changing_layout
        .values()
        .filter(|&&seat| seat == Seat::Occupied)
        .count()
}

fn part1(layout: &HashMap<(i32, i32), Seat>) -> usize {
    solve(layout, count_adjacent_occupied, 4)
}

// fn part2(layout: &HashMap<(i32, i32), Seat>) -> usize {
//     solve(layout, adjacency_fn, 5)
// }

fn main() {
    let setup_time = std::time::Instant::now();
    let layout = parse_input(
        std::fs::read_to_string("inputs/day11.txt")
            .expect("Could not read day 11 input")
            .as_str(),
    );
    println!(
        "Setup took {:.6} microseconds",
        setup_time.elapsed().as_micros()
    );

    let part1_time = std::time::Instant::now();
    let part1_soln = part1(&layout);
    println!(
        "Part 1 took {:.6} microseconds",
        part1_time.elapsed().as_micros()
    );
}

#[test]
fn test_parse_1() {
    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
    let expected_vec = vec![
        vec![
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Empty,
        ],
        vec![
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Empty,
        ],
        vec![
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Ground,
            Seat::Ground,
            Seat::Empty,
            Seat::Ground,
            Seat::Ground,
        ],
        vec![
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Empty,
        ],
        vec![
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Empty,
        ],
        vec![
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Empty,
        ],
        vec![
            Seat::Ground,
            Seat::Ground,
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Ground,
            Seat::Ground,
            Seat::Ground,
            Seat::Ground,
            Seat::Ground,
        ],
        vec![
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
        ],
        vec![
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
        ],
        vec![
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Empty,
            Seat::Ground,
            Seat::Empty,
            Seat::Empty,
        ],
    ];
    let mut expected: HashMap<(i32, i32), Seat> = HashMap::new();
    for (row_idx, row) in expected_vec.iter().enumerate() {
        for (col_idx, &seat) in row.iter().enumerate() {
            expected.insert((row_idx as i32, col_idx as i32), seat);
        }
    }

    let got = parse_input(&input);

    assert_eq!(got, expected);
}

#[test]
fn test_part1_iter() {
    let input: Vec<&str> = vec![
        "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
        "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
        "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##",
        "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##",
    ];
    // Only works if we trutst parse_input
    let layouts: Vec<HashMap<(i32, i32), Seat>> = input.iter().map(|&s| parse_input(s)).collect();

    // Iterate from the first for the length of `layouts`
    let mut got = vec![layouts[0].clone()];
    for idx in 0..layouts.len() {
        let new_result = seat_iter(&layouts[idx], count_adjacent_occupied, 4);
        let new_layout = new_result.0;
        got.push(new_layout);
    }

    // Compare that what we got is what we expected
    for (exptected, got) in layouts.iter().zip(got.iter()) {
        for (key1, key2) in exptected.keys().sorted().zip(got.keys().sorted()) {
            assert_eq!(key1, key2, "expected key {:?} ≠ got key {:?}", key1, key2);
            assert_eq!(
                exptected[key1], got[key2],
                "expected value {:?} ≠ got value {:?}",
                exptected[key1], got[key2]
            );
        }
    }
}

#[test]
fn test_part1_solve() {
    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
    let layout = parse_input(input);
    let got = part1(&layout);
    let expected: usize = 37;
    assert_eq!(expected, got);
}
