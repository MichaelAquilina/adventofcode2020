mod instruction;

use std::error::Error;
use std::io::Read;

use instruction::{fix_instructions, run_instructions, Instruction};

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;

    // TODO: how to change this to use an interator?
    let mut instructions: Vec<Instruction> = vec![];
    for line in contents.lines() {
        let instruction = line.parse()?;
        instructions.push(instruction);
    }

    let result = run_instructions(&instructions)?;
    println!("Part 1: {}", result.accumulator);

    let result = fix_instructions(&instructions)?;
    println!("Part 2: {}", result.accumulator);

    Ok(())
}
