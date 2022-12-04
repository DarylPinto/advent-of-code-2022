fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer}");
}

fn puzzle(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod template {
    use super::*;

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 0);
    }
}
