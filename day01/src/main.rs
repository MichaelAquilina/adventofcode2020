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

    if let Some((v1, v2)) = find_result(&values) {
        println!("{} * {} = {}", v1, v2, v1 * v2);
    } else {
        println!("Unable to find result!");
    }

    Ok(())
}

fn find_result(values: &[i32]) -> Option<(i32, i32)> {
    for v1 in values {
        for v2 in values {
            if v1 + v2 == 2020 {
                return Some((*v1, *v2));
            }
        }
    }
    None
}
