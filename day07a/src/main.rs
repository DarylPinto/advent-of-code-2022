mod types;

use types::{Command, Entry, Node, TerminalLine};

fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer:?}");
}

fn puzzle(input: &str) -> Result<i32, String> {
    let lines = input
        .lines()
        .filter_map(|line| TerminalLine::try_from(line).ok());

    // Virtual system memory containing directory nodes. Each node can be
    // accessed using a pointer (index) to it's location
    let mut memory = vec![Node::new("/", None)];

    // Pointer to the current working directory. Starts at the root
    let mut cwd_pointer = 0;

    // Walk through the lines in the puzzle input, populate virtual system
    // memory with nodes and their respective pointers to each other
    for line in lines {
        match line {
            TerminalLine::Input(command) => match command {
                Command::GotoRoot => cwd_pointer = 0,
                Command::GotoParent => {
                    cwd_pointer = memory[cwd_pointer]
                        .parent
                        .ok_or("tried to `cd ..` in a directory with no parent")?
                }
                Command::GotoChild(child_name) => {
                    cwd_pointer = memory[cwd_pointer]
                        .children
                        .iter()
                        .find(|(_, name)| name == &child_name)
                        .ok_or("tried to `cd` into non-existent child directory")?
                        .0;
                }
                Command::List => (),
            },
            TerminalLine::Output(entry) => match entry {
                Entry::Directory(name) => {
                    let dir_pointer = memory.len();
                    memory.push(Node::new(&name, Some(cwd_pointer)));
                    memory[cwd_pointer].children.push((dir_pointer, name));
                }
                Entry::File(file_size, _) => {
                    memory[cwd_pointer].size += file_size;
                }
            },
        };
    }

    Ok(memory
        .iter()
        .map(|node| node.total_size(&memory) as i32)
        .filter(|&size| size < 100_000)
        .sum())
}

#[cfg(test)]
mod day07a {
    use super::*;

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, Ok(95_437));
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, Ok(1_367_870));
    }
}
