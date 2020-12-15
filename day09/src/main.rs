use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;

    let mut numbers: Vec<u64> = vec![];
    for line in contents.lines() {
        numbers.push(line.parse()?);
    }

    let invalid_number =
        find_invalid_number(&numbers, 25)?.ok_or("Could not find invalid number")?;
    println!("Part 1: {}", invalid_number);

    let weakness =
        find_encryption_weakness(&numbers, invalid_number).ok_or("Could not find weakness")?;
    println!("Part 2: {}", weakness);

    Ok(())
}

fn find_invalid_number(numbers: &[u64], preamble: usize) -> Result<Option<u64>, Box<dyn Error>> {
    let mut buffer: Vec<u64> = vec![0; preamble];

    for (index, &value) in numbers.iter().enumerate() {
        if index >= preamble && !is_valid(&buffer, value) {
            return Ok(Some(value));
        }

        let actual_index = index % preamble;
        buffer[actual_index] = value;
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

fn find_encryption_weakness(numbers: &[u64], invalid_number: u64) -> Option<u64> {
    let mut index = 0;
    while index < numbers.len() {
        if let Some(buffer) = match_sum(&numbers[index..], invalid_number) {
            let min_value = buffer.iter().min().unwrap_or(&0);
            let max_value = buffer.iter().max().unwrap_or(&0);

            return Some(min_value + max_value);
        }

        index += 1;
    }

    None
}

fn match_sum(numbers: &[u64], target: u64) -> Option<Vec<u64>> {
    let mut buffer: Vec<u64> = vec![];
    for &value in numbers {
        buffer.push(value);

        let sum: u64 = buffer.iter().sum();

        return match sum.cmp(&target) {
            std::cmp::Ordering::Equal => Some(buffer),
            std::cmp::Ordering::Greater => None,
            std::cmp::Ordering::Less => continue,
        };
    }

    None
}

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn test_provided_example_1() -> Result<(), Box<dyn Error>> {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let invalid_number =
            find_invalid_number(&numbers, 5)?.ok_or("could not find invalid number")?;
        assert_eq!(invalid_number, 127);

        let weakness = find_encryption_weakness(&numbers, invalid_number);

        assert_eq!(weakness, Some(62));

        Ok(())
    }
}
