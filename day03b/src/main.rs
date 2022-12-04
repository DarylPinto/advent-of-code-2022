fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer}");
}

fn to_priority(letter: char) -> usize {
    "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .as_bytes()
        .iter()
        .position(|&byte| letter as u8 == byte)
        .unwrap_or(0)
}

fn puzzle(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .filter_map(|slices| {
            slices[0]
                .chars()
                .filter(|&char| slices[1].contains(char) && slices[2].contains(char))
                .into_iter()
                .next()
        })
        .map(to_priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03b() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 70);
    }
}
