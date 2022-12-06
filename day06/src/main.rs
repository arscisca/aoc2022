use std::collections::VecDeque;
use std::io::Read;

pub struct Window<T> {
    data: VecDeque<T>,
    size: usize,
}

impl<T> Window<T> {
    pub fn new(size: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(size + 1),
            size,
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_filled(&self) -> bool {
        self.data.len() == self.size
    }

    pub fn insert(&mut self, value: T) {
        self.data.push_back(value);
        if self.data.len() > self.size {
            self.data.pop_front();
        }
    }
}

impl<T> std::ops::Index<usize> for Window<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

fn is_start_marker(window: &Window<char>) -> bool {
    if !window.is_filled() {
        return false;
    }
    let characters: std::collections::HashSet<char> = window.data.iter().map(|c| *c).collect();
    characters.len() == window.len()
}

impl<T: std::fmt::Debug> std::fmt::Debug for Window<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.data, f)
    }
}

fn main() {
    let mut f = std::fs::File::open("data.txt").expect("Could not open file");
    let mut signal = String::new();
    f.read_to_string(&mut signal).expect("Could not read file");
    // Search for packet start
    let mut packet_start_window = Window::new(4);
    let mut char_indices = signal.char_indices();
    for (i, c) in &mut char_indices {
        packet_start_window.insert(c);
        if is_start_marker(&packet_start_window) {
            println!("Packet start is at character {}", i + 1);
            break;
        }
    }
    // Search for message start
    let mut message_start_window = Window::new(14);
    for c in packet_start_window.data.into_iter() {
        message_start_window.insert(c);
    }
    for (i, c) in char_indices {
        message_start_window.insert(c);
        if is_start_marker(&message_start_window) {
            println!("Message start is at character {}", i + 1);
            break;
        }
    }
}
