use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let count = part1(&contents);
    println!("Part 1: {}", count);

    let count = part2(&contents);
    println!("Part 2: {}", count);

    Ok(())
}

fn part1(contents: &str) -> usize {
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

    count
}

fn part2(contents: &str) -> usize {
    let mut count = 0;
    for group in contents.split("\n\n") {
        let mut answers: HashMap<char, u32> = HashMap::new();
        let mut group_size = 0;
        for line in group.lines() {
            group_size += 1;

            for answer in line.chars() {
                let value = answers.entry(answer).or_insert(0);
                *value += 1;
            }
        }

        count += answers.values().filter(|v| *v == &group_size).count();
    }

    count
}
