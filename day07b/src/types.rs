/// Represents a single line on the terminal screen
#[derive(Debug, Clone)]
pub enum TerminalLine {
    Input(Command),
    Output(Entry),
}

/// Commands you can execute
#[derive(Debug, Clone)]
pub enum Command {
    GotoRoot,
    GotoParent,
    GotoChild(String),
    List,
}

/// Output printed by the system
#[derive(Debug, Clone)]
pub enum Entry {
    Directory(String),
    File(usize, String),
}

impl TryFrom<&str> for TerminalLine {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, &'static str> {
        let mut words = line.split_whitespace();
        match words.next().ok_or("empty string")? {
            "$" => match words.next().ok_or("no command following '$'")? {
                "cd" => match words.next().ok_or("no directory following 'cd'")? {
                    "/" => Ok(Self::Input(Command::GotoRoot)),
                    ".." => Ok(Self::Input(Command::GotoParent)),
                    dir_name => Ok(Self::Input(Command::GotoChild(dir_name.into()))),
                },
                "ls" => Ok(Self::Input(Command::List)),
                _ => Err("unknown shell command")?,
            },
            "dir" => Ok(Self::Output(Entry::Directory(
                words
                    .next()
                    .ok_or("no directory name following 'dir'")?
                    .to_string(),
            ))),
            file_size => Ok(Self::Output(Entry::File(
                file_size
                    .parse()
                    .map_err(|_| "unable to parse first word as number (file size)")?,
                words
                    .next()
                    .ok_or("no file name following file size")?
                    .to_string(),
            ))),
        }
    }
}

/// Node represents a directory in the file tree.
/// References to other nodes are `usize`s that represent pointers into the
/// virtual system's memory
#[derive(Debug)]
pub struct Node {
    _name: String,
    pub size: usize,
    pub parent: Option<usize>,
    // Each child is a tuple containing: (address, label)
    pub children: Vec<(usize, String)>,
}

impl Node {
    pub fn new(name: &str, parent: Option<usize>) -> Self {
        Self {
            _name: name.to_string(),
            size: 0,
            parent,
            children: vec![],
        }
    }

    /// Calculates the total size of a directory and all of it's children
    pub fn total_size(&self, memory: &[Node]) -> usize {
        let children_size = self
            .children
            .iter()
            .map(|(address, _)| {
                let child = &memory[address.clone()];
                child.total_size(&memory)
            })
            .sum::<usize>();

        self.size + children_size
    }
}
