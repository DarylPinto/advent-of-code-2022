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
        return *tail;
    }

    let x_diff = head.0 - tail.0;
    let y_diff = head.1 - tail.1;

    // True if the horizontal distance between the head and tail is greater
    // than the vertical distance between the head and the tail
    let is_tail_offset_horizontally = x_abs_diff > y_abs_diff;

    // True if the tail is equadistant from the head in both the x and the y
    // axis
    let is_tail_offset_diagonally = x_abs_diff == y_abs_diff;

    let is_tail_above_head = y_diff < 0;
    let is_tail_right_of_head = x_diff < 0;

    if is_tail_offset_diagonally {
        if !is_tail_right_of_head && is_tail_above_head {
            Knot(head.0 - 1, head.1 + 1)
        } else if is_tail_right_of_head && is_tail_above_head {
            Knot(head.0 + 1, head.1 + 1)
        } else if !is_tail_right_of_head && !is_tail_above_head {
            Knot(head.0 - 1, head.1 - 1)
        } else {
            Knot(head.0 + 1, head.1 - 1)
        }
    } else if is_tail_offset_horizontally {
        if is_tail_right_of_head {
            Knot(head.0 + 1, head.1)
        } else {
            Knot(head.0 - 1, head.1)
        }
    } else if is_tail_above_head {
        Knot(head.0, head.1 + 1)
    } else {
        Knot(head.0, head.1 - 1)
    }
}

fn puzzle(input: &str) -> usize {
    let visited = input
        .lines()
        .filter_map(|line| {
            let (direction, amount) = line.split_once(' ')?;
            let direction: Direction = direction.try_into().ok()?;
            let amount = amount.parse::<i64>().ok()?;
            Some((direction, amount))
        })
        .fold(
            vec![[Knot::default(); 10]],
            |visited, (direction, amount)| {
                let mut visited = visited;

                (0..amount).for_each(|_| {
                    if let Some(rope) = visited.last() {
                        let mut rope = *rope;
                        rope[0] = move_head(&rope[0], &direction);
                        for i in 1..rope.len() {
                            rope[i] = move_tail(&rope[i], &rope[i - 1]);
                        }
                        visited.push(rope);
                    }
                });

                visited
            },
        );

    visited
        .iter()
        .filter_map(|rope| rope.last())
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod day09b {
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

            let head = Knot(6, 6);
            let tail = Knot(5, 5);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(5, 5));

            let head = Knot(4, 4);
            let tail = Knot(5, 5);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(5, 5));

            let head = Knot(4, 6);
            let tail = Knot(5, 5);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(5, 5));

            let head = Knot(6, 4);
            let tail = Knot(5, 5);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(5, 5));
        }

        #[test]
        fn works_when_tail_is_diagonally_offset() {
            let head = Knot(0, 0);
            let tail = Knot(-5, -5);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(-1, -1));

            let head = Knot(0, 0);
            let tail = Knot(5, 5);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(1, 1));

            let head = Knot(0, 0);
            let tail = Knot(-5, 5);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(-1, 1));

            let head = Knot(0, 0);
            let tail = Knot(5, -5);
            let new_tail = move_tail(&tail, &head);
            assert_eq!(new_tail, Knot(1, -1));
        }
    }

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 36);
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 2493);
    }
}
