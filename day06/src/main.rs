use std::collections::HashSet;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let mut count = 0;

    for group in contents.split("\n\n") {
        let mut answers = HashSet::new();
        for line in group.lines() {
            for answer in line.chars() {
                answers.insert(answer);
            }
        }
        count += answers.len();
    }

    println!("Part 1: {}", count);

    Ok(())
}
