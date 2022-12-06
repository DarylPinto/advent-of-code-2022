const WINDOW_SIZE: usize = 14;

fn main() {
    let input = include_str!("../input.txt");
    let answer = match puzzle(input) {
        Some(solution) => solution.to_string(),
        None => "No solution".to_string(),
    };
    println!("{answer}");
}

fn puzzle(input: &str) -> Option<usize> {
    input
        .as_bytes()
        .windows(WINDOW_SIZE)
        .position(|window| {
            let mut bytes = window.to_vec();
            bytes.sort_unstable();
            bytes.dedup();
            bytes.len() == WINDOW_SIZE
        })
        .map(|index| index + WINDOW_SIZE)
}

#[cfg(test)]
mod day06b {
    use super::*;
    use std::iter::zip;

    #[test]
    fn works_with_sample_input() {
        let input_list = include_str!("../sample.txt").lines();
        let expected_list = [19, 23, 23, 29, 26];
        for (input, expected) in zip(input_list, expected_list) {
            let answer = puzzle(input);
            assert_eq!(answer, Some(expected));
        }
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, Some(3263));
    }
}
