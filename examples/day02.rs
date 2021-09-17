fn part1(inputs: &[Password]) -> usize {
    // Iterate over the inputs, filter out bad ones, and count those that get through
    inputs.iter().filter(|line| part1_pw_is_valid(line)).count()
}

fn part2(inputs: &[Password]) -> usize {
    // Iterate over the inputs, filter out bad ones, and count those that get through
    inputs.iter().filter(|line| part2_pw_is_valid(line)).count()
}

#[derive(Debug)]
struct Password<'a> {
    lower: usize,
    upper: usize,
    letter: char,
    password: &'a str,
}

fn new_password(line: &str) -> Password {
    // Examples:
    // 1-3 a: abcde
    // 1-3 b: cdefg
    // 2-9 c: ccccccccc

    // Split on whitespace
    let parts: Vec<&str> = line.split_whitespace().collect();

    // The first part is the range
    let lower_upper: Vec<&str> = parts[0].split('-').collect();
    let lower: usize = lower_upper[0].parse().expect("Could not parse lower bound");
    let upper: usize = lower_upper[1].parse().expect("Could not parse upper bound");

    // The second part is the letter that must exist in the password
    let letter = parts[1]
        .chars()
        .next()
        .expect("Could not get that required letter");

    // The third part is the password
    let password = parts[2];

    Password {
        lower,
        upper,
        letter,
        password,
    }
}

fn part1_pw_is_valid(pw: &Password) -> bool {
    // The password policy indicates the lowest and highest number of times a given
    // letter must appear for the password to be valid. For example, 1-3 a means that
    // the password must contain a at least 1 time and at most 3 times.
    // Examples:
    // 1-3 a: abcde
    // 1-3 b: cdefg
    // 2-9 c: ccccccccc

    // Count how many times the letter exists in the password
    let letter_count = pw.password.chars().filter(|&c| c == pw.letter).count();

    (pw.lower <= letter_count) && (letter_count <= pw.upper)
}

fn part2_pw_is_valid(pw: &Password) -> bool {
    // Each policy actually describes two positions in the password, where 1 means the
    // first character, 2 means the second character, and so on. (Be careful; Toboggan
    // Corporate Policies have no concept of "index zero"!) Exactly one of these
    // positions must contain the given letter. Other occurrences of the letter are
    // irrelevant for the purposes of policy enforcement.

    // Given the same example list from above:

    // 1-3 a: abcde is valid: position 1 contains a and position 3 does not.
    // 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
    // 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.

    // How many passwords are valid according to the new interpretation of the policies?

    // Convert lower and upper to indices
    let lower_pos = pw.lower - 1;
    let upper_pos = pw.upper - 1;

    // Check that the upper index actually exist
    if upper_pos >= pw.password.len() {
        return false;
    }

    let char_in_first_pos = pw
        .password
        .chars()
        .nth(lower_pos)
        .expect("Couldn't get lower character from password")
        == pw.letter;

    let char_in_second_pos = pw
        .password
        .chars()
        .nth(upper_pos)
        .expect("Couldn't get upper character from password")
        == pw.letter;

    let mysum = (char_in_first_pos as usize) + (char_in_second_pos as usize);

    mysum == 1
}

fn main() {
    // Read in the input
    let input =
        std::fs::read_to_string("inputs/day02.txt").expect("Could not read day02 input file");
    let input_lines: Vec<Password> = input.as_str().lines().map(|l| new_password(l)).collect();

    // Solve for the first puzzle
    println!("First puzzle: {}", part1(&input_lines));

    // Solve the second puzzle
    println!("First puzzle: {}", part2(&input_lines));
}

#[test]
fn test_part1() {
    // Test that for the input &strs:
    // 1-3 a: abcde
    // 1-3 b: cdefg
    // 2-9 c: ccccccccc
    // the result is 2
    let input: Vec<Password> = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
        .iter()
        .map(|&p| new_password(p))
        .collect();

    assert_eq!(part1(&input), 2);
}

#[test]
fn test_part2() {
    // Test that for the input &strs:
    // 1-3 a: abcde
    // 1-3 b: cdefg
    // 2-9 c: ccccccccc
    // the result is 1
    let input: Vec<Password> = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
        .iter()
        .map(|&p| new_password(p))
        .collect();

    assert_eq!(part2(&input), 1);
}
