use std::collections::HashMap;
use utils::matrix;
use Direction::*;

fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer:?}");
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Tree((usize, usize), i32);

enum Direction {
    Forward,
    Backward,
}

// For a single tree slice, update each tree's scenic score to either
// the right (Forward), or the left (Backward)
fn update_scenic_scores(
    trees: &[Tree],
    direction: Direction,
    scenic_scores: &mut HashMap<Tree, usize>,
) {
    let trees = trees.to_vec();
    let trees_rev = trees.iter().cloned().rev().collect::<Vec<_>>();

    let tree_iter = || match direction {
        Forward => trees.iter(),
        Backward => trees_rev.iter(),
    };

    tree_iter().enumerate().for_each(|(start_idx, &tree)| {
        let stop_idx = tree_iter()
            .enumerate()
            .skip(start_idx)
            .find(|(next_idx, next)| next.1 >= tree.1 && next_idx > &start_idx)
            .map(|(next_idx, _)| next_idx);

        let view_distance = match stop_idx {
            Some(stop_idx) => trees[start_idx + 1..=stop_idx].len(),
            None => trees[start_idx + 1..].len(),
        };

        scenic_scores
            .entry(tree)
            .and_modify(|distance| *distance *= view_distance)
            .or_insert(view_distance);
    });
}

fn puzzle(input: &str) -> Option<usize> {
    let forest = input
        .lines()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(j, char)| Some(Tree((i, j), char.to_digit(10)? as i32)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let forest_transpose = matrix::transpose(&forest);

    let mut scenic_scores: HashMap<Tree, usize> = HashMap::new();

    forest.iter().for_each(|row| {
        update_scenic_scores(row, Forward, &mut scenic_scores);
        update_scenic_scores(row, Backward, &mut scenic_scores);
    });

    forest_transpose.iter().for_each(|row| {
        update_scenic_scores(row, Forward, &mut scenic_scores);
        update_scenic_scores(row, Backward, &mut scenic_scores);
    });

    scenic_scores.into_values().max()
}

#[cfg(test)]
mod day08a {
    use super::*;

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, Some(8));
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, Some(410400));
    }
}
