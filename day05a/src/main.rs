mod matrix;

fn main() {
    let input = include_str!("../input.txt");
    let answer = match puzzle(input) {
        Some(solution) => solution,
        None => "No solution".into(),
    };
    println!("{answer}");
}

fn puzzle(input: &str) -> Option<String> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut section_iter = lines.split(|line| line.is_empty());

    let initial_crates = section_iter
        .next()?
        .split_last()?
        .1
        .iter()
        .map(|line| line.chars().skip(1).step_by(4).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let instructions = section_iter
        .next()?
        .iter()
        .filter_map(|line| {
            let mut num_iter = line
                .split_whitespace()
                .filter_map(|word| word.parse::<usize>().ok());
            Some((num_iter.next()?, num_iter.next()? - 1, num_iter.next()? - 1))
        })
        .collect::<Vec<_>>();

    let crates = matrix::rotate(&initial_crates);
    let crates = crates
        .iter()
        .map(|stack| {
            stack
                .iter()
                .filter(|c| c.is_alphabetic())
                .cloned()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Some(
        instructions
            .iter()
            .fold(crates, |acc, (amount, from, to)| {
                let source = acc[*from].clone();
                let mut dest = acc[*to].clone();

                let (source, lifted) = source.split_at(source.len() - amount);
                dest.extend(lifted.iter().rev());

                acc.iter()
                    .enumerate()
                    .map(|(i, stack)| {
                        if &i == from {
                            source.to_vec()
                        } else if &i == to {
                            dest.clone()
                        } else {
                            stack.to_vec()
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .iter()
            .filter_map(|stack| stack.last())
            .collect::<String>(),
    )
}

#[cfg(test)]
mod day05b {
    use super::*;

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, Some("CMZ".into()));
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, Some("BSDMQFLSP".into()));
    }
}
