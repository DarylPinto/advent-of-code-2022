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

    let mut calorie_sum_list = calorie_list
        .split(|item| item.is_none())
        .map(|x| x.iter().filter_map(|&y| y).sum::<i32>())
        .collect::<Vec<_>>();

    calorie_sum_list.sort_unstable();

    calorie_sum_list.iter().rev().take(3).sum()
}

#[cfg(test)]
mod day01b {
    use super::*;

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 45000);
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 215594);
    }
}
