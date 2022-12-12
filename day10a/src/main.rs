mod types;
use types::{Cpu, CpuHistory, Instruction};

fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer}");
}

fn puzzle(input: &str) -> i32 {
    let history: CpuHistory = input
        .lines()
        .filter_map(|line| -> Option<Instruction> { line.try_into().ok() })
        .fold(CpuHistory(vec![Cpu(1)]), |history, instruction| {
            history.execute(instruction)
        });

    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|cycle| {
            let index = cycle - 1;
            let cycle = cycle as i32;
            let value = history.0[index].0;
            value * cycle
        })
        .sum()
}

#[cfg(test)]
mod template {
    use super::*;

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 13140);
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 17380);
    }
}
