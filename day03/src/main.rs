use std::error::Error;
use std::io::Read;

struct Slope {
    x: usize,
    y: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let map: Vec<Vec<char>> = contents.lines().map(|s| s.chars().collect()).collect();

    let count = traverse_slope(&map, Slope { x: 3, y: 1 });

    println!("Part 1: {}", count);

    let total = &[
        traverse_slope(&map, Slope { x: 1, y: 1 }),
        traverse_slope(&map, Slope { x: 3, y: 1 }),
        traverse_slope(&map, Slope { x: 5, y: 1 }),
        traverse_slope(&map, Slope { x: 7, y: 1 }),
        traverse_slope(&map, Slope { x: 1, y: 2 }),
    ];

    println!("Part 2: {}", total.iter().product::<usize>());

    Ok(())
}

fn traverse_slope(map: &[Vec<char>], slope: Slope) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;

    loop {
        x += slope.x;
        y += slope.y;

        if y >= map.len() {
            return count;
        }

        let actual_x = x % map[y].len();

        if map[y][actual_x] == '#' {
            count += 1;
        }
    }
}
