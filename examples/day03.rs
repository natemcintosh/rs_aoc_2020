fn create_tree_field(input: &str) -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    for row in input.lines() {
        result.push(row.chars().map(|c| c == '#').collect());
    }
    result
}

fn count_n_trees_hit(field: &[Vec<bool>], stride: &Stride) -> usize {
    // Get the number of rows and columns in the field
    let nrows = field.len();
    let ncols = field[0].len();

    // Calculate the indices of all visited points
    let col_indices: Vec<usize> = (0..nrows).map(|i| (i * stride.right) % ncols).collect();

    let row_indices: Vec<usize> = (0..nrows).step_by(stride.down).collect();

    // Filter if they are true, then count()
    col_indices
        .iter()
        .zip(row_indices.iter())
        .filter(|(&col, &row)| field[row][col])
        .count()
}

struct Stride {
    right: usize,
    down: usize,
}

fn main() {
    let setup_time = std::time::Instant::now();
    let input_str =
        std::fs::read_to_string("inputs/day03.txt").expect("Could not read day 3 input");
    let field = create_tree_field(input_str.as_str());
    println!(
        "Setup took {:.6} microseconds",
        setup_time.elapsed().as_micros()
    );

    let part1_time = std::time::Instant::now();
    // Solve for the first puzzle
    let part1_stride = Stride { right: 3, down: 1 };
    let part1_soln = count_n_trees_hit(&field, &part1_stride);
    println!(
        "Part 1 took {:.6} microseconds",
        part1_time.elapsed().as_micros()
    );

    let part2_time = std::time::Instant::now();
    let part2_strides = vec![
        Stride { right: 1, down: 1 },
        Stride { right: 3, down: 1 },
        Stride { right: 5, down: 1 },
        Stride { right: 7, down: 1 },
        Stride { right: 1, down: 2 },
    ];
    let part2_solution: usize = part2_strides
        .iter()
        .map(|s| count_n_trees_hit(&field, s))
        .product();
    println!(
        "Part 2 took {:.6} microseconds",
        part2_time.elapsed().as_micros()
    );

    println!("First puzzle: {}", part1_soln);
    println!("Second puzzle: {}", part2_solution);
}

#[test]
fn test_read_array() {
    let input_str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    let expected = vec![
        vec![
            false, false, true, true, false, false, false, false, false, false, false,
        ],
        vec![
            true, false, false, false, true, false, false, false, true, false, false,
        ],
        vec![
            false, true, false, false, false, false, true, false, false, true, false,
        ],
        vec![
            false, false, true, false, true, false, false, false, true, false, true,
        ],
        vec![
            false, true, false, false, false, true, true, false, false, true, false,
        ],
        vec![
            false, false, true, false, true, true, false, false, false, false, false,
        ],
        vec![
            false, true, false, true, false, true, false, false, false, false, true,
        ],
        vec![
            false, true, false, false, false, false, false, false, false, false, true,
        ],
        vec![
            true, false, true, true, false, false, false, true, false, false, false,
        ],
        vec![
            true, false, false, false, true, true, false, false, false, false, true,
        ],
        vec![
            false, true, false, false, true, false, false, false, true, false, true,
        ],
    ];

    let got = create_tree_field(input_str);
    for (got_row, expected_row) in got.iter().zip(expected.iter()) {
        assert_eq!(got_row, expected_row);
    }
}

#[test]
fn test_part1() {
    let input_str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    let field = create_tree_field(input_str);
    let stride = Stride { right: 3, down: 1 };
    let n_trees_hit = count_n_trees_hit(&field, &stride);
    assert_eq!(n_trees_hit, 7);
}

#[test]
fn test_part2() {
    let input_str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    let field = create_tree_field(input_str);
    let part2_strides = vec![
        Stride { right: 1, down: 1 },
        Stride { right: 3, down: 1 },
        Stride { right: 5, down: 1 },
        Stride { right: 7, down: 1 },
        Stride { right: 1, down: 2 },
    ];
    let part2_solution: usize = part2_strides
        .iter()
        .map(|s| count_n_trees_hit(&field, s))
        .product();
    assert_eq!(part2_solution, 336);
}
