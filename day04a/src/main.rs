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

            let x1 = assignments[0][0];
            let x2 = assignments[0][1];
            let y1 = assignments[1][0];
            let y2 = assignments[1][1];

            (x1 >= y1 && x2 <= y2) || (y1 >= x1 && y2 <= x2)
        })
        .count()
}

#[cfg(test)]
mod day04a {
    use super::*;

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 2);
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 582);
    }
}
