mod types;

use types::{Command, Entry, Node, TerminalLine};

fn main() {
    let input = include_str!("../input.txt");
    let answer = puzzle(input);
    println!("{answer:?}");
}

fn puzzle(input: &str) -> Result<usize, String> {
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

    let disk_capacity = 70_000_000;
    let disk_usage = memory[0].total_size(&memory);
    let free_disk_space = disk_capacity - disk_usage;
    let update_size = 30_000_000;

    let required_space_for_update = update_size - free_disk_space;

    let mut node_sizes = memory
        .iter()
        .map(|node| node.total_size(&memory))
        .collect::<Vec<_>>();

    node_sizes.sort_unstable();

    Ok(node_sizes
        .iter()
        .filter(|&&size| size > required_space_for_update)
        .copied()
        .next()
        .ok_or("No single directory big enough to make room by deleting")?)
}

#[cfg(test)]
mod day07b {
    use super::*;

    #[test]
    fn works_with_sample_input() {
        let input = include_str!("../sample.txt");
        let answer = puzzle(input);
        assert_eq!(answer, Ok(24_933_642));
    }

    #[test]
    fn works_with_puzzle_input() {
        let input = include_str!("../input.txt");
        let answer = puzzle(input);
        assert_eq!(answer, Ok(549_173));
    }
}
