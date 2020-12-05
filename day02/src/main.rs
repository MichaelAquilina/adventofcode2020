use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut content = String::new();

    std::io::stdin().read_to_string(&mut content)?;

    let valid_passwords = get_valid_passwords(&content)?;

    println!("{} valid passwords found", valid_passwords.len());

    Ok(())
}

fn get_valid_passwords(content: &str) -> Result<Vec<&str>, Box<dyn Error>> {
    let mut valid_passwords = vec![];

    for line in content.lines() {
        let (min_frequency, max_frequency, character, password) = parse_line(line)?;

        let count = password.matches(character).count();

        if count >= min_frequency && count <= max_frequency {
            valid_passwords.push(password);
        }
    }

    Ok(valid_passwords)
}

fn parse_line(line: &str) -> Result<(usize, usize, &str, &str), Box<dyn Error>> {
    let mut tokens = line.split(": ");

    let policy = tokens.next().ok_or(String::from("Missing policy"))?;
    let password = tokens.next().ok_or(String::from("Missing password"))?;

    let mut tokens = policy.split(" ");
    let frequency = tokens.next().ok_or(String::from("Missing frequency"))?;
    let character = tokens.next().ok_or(String::from("Missing character"))?;

    let mut tokens = frequency.split("-");
    let min_frequency: usize = tokens
        .next()
        .ok_or(String::from("Missing min frequency"))?
        .parse()?;
    let max_frequency: usize = tokens
        .next()
        .ok_or(String::from("Missing max frequency"))?
        .parse()?;

    Ok((min_frequency, max_frequency, character, password))
}
