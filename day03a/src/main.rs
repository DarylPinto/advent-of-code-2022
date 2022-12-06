fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer}");
}

fn to_priority(letter: char) -> usize {
    "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .position(|c| letter == c)
        .unwrap_or(0)
}

fn puzzle(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            let compartments = chars.chunks(chars.len() / 2).collect::<Vec<_>>();
            compartments[0]
                .iter()
                .find(|item| compartments[1].contains(item))
                .map(|&letter| to_priority(letter))
        })
        .sum()
}

#[cfg(test)]
mod day03a {
    use super::*;

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 157);
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 7737);
    }
}
