fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer}");
}

fn puzzle(input: &str) -> i32 {
    let calorie_list = input
        .lines()
        .map(|line| line.parse::<i32>().ok())
        .collect::<Vec<_>>();

    let calorie_sum_list = calorie_list
        .split(|item| item.is_none())
        .map(|x| x.iter().filter_map(|&y| y).sum::<i32>())
        .collect::<Vec<_>>();

    calorie_sum_list.into_iter().max().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01a() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 24000);
    }
}
