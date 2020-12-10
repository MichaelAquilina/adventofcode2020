use std::error::Error;

pub struct PolicyEntry {
    min_frequency: usize,
    max_frequency: usize,
    character: char,
    password: String,
}

impl PolicyEntry {
    pub fn is_valid_part1(&self) -> bool {
        let count = self.password.matches(self.character).count();

        count >= self.min_frequency && count <= self.max_frequency
    }

    pub fn is_valid_part2(&self) -> bool {
        // indexes start from 1 in the policy NOT 0
        let position1 = self.password.chars().nth(self.min_frequency - 1) == Some(self.character);
        let position2 = self.password.chars().nth(self.max_frequency - 1) == Some(self.character);

        position1 ^ position2
    }
}

impl std::str::FromStr for PolicyEntry {
    type Err = Box<dyn Error>;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut tokens = line.split(": ");

        let policy = tokens
            .next()
            .ok_or_else(|| String::from("Missing policy"))?;
        let password = String::from(
            tokens
                .next()
                .ok_or_else(|| String::from("Missing password"))?,
        );

        let mut tokens = policy.split(' ');
        let frequency = tokens
            .next()
            .ok_or_else(|| String::from("Missing frequency"))?;
        let character: char = tokens
            .next()
            .ok_or_else(|| String::from("Missing character"))?
            .parse()?;

        let mut tokens = frequency.split('-');
        let min_frequency: usize = tokens
            .next()
            .ok_or_else(|| String::from("Missing min frequency"))?
            .parse()?;
        let max_frequency: usize = tokens
            .next()
            .ok_or_else(|| String::from("Missing max frequency"))?
            .parse()?;

        Ok(PolicyEntry {
            min_frequency,
            max_frequency,
            character,
            password,
        })
    }
}

#[cfg(test)]
mod test_policy_2 {
    use super::*;
    use rstest::*;

    #[rstest(
        line,
        valid,
        case("1-3 a: abcde", true),
        case("1-3 b: cdefg", false),
        case("2-9 c: ccccccccc", false)
    )]
    fn test_provided_examples(line: &str, valid: bool) -> Result<(), Box<dyn Error>> {
        let policy: PolicyEntry = line.parse()?;
        assert_eq!(policy.is_valid_part2(), valid);

        Ok(())
    }
}
