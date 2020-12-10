use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum InstructionError {
    #[error("unknown operation: {0}")]
    UnknownOperation(String),
    #[error("invalid instruction location: {0}")]
    InvalidInstructionLocation(i32),
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub operation: Operation,
    pub argument: i32,
}

#[derive(Debug, PartialEq)]
pub enum TerminationReason {
    InfiniteLoop,
    ReachedEnd,
}

#[derive(Debug, PartialEq)]
pub struct InstructionResult {
    pub accumulator: i32,
    pub termination_reason: TerminationReason,
}

pub fn run_instructions(
    instructions: &[Instruction],
) -> Result<InstructionResult, InstructionError> {
    let mut accumulator = 0;
    let mut index = 0;

    let mut visited: HashSet<usize> = HashSet::new();

    while index < instructions.len() {
        if visited.contains(&index) {
            return Ok(InstructionResult {
                accumulator,
                termination_reason: TerminationReason::InfiniteLoop,
            });
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

    Ok(InstructionResult {
        accumulator,
        termination_reason: TerminationReason::ReachedEnd,
    })
}

pub fn fix_instructions(instructions: &[Instruction]) -> Result<InstructionResult, Box<dyn Error>> {
    let mut instructions_fix = instructions.to_vec();

    let mut index = 0;
    while index < instructions.len() {
        match instructions[index].operation {
            Operation::Acc => {
                index += 1;
                continue;
            }
            Operation::Jmp => instructions_fix[index].operation = Operation::Nop,
            Operation::Nop => instructions_fix[index].operation = Operation::Jmp,
        };

        let result = run_instructions(&instructions_fix)?;

        if result.termination_reason == TerminationReason::ReachedEnd {
            return Ok(result);
        }

        // restore operation to original value sinc it did not work
        instructions_fix[index].operation = instructions[index].operation;
        index += 1;
    }

    panic!("Oh oh");
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
        let mut tokens = content.split(' ');
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
        assert_eq!(
            result,
            InstructionResult {
                accumulator: 5,
                termination_reason: TerminationReason::InfiniteLoop
            }
        );

        Ok(())
    }

    #[test]
    fn test_provided_example_2() -> Result<(), Box<dyn Error>> {
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

        let result = fix_instructions(&instructions)?;
        assert_eq!(
            result,
            InstructionResult {
                accumulator: 8,
                termination_reason: TerminationReason::ReachedEnd,
            }
        );

        Ok(())
    }
}
