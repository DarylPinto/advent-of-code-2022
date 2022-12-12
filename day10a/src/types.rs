#[derive(Clone, Debug)]
pub struct Cpu(pub i32);

pub struct CpuHistory(pub Vec<Cpu>);

impl CpuHistory {
    pub fn execute(&self, instruction: Instruction) -> Self {
        let mut history = self.0.clone();
        let cpu = history.last().unwrap().clone();
        match instruction {
            Instruction::NoOp => history.push(cpu),
            Instruction::AddX(value) => {
                history.push(cpu.clone());
                history.push(Cpu(cpu.0 + value))
            }
        }
        Self(history)
    }
}

pub enum Instruction {
    NoOp,
    AddX(i32),
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, &'static str> {
        let mut words = line.split_whitespace();
        match words.next().ok_or("empty string")? {
            "noop" => Ok(Self::NoOp),
            "addx" => {
                let value = words
                    .next()
                    .ok_or("no value for addx instruction")?
                    .parse::<i32>()
                    .map_err(|_| "invalid value for addx instruction")?;

                Ok(Self::AddX(value))
            }
            _ => unreachable!(),
        }
    }
}
