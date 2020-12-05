mod policy_entry;

use std::error::Error;
use std::io::Read;

use policy_entry::PolicyEntry;

fn main() -> Result<(), Box<dyn Error>> {
    let mut content = String::new();

    std::io::stdin().read_to_string(&mut content)?;

    let entries: Vec<PolicyEntry> = content
        .lines()
        .into_iter()
        .map(|l| l.parse().unwrap())
        .collect();

    let valid_1 = entries.iter().filter(|e| e.is_valid_part1()).count();
    let valid_2 = entries.iter().filter(|e| e.is_valid_part2()).count();

    println!("Part 1: {}", valid_1);
    println!("Part 2: {}", valid_2);

    Ok(())
}
