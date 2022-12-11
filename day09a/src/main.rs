mod types;
use std::collections::HashSet;
use types::{
    Direction::{self, *},
    Knot,
};

fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer}");
}

/// Create a knot in a position 1 away from `head` in the given `direction`
fn move_head(head: &Knot, direction: &Direction) -> Knot {
    let Knot(x, y) = *head;
    match direction {
        Up => Knot(x, y + 1),
        Down => Knot(x, y - 1),
        Left => Knot(x - 1, y),
        Right => Knot(x + 1, y),
    }
}

/// Create a knot in the correct tail position for a given `head` and `tail`
/// that are too far apart
fn move_tail(tail: &Knot, head: &Knot) -> Knot {
    let x_abs_diff = head.0.abs_diff(tail.0);
    let y_abs_diff = head.1.abs_diff(tail.1);

    if x_abs_diff <= 1 && y_abs_diff <= 1 {
        return tail.clone();
    }

    let x_diff = head.0 - tail.0;
    let y_diff = head.1 - tail.1;

    // True if the horizontal distance between the head and tail is greater
    // than the vertical distance between the head and the tail
    let is_tail_offset_horizontally = x_abs_diff > y_abs_diff;

    let is_tail_above_head = y_diff < 0;
    let is_tail_right_of_head = x_diff < 0;

    match is_tail_offset_horizontally {
        true => match is_tail_right_of_head {
            true => Knot(head.0 + 1, head.1),
            false => Knot(head.0 - 1, head.1),
        },
        false => match is_tail_above_head {
            true => Knot(head.0, head.1 + 1),
            false => Knot(head.0, head.1 - 1),
        },
    }
}

fn puzzle(input: &str) -> usize {
    let visited = input
        .lines()
        .filter_map(|line| {
            let (direction, amount) = line.split_once(' ')?;
            let direction: Direction = direction.try_into().ok()?;
            let amount = amount.parse::<i32>().ok()?;
            Some((direction, amount))
        })
        .fold(
            vec![(Knot::default(), Knot::default())],
            |visited, (direction, amount)| {
                let mut visited = visited.clone();

                (0..amount).for_each(|_| {
                    let (head, tail) = visited.last().unwrap();
                    let next_head = move_head(&head, &direction);
                    let next_tail = move_tail(&tail, &next_head);
                    visited.push((next_head, next_tail));
                });

                visited
            },
        );

    visited
        .iter()
        .map(|(_, tail)| tail)
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod day09a {
    use super::*;

    mod move_tail {
        use super::*;
        #[test]
        fn works_when_head_is_above_tail() {
            let head = Knot(5, 10);
            let tail = Knot(0, 0);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(5, 9));
        }

        #[test]
        fn works_when_head_is_below_tail() {
            let head = Knot(3, 0);
            let tail = Knot(1, 10);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(3, 1));
        }

        #[test]
        fn works_when_head_is_to_the_right_of_tail() {
            let head = Knot(8, 1);
            let tail = Knot(0, 0);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(7, 1))
        }

        #[test]
        fn works_when_head_is_to_the_left_of_tail() {
            let head = Knot(0, 0);
            let tail = Knot(8, 1);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(1, 0))
        }

        #[test]
        fn works_when_head_and_tail_are_on_the_same_space() {
            let head = Knot(5, 5);
            let tail = Knot(5, 5);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(5, 5));
        }

        #[test]
        fn works_when_head_is_one_space_away() {
            let head = Knot(5, 6);
            let tail = Knot(5, 5);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(5, 5));

            let head = Knot(6, 5);
            let tail = Knot(5, 5);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(5, 5));

            let head = Knot(4, 5);
            let tail = Knot(5, 5);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(5, 5));

            let head = Knot(5, 4);
            let tail = Knot(5, 5);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(5, 5));
        }
    }

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 13);
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 6087);
    }
}
