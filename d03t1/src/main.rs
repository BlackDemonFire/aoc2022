use std::{fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let lines = input.lines();
    let mut result = 0u64;
    for line in lines {
        let (first_compartment, second_compartment) = line.split_at(line.len()/2);
        let mutual_char = first_compartment.chars().find(|&c| second_compartment.contains(c)).expect("No mutual char found");
        if mutual_char.is_ascii_uppercase() {
            result += mutual_char as u64 - 'A' as u64 + 27;
        } else {
            result += mutual_char as u64 - 'a' as u64 + 1;
        }
    }
    println!("{}", result);
}
