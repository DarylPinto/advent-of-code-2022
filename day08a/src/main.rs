mod matrix;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer}");
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Tree((usize, usize), i32);

/// Used as second argument in a `Iterator<Tree>.fold()` fn
/// Reduces into a tuple containing:
/// - Height of the highest tree in the iterator
/// - Vector of all trees that can be seen from the start of the iterator
fn into_visible_trees(
    (highest, visible_trees): (i32, Vec<Tree>),
    &tree: &Tree,
) -> (i32, Vec<Tree>) {
    if tree.1 > highest {
        let mut trees: Vec<Tree> = visible_trees.clone();
        trees.push(tree);
        (tree.1, trees)
    } else {
        (highest, visible_trees)
    }
}

fn puzzle(input: &str) -> usize {
    let forest = input
        .lines()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(j, tree)| {
                    let tree = tree.to_digit(10)?;
                    Some(Tree((i, j), tree as i32))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let transposed_forest = matrix::transpose(&forest);

    vec![
        forest
            .iter()
            .map(|row| row.iter().fold((-1, vec![]), into_visible_trees).1)
            .flatten()
            .collect::<Vec<_>>(),
        forest
            .iter()
            .map(|row| row.iter().rev().fold((-1, vec![]), into_visible_trees).1)
            .flatten()
            .collect::<Vec<_>>(),
        transposed_forest
            .iter()
            .map(|row| row.iter().fold((-1, vec![]), into_visible_trees).1)
            .flatten()
            .collect::<Vec<_>>(),
        transposed_forest
            .iter()
            .map(|row| row.iter().rev().fold((-1, vec![]), into_visible_trees).1)
            .flatten()
            .collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
    .collect::<HashSet<Tree>>()
    .len()
}

#[cfg(test)]
mod day08a {
    use super::*;

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 21);
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 1688);
    }
}
