use std::io::{BufReader, BufRead};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Kind {
    Lowercase,
    Uppercase,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Item {
    kind: Kind,
    priority: u32,
}

impl Item {
    pub fn new(kind: Kind, priority: u32) -> Self {
        Item { kind, priority }
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn priority(&self) -> u32 {
        self.priority
    }
}

impl TryFrom<char> for Item {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let (kind, priority) = if c.is_uppercase() {
            let kind = Kind::Uppercase;
            let priority = c as u32 - 'A' as u32 + 27;
            (kind, priority)
        } else if c.is_lowercase() {
            let kind = Kind::Lowercase;
            let priority = c as u32 - 'a' as u32 + 1;
            (kind, priority)
        } else {
            return Err(format!("Character {} is not valid", c));
        };
        Ok(Self::new(kind, priority))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("data.txt").expect("Could not open file");
    let reader = BufReader::new(f);
    let mut tot_p1_priority = 0;
    let mut tot_p2_priority = 0;
    let mut group_items: HashSet<Item> = HashSet::new();
    let mut last_3_elves = VecDeque::new();
    for (i, line) in reader.lines().enumerate() {
        let items: Vec<char> = line?.chars().collect();
        let num_items = items.len();
        // The two rucksacks
        let r1: Result<HashSet<Item>, _> = items[0..num_items / 2].iter().map(|c| Item::try_from(*c)).collect();
        let r2: Result<HashSet<Item>, _> = items[num_items/2..].iter().map(|c| Item::try_from(*c)).collect();
        let (r1, r2) = (r1?, r2?);
        for item1 in &r1 {
            if r2.contains(item1) {
                tot_p1_priority += item1.priority;
            }
        }
        // Check what's common between a group of three elves
        let elf_items: HashSet<Item> = HashSet::from_iter(r1.into_iter().chain(r2.into_iter()));
        last_3_elves.push_back(elf_items);
        if last_3_elves.len() == 3 {
            // Completed a group: find common item
            let common_item_set = last_3_elves
                .into_iter()
                .reduce(|acc, current| {
                    let intersection = acc.intersection(&current);
                    intersection.map(|ritem| ritem.clone()).collect()
                });
            if let Some(common_item) = common_item_set.map(|set| set.into_iter().next()) {
                if let Some(common_item) = common_item {
                    tot_p2_priority += common_item.priority();
                } else {
                    return Err("Could not process item".into());
                }
            } else {
                return Err("Could not process item".into());
            }
            last_3_elves = VecDeque::new();
        }
    }
    println!("P1 Total priority: {}", tot_p1_priority);
    println!("P2 Total priority: {}", tot_p2_priority);
    Ok(())
}
