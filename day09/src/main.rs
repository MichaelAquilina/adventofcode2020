use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;

    let result = find_invalid_number(&contents)?;
    println!("Part 1: {:?}", result);

    Ok(())
}

fn find_invalid_number(contents: &str) -> Result<Option<u64>, Box<dyn Error>> {
    let mut numbers: Vec<u64> = vec![0; 25];

    for (index, line) in contents.lines().enumerate() {
        let value = line.parse()?;

        if index >= 25 && !is_valid(&numbers, value) {
            return Ok(Some(value));
        }

        let actual_index = index % 25;
        numbers[actual_index] = value;
    }

    Ok(None)
}

fn is_valid(numbers: &[u64], value: u64) -> bool {
    for n1 in numbers {
        for n2 in numbers {
            if n1 + n2 == value {
                return true;
            }
        }
    }
    false
}
