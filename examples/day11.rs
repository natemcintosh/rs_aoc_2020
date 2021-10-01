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

fn parse_input(grid: &str) -> Vec<Vec<Seat>> {
    let mut result = vec![];
    for line in grid.lines() {
        let row = line.chars().map(Seat::new).collect::<Vec<_>>();
        result.push(row);
    }
    result
}

fn main() {
    let _grid = parse_input(
        std::fs::read_to_string("inputs/day11.txt")
            .expect("Could not read day 11 input")
            .as_str(),
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
    let expected = vec![
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
    let got = parse_input(&input);
    for ((idx, goti), expectedi) in got.iter().enumerate().zip(expected.iter()) {
        println!("{}", idx);
        assert_eq!(goti, expectedi);
    }
}
