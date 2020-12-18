use std::error::Error;
use std::io::Read;

struct JoltResult {
    differences_1: Vec<u32>,
    differences_3: Vec<u32>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let mut adapters: Vec<u32> = vec![];
    for line in contents.lines() {
        adapters.push(line.parse()?);
    }

    let result = get_jolt_result(&mut adapters);

    let part_1 = result.differences_1.len() * result.differences_3.len();
    println!("Part 1: {}", part_1);

    Ok(())
}

fn get_jolt_result(adapters: &mut Vec<u32>) -> JoltResult {
    let inbuilt = adapters.iter().max().unwrap() + 3;

    let mut differences_3 = vec![inbuilt];
    let mut differences_1 = vec![];

    let mut outlet = 0;

    while !adapters.is_empty() {
        let mut min_joltage = None;
        let mut min_index = None;
        for (index, joltage) in adapters.iter().enumerate() {
            if min_joltage.is_none() || joltage < min_joltage.unwrap() {
                min_joltage = Some(joltage);
                min_index = Some(index);
            }
        }

        if let Some(index) = min_index {
            let joltage = adapters[index];

            let jolt_difference = joltage - outlet;

            match jolt_difference {
                3 => differences_3.push(joltage),
                2 => {}
                1 => differences_1.push(joltage),
                0 => {}
                _ => panic!("Found incorrect joltage difference of {}", jolt_difference),
            }

            outlet = joltage;
            adapters.remove(index);
        } else {
            panic!("Unable to find match");
        }
    }

    JoltResult {
        differences_3,
        differences_1,
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        let result = get_jolt_result(&mut adapters);
        assert_eq!(result.differences_3.len(), 5);
        assert_eq!(result.differences_1.len(), 7);
    }

    #[test]
    fn test_example_2() {
        let mut adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        let result = get_jolt_result(&mut adapters);
        assert_eq!(result.differences_3.len(), 10);
        assert_eq!(result.differences_1.len(), 22);
    }
}
