fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer}");
}

fn puzzle(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let assignments = line
                .split(',')
                .map(|assignment| {
                    assignment
                        .split('-')
                        .filter_map(|n| n.parse::<i32>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let assignment_one = assignments[0][0]..=assignments[0][1];
            let assignment_two = assignments[1][0]..=assignments[1][1];

            assignment_one
                .filter(|n| assignment_two.contains(n))
                .next()
                .is_some()
        })
        .count()
}

#[cfg(test)]
mod day04b {
    use super::*;

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 4);
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 893);
    }
}
