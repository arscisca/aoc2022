use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Range {
    start: u32,
    end: u32,
}

impl Range {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn fully_contains(&self, other: &Self) -> bool {
        (self.start <= other.start && other.start <= self.end)
            && (self.start <= other.end && other.end <= self.end)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (self.start <= other.end && other.end <= self.end)
            || (other.start <= self.start && self.start <= other.end)
            || (other.start <= self.end && self.end <= other.end)
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('-') {
            Some((start, end)) => {
                let start = start.parse::<u32>().map_err(|e| e.to_string())?;
                let end = end.parse::<u32>().map_err(|e| e.to_string())?;
                Ok(Self::new(start, end))
            }
            None => Err(format!("Could not parse string: {}", s)),
        }
    }
}

fn main() {
    let f = File::open("data.txt").expect("Could not open file");
    let reader = BufReader::new(f);

    let mut tot_fully_overlapping = 0;
    let mut tot_overlapping = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let (r1, r2) = line.split_once(',').expect("Ill-formatted line");
        let r1: Range = r1.parse().expect("Could not parse range 1");
        let r2: Range = r2.parse().expect("Could not parse range 2");
        if r1.overlaps(&r2) {
            println!("Ranges {} and {} overlap", r1, r2);
            tot_overlapping += 1;
        } else {
            println!("Ranges {} and {} DON'T overlap", r1, r2);
        }
        if r1.fully_contains(&r2) || r2.fully_contains(&r1) {
            tot_fully_overlapping += 1;
        }
    }

    println!("Total fully overlapping ranges: {}", tot_fully_overlapping);
    println!("Total overlapping ranges: {}", tot_overlapping);
}
