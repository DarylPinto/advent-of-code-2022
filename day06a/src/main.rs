const WINDOW_SIZE: usize = 4;

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
mod day06a {
    use super::*;
    use std::iter::zip;

    #[test]
    fn works_with_sample_input() {
        let input_list = include_str!("../sample.txt").lines();
        let expected_list = [7, 5, 6, 10, 11];
        for (input, expected) in zip(input_list, expected_list) {
            let answer = puzzle(input);
            assert_eq!(answer, Some(expected));
        }
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, Some(1361));
    }
}
