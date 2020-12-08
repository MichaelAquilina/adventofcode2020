use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::Read;

type BagMap = HashMap<String, Vec<(String, usize)>>;

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;

    let bags = parse_bags(&contents)?;

    let count = traverse(&bags, "shiny gold");

    println!("Part 1: {}", count.len());

    Ok(())
}

fn traverse(bags: &BagMap, value: &str) -> HashSet<String> {
    let mut result = HashSet::new();
    if let Some(parents) = bags.get(value) {
        for (parent, _) in parents {
            result.insert(String::from(parent));

            // merge output from traverse into this result
            for parent in traverse(bags, &parent) {
                result.insert(parent);
            }
        }
    }

    result
}

fn parse_bags(content: &str) -> Result<BagMap, Box<dyn Error>> {
    let mut bags = HashMap::new();

    for line in content.lines() {
        let mut tokens = line.split(" bags contain ");
        let parent = tokens.next().ok_or("Missing bag color")?;
        let children = tokens
            .next()
            .ok_or("Missing bag children")?
            .strip_suffix(".")
            .unwrap();

        if children != "no other bags" {
            for child in children.split(", ") {
                // pretty hacky, could probably improve this string replace!
                let child = child.replace(" bags", "").replace(" bag", "");
                let mut tokens = child.split(" ");

                let number: usize = tokens.next().ok_or("Missing child number")?.parse()?;
                let remaining_tokens = tokens.collect::<Vec<&str>>();

                let child_color = remaining_tokens.join(" ");

                let parents = bags.entry(child_color).or_insert(Vec::new());
                parents.push((String::from(parent), number));
            }
        }
    }

    Ok(bags)
}

#[cfg(test)]
mod test_traverse {
    use super::*;

    #[test]
    fn test_provided_example_part_1() -> Result<(), Box<dyn Error>> {
        let content = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ]
        .join("\n");

        let bags = parse_bags(&content)?;
        let result = traverse(&bags, "shiny gold");

        assert_eq!(result.len(), 4);

        Ok(())
    }
}
