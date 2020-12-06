use anyhow::Context;
use std::error::Error;
use std::io::Read;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let mut seat_ids = vec![];

    for entry in contents.lines() {
        let (row, column) = parse_boarding_pass(entry).context(String::from(entry))?;

        let seat_id = (row * 8) + column;
        seat_ids.push(seat_id);
    }

    println!("Part 1: {:?}", seat_ids.iter().max());

    Ok(())
}

fn parse_boarding_pass(entry: &str) -> Result<(u32, u32), ParseIntError> {
    // the solution is essentially a simple binary encoding
    // where B=1 and F=0 for the rows and R=1 and L=0 for the columns

    let row_encoding = entry[0..7].replace("B", "1").replace("F", "0");
    let column_encoding = entry[7..10].replace("R", "1").replace("L", "0");

    let row = u32::from_str_radix(&row_encoding, 2)?;
    let column = u32::from_str_radix(&column_encoding, 2)?;

    Ok((row, column))
}
