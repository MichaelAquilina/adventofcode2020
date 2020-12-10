use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum InstructionError {
    #[error("unknown operation: {0}")]
    UnknownOperation(String),
    #[error("invalid instruction location: {0}")]
    InvalidInstructionLocation(i32),
}

#[derive(Debug)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug)]
pub struct Instruction {
    operation: Operation,
    argument: i32,
}

pub fn run_instructions(instructions: &[Instruction]) -> Result<i32, InstructionError> {
    let mut accumulator = 0;
    let mut index = 0;

    let mut visited: HashSet<usize> = HashSet::new();

    loop {
        if visited.contains(&index) {
            break;
        }

        let instruction = &instructions[index];
        visited.insert(index);

        match instruction.operation {
            Operation::Acc => accumulator += instruction.argument,
            Operation::Jmp => {
                let new_index = index as i32 + instruction.argument;
                if new_index < 0 {
                    return Err(InstructionError::InvalidInstructionLocation(new_index));
                }

                index = new_index as usize;
                continue;
            }
            Operation::Nop => {}
        }
        index += 1;
    }

    Ok(accumulator)
}

impl std::str::FromStr for Operation {
    type Err = InstructionError;

    fn from_str(content: &str) -> Result<Operation, Self::Err> {
        match content {
            "nop" => Ok(Operation::Nop),
            "jmp" => Ok(Operation::Jmp),
            "acc" => Ok(Operation::Acc),
            other => Err(InstructionError::UnknownOperation(other.to_string())),
        }
    }
}

impl std::str::FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(content: &str) -> Result<Instruction, Self::Err> {
        let mut tokens = content.split(" ");
        let operation = tokens.next().ok_or("Missing operation")?.parse()?;
        let argument = tokens.next().ok_or("Missing argument")?.parse()?;
        Ok(Instruction {
            operation,
            argument,
        })
    }
}

#[cfg(test)]
mod test_instruction {
    use super::*;

    #[test]
    fn test_provided_example_1() -> Result<(), Box<dyn Error>> {
        let instructions: Vec<Instruction> = vec![
            "nop +0".parse()?,
            "acc +1".parse()?,
            "jmp +4".parse()?,
            "acc +3".parse()?,
            "jmp -3".parse()?,
            "acc -99".parse()?,
            "acc +1".parse()?,
            "jmp -4".parse()?,
            "acc +6".parse()?,
        ];

        let result = run_instructions(&instructions)?;
        assert_eq!(result, 5);

        Ok(())
    }
}
