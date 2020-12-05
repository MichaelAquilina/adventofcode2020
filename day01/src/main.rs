use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let mut values: Vec<i32> = vec![];

    for line in contents.lines() {
        let value = line.parse()?;
        values.push(value);
    }

    if let Some((v1, v2)) = find_result_pairs(&values) {
        println!("Part 1: {} * {} = {}", v1, v2, v1 * v2);
    } else {
        println!("Unable to find result for part 1!");
    }

    if let Some((v1, v2, v3)) = find_result_triples(&values) {
        println!("Part 2: {} * {} * {} = {}", v1, v2, v3, v1 * v2 * v3);
    } else {
        println!("Unable to find result for part 2!");
    }

    Ok(())
}

fn find_result_pairs(values: &[i32]) -> Option<(i32, i32)> {
    for v1 in values {
        for v2 in values {
            if v1 + v2 == 2020 {
                return Some((*v1, *v2));
            }
        }
    }
    None
}

fn find_result_triples(values: &[i32]) -> Option<(i32, i32, i32)> {
    for v1 in values {
        for v2 in values {
            for v3 in values {
                if v1 + v2 + v3 == 2020 {
                    return Some((*v1, *v2, *v3));
                }
            }
        }
    }
    None
}
