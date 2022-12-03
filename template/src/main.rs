fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer}");
}

fn puzzle(input: &str) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01a() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, ());
    }
}
