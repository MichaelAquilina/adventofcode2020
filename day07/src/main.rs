use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::Read;

type BagMap = HashMap<String, Bag>;

struct Bag {
    parents: Vec<String>,
    children: Vec<(String, usize)>,
}

impl Bag {
    fn new() -> Self {
        Bag {
            parents: vec![],
            children: vec![],
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;

    let bags = parse_bags(&contents)?;

    let count = traverse(&bags, "shiny gold");

    println!("Part 1: {}", count.len());
    println!("Part 2: {}", traverse_2(&bags, "shiny gold"));

    Ok(())
}

fn traverse(bags: &BagMap, value: &str) -> HashSet<String> {
    let mut result = HashSet::new();
    if let Some(bag) = bags.get(value) {
        for parent in &bag.parents {
            result.insert(String::from(parent));

            // merge output from traverse into this result
            for parent in traverse(bags, parent) {
                result.insert(parent);
            }
        }
    }

    result
}

fn traverse_2(bags: &BagMap, value: &str) -> usize {
    let mut result = 0;
    if let Some(bag) = bags.get(value) {
        for (child, count) in &bag.children {
            result += count + (count * traverse_2(bags, child));
        }
    }

    result
}

fn parse_bags(content: &str) -> Result<BagMap, Box<dyn Error>> {
    let mut bags: HashMap<String, Bag> = HashMap::new();

    for line in content.lines() {
        let mut tokens = line.split(" bags contain ");
        let parent_color = tokens.next().ok_or("Missing bag color")?;
        let children = tokens
            .next()
            .ok_or("Missing bag children")?
            .strip_suffix(".")
            .unwrap();

        if children != "no other bags" {
            let mut child_colors = vec![];
            for child in children.split(", ") {
                // pretty hacky, could probably improve this string replace!
                let child = child.replace(" bags", "").replace(" bag", "");
                let mut tokens = child.split(' ');

                let number: usize = tokens.next().ok_or("Missing child number")?.parse()?;
                let remaining_tokens = tokens.collect::<Vec<&str>>();

                let child_color = remaining_tokens.join(" ");
                child_colors.push((child_color, number));
            }

            for (child_color, _) in &child_colors {
                let child_bag = bags
                    .entry(String::from(child_color))
                    .or_insert_with(Bag::new);
                child_bag.parents.push(String::from(parent_color));
            }

            let parent_bag = bags
                .entry(String::from(parent_color))
                .or_insert_with(Bag::new);
            parent_bag.children = child_colors;
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

    #[test]
    fn test_provided_example_part_2() -> Result<(), Box<dyn Error>> {
        let content = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ]
        .join("\n");

        let bags = parse_bags(&content)?;
        let result = traverse_2(&bags, "shiny gold");

        assert_eq!(result, 126);

        Ok(())
    }
}
