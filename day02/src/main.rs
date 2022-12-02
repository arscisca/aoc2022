use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone)]
pub enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    pub fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    pub fn inverse(&self) -> Outcome {
        match self {
            Self::Win => Self::Lose,
            Self::Draw => Self::Draw,
            Self::Lose => Self::Win,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn from_challenge_outcome(opponent: &Shape, outcome: &Outcome) -> Self {
        match (opponent, outcome) {
            (&Self::Rock, &Outcome::Win) => Self::Paper,
            (&Self::Rock, &Outcome::Draw) => Self::Rock,
            (&Self::Rock, &Outcome::Lose) => Self::Scissors,
            (&Self::Paper, &Outcome::Win) => Self::Scissors,
            (&Self::Paper, &Outcome::Draw) => Self::Paper,
            (&Self::Paper, &Outcome::Lose) => Self::Rock,
            (&Self::Scissors, &Outcome::Win) => Self::Rock,
            (&Self::Scissors, &Outcome::Draw) => Self::Scissors,
            (&Self::Scissors, &Outcome::Lose) => Self::Paper,
        }
    }

    pub fn challenge(&self, other: &Shape) -> Outcome {
        match (self, other) {
            (&Self::Rock, &Self::Rock) => Outcome::Draw,
            (&Self::Rock, &Self::Paper) => Outcome::Lose,
            (&Self::Rock, &Self::Scissors) => Outcome::Win,
            (&Self::Paper, &Self::Rock) => Outcome::Win,
            (&Self::Paper, &Self::Paper) => Outcome::Draw,
            (&Self::Paper, &Self::Scissors) => Outcome::Lose,
            (&Self::Scissors, &Self::Rock) => Outcome::Lose,
            (&Self::Scissors, &Self::Paper) => Outcome::Win,
            (&Self::Scissors, &Self::Scissors) => Outcome::Draw,
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

fn score_challenge(p1: &Shape, p2: &Shape) -> u32 {
    p1.challenge(p2).score() + p1.value()
}

fn part1() {
    println!("--- PART 1 ---");
    // Read strategy
    let file = File::open("data.txt").expect("Could not open file");
    let reader = BufReader::new(file);
    // Compute scores
    let mut total_score = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let symbols = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let (opponent, player) = (symbols[0], symbols[1]);
        let opponent = match opponent {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Cannot parse opponent move {}", opponent),
        };
        let player = match player {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("Cannot parse player symbol {}", player),
        };
        let score = score_challenge(&player, &opponent);
        total_score += score;
    }
    println!("Total score: {}", total_score);
}

fn part2() {
    println!("--- PART 1 ---");
    // Read strategy
    let file = File::open("data.txt").expect("Could not open file");
    let reader = BufReader::new(file);
    // Compute scores
    let mut total_score = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let symbols = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let (opponent, outcome) = (symbols[0], symbols[1]);
        let opponent = match opponent {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Cannot parse opponent move {}", opponent),
        };
        let outcome = match outcome {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Cannot outcome symbol {}", outcome),
        };
        let player = dbg!(Shape::from_challenge_outcome(&opponent, &outcome));
        let score = score_challenge(&player, &opponent);
        total_score += score;
    }
    println!("Total score: {}", total_score);
}

fn main() {
    part1();
    part2();
}
