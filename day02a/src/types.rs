pub enum Weapon {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<&str> for Weapon {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err("unable to convert &str to Weapon"),
        }
    }
}
