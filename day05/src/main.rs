use std::error::Error;
use std::io::{BufReader, BufRead};

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn top(&self) -> Option<&T> {
        let len = self.len();
        if len == 0 {
            None
        } else {
            Some(&self.items[len - 1])
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn push_n(&mut self, items: Vec<T>) {
        for item in items.into_iter().rev() {
            self.push(item);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn pop_n(&mut self, n: usize) -> Option<Vec<T>> {
        if self.len() < n {
            return None;
        }
        let mut items = Vec::with_capacity(n);
        for _ in 0..n {
            items.push(self.items.pop().unwrap());
        }
        Some(items)
    }

    pub fn reverse(&mut self) {
        self.items.reverse();
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for i in (0..self.len()).rev() {
            if i != self.len() - 1 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", self.items[i])?;
        }
        write!(f, "]")
    }
}

fn parse_stacks<R: std::io::BufRead>(f: &mut R) -> Result<Vec<Stack<char>>, Box<dyn Error>> {
    let mut line = String::new();
    f.read_line(&mut line)?;
    // Determine the number of stacks based on the constant spacing of the input
    let num_stacks = (line.len() + 1) / 4;
    let mut stacks = Vec::with_capacity(num_stacks);
    stacks.resize_with(num_stacks, || Stack::default());
    loop {
        let chars: Vec<char> = line.chars().collect();
        // Check if line contains numbers (final line)
        if chars[1] == '1' {
            for stack in &mut stacks {
                stack.reverse();
            }
            return Ok(stacks);
        }
        // Process line
        for i in 0..num_stacks {
            let char_index = 1 + i * 4;
            if chars[char_index].is_ascii_alphabetic() {
                stacks[i].push(chars[char_index]);
            }
        }
        line.clear();
        f.read_line(&mut line)?;
    }
}

fn print_tops(stacks: &Vec<Stack<char>>) {
    for stack in stacks {
        let top = stack.top().unwrap_or(&' ');
        print!("{}", top);
    }
    println!();
}

fn part1() {
    println!("--- Part 2 ---");
    let f = std::fs::File::open("data.txt").expect("Could not open file");
    let mut reader = BufReader::new(f);
    // Read stacks
    let mut stacks = parse_stacks(&mut reader).expect("Could not parse stacks");
    println!("Stacks: {:?}", stacks);
    // Skip empty line
    let mut line = String::new();
    reader.read_line(&mut line).expect("Could not skip empty line");
    // Read commands
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let quantity: usize = tokens[1].parse().expect("Could not parse quantity");
        let origin: usize = tokens[3].parse().expect("Could not parse origin");
        let origin = origin - 1;
        let dest: usize = tokens[5].parse().expect("Could not parse destination");
        let dest = dest - 1;
        for _ in 0..quantity {
            match stacks[origin].pop() {
                Some(item) => stacks[dest].push(item),
                None => panic!("Could not move item"),
            }
        }
    }
    // Print final status
    println!("Stacks: {:?}", stacks);
    print_tops(&stacks);
}

fn part2() {
    let f = std::fs::File::open("data.txt").expect("Could not open file");
    let mut reader = BufReader::new(f);
    // Read stacks
    let mut stacks = parse_stacks(&mut reader).expect("Could not parse stacks");
    println!("Stacks: {:?}", stacks);
    // Skip empty line
    let mut line = String::new();
    reader.read_line(&mut line).expect("Could not skip empty line");
    // Read commands
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let quantity: usize = tokens[1].parse().expect("Could not parse quantity");
        let origin: usize = tokens[3].parse().expect("Could not parse origin");
        let origin = origin - 1;
        let dest: usize = tokens[5].parse().expect("Could not parse destination");
        let dest = dest - 1;
        let moved = stacks[origin].pop_n(quantity).expect("Could not pop items");
        stacks[dest].push_n(moved);
    }
    // Print final status
    println!("Stacks: {:?}", stacks);
    print_tops(&stacks);
}

fn main() {
    part1();
    part2();
}