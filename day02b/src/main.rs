fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer}");
}

fn calculate_score(weapons: (&str, &str)) -> i32 {
    match weapons {
        ("A", "X") => 3,
        ("A", "Y") => 4,
        ("A", "Z") => 8,

        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,

        ("C", "X") => 2,
        ("C", "Y") => 6,
        ("C", "Z") => 7,
        _ => unreachable!(),
    }
}

fn puzzle(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| line.split_once(' ').map(calculate_score))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02a() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 12);
    }
}
