mod types;
use types::Weapon::{self, *};

fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer}");
}

fn calculate_score(weapons: (Weapon, Weapon)) -> i32 {
    let outcome_score: i32 = match &weapons {
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 0,
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
        _ => 3,
    };
    (weapons.1 as i32) + outcome_score
}

fn puzzle(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(' ').and_then(|(their_weapon, my_weapon)| {
                let my_weapon: Weapon = my_weapon.try_into().ok()?;
                let their_weapon: Weapon = their_weapon.try_into().ok()?;
                Some((their_weapon, my_weapon))
            })
        })
        .map(calculate_score)
        .sum()
}

#[cfg(test)]
mod day02a {
    use super::*;

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 15);
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 10624);
    }
}
