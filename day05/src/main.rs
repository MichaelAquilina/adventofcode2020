use anyhow::Context;
use std::error::Error;
use std::io::Read;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let mut seats = vec![];

    for entry in contents.lines() {
        let (row, column) = parse_boarding_pass(entry).context(String::from(entry))?;

        let seat_id = get_seat_id(row, column);
        seats.push(seat_id);
    }
    seats.sort_unstable();

    println!("Part 1: {}", seats.last().ok_or("No data")?);

    let missing = find_missing(&seats).ok_or("No data")?;

    println!("Part 2: {}", missing);

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

fn get_seat_id(row: u32, column: u32) -> u32 {
    (row * 8) + column
}

// TODO: There must be a more elegant way to do this
fn find_missing(seats: &[u32]) -> Option<u32> {
    if let Some(first) = seats.first() {
        let mut current = *first;
        for seat in seats {
            if seat != &current {
                return Some(current);
            }

            current += 1;
        }
    }
    None
}
