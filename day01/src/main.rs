use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_elves(fname: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let mut elves: Vec<u32> = Vec::new();
    elves.push(0);
    // Read input
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let last_elf_idx = elves.len() - 1;
        let elf = &mut elves[last_elf_idx];
        if !line.is_empty() {
            let calories: u32 = line.parse().map_err(|_| format!("Could not parse value: {}", line))?;
            *elf += calories;
        } else {
            elves.push(0);
        }
    }
    Ok(elves)
}

fn main() {
    let elves = parse_elves("data.txt").expect("Could not parse elves");

    // Find top three elves with the most calories
    let mut top_three = [0; 3];
    for elf in &elves {
        for i in 0..3 {
            if *elf > top_three[i] {
                // Insert in ranking
                for j in (i .. 3-1).rev() {
                    top_three[j+1] = top_three[j];
                }
                top_three[i] = *elf;
                break;
            }
        }
    }

    let top_elf_total = top_three[0];
    let top_three_total: u32 = top_three.iter().sum();
    println!("Elf with most calories: {}", top_elf_total);
    println!("Top 3 elves total calories: {}", top_three_total);
}
