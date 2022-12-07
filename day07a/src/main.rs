fn main() {
    let input = include_str!("../sample.txt");
    let answer = puzzle(input);
    println!("{answer}");
}

#[derive(Debug)]
enum TerminalLine {
    Input(Command),
    Output(Entry),
}

impl From<&str> for TerminalLine {
    fn from(line: &str) -> Self {
        let mut words = line.split_whitespace();
        match words.next().unwrap() {
            "$" => match words.next().unwrap() {
                "cd" => match words.next().unwrap() {
                    "/" => Self::Input(Command::GotoRoot),
                    ".." => Self::Input(Command::GotoParent),
                    // TODO: Make this `GotoChild` command contain a proper pointer
                    dir_name => Self::Input(Command::GotoChild(0)),
                },
                "ls" => Self::Input(Command::List),
                _ => unreachable!(),
            },
            "dir" => Self::Output(Entry::Directory {
                name: words.next().unwrap().to_string(),
                parent: None,
                children: vec![],
            }),
            file_size => Self::Output(Entry::File {
                name: words.next().unwrap().to_string(),
                size: file_size.parse().unwrap(),
            }),
        }
    }
}

#[derive(Debug)]
enum Command {
    GotoRoot,
    GotoParent,
    GotoChild(usize),
    List,
}

#[derive(Debug)]
enum Entry {
    Directory {
        name: String,
        parent: Option<usize>,
        children: Vec<usize>,
    },
    File {
        name: String,
        size: usize,
    },
}

fn puzzle(input: &str) -> i32 {
    let terminal_lines = [TerminalLine::Output(Entry::Directory {
        name: "/".to_string(),
        parent: None,
        children: vec![],
    })]
    .into_iter()
    .chain(input.lines().map(|line| line.into()))
    .collect::<Vec<_>>();

    dbg!(terminal_lines);

    0
}

#[cfg(test)]
mod day07a {
    use super::*;

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, 95437);
    }

    // #[test]
    // fn works_with_puzzle_input() {
    //     let input = include_str!("../input.txt");
    //     let answer = puzzle(input);
    //     assert_eq!(answer, 0);
    // }
}
