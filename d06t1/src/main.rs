use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut storage = VecDeque::<char>::new();
    for (index, char) in input.chars().enumerate() {
        storage.push_back(char);
        if storage.len() > 4 {
            storage.pop_front();
            if !storage
                .iter()
                .any(|c| storage.iter().filter(|d| &c == d).collect::<Vec<_>>().len() > 1)
            {
                println!("{}", index + 1);
                break;
            }
        }
    }
}
