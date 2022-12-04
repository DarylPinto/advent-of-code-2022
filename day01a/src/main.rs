fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer}");
}

fn puzzle(input: &str) -> i32 {
    let calorie_list = input
        .lines()
        .map(|line| line.parse::<i32>().ok())
        .collect::<Vec<_>>();

    calorie_list
        .split(|item| item.is_none())
        .map(|x| x.iter().filter_map(|&y| y).sum::<i32>())
        .max()
        .unwrap_or_default()
}

#[cfg(test)]
mod day01a {
    use super::*;

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 24000);
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 75501);
    }
}
