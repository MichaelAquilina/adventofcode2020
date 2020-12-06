mod passport;

use std::error::Error;
use std::io::Read;

use passport::Passport;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let mut count = 0;
    for entry in contents.split("\n\n") {
        match entry.parse::<Passport>() {
            Ok(_) => {
                count += 1;
            }
            Err(_) => continue,
        }
    }

    println!("Part 2: {}", count);

    Ok(())
}
