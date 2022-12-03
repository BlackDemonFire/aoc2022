use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let mut result = 0u64;
    loop {
        let opt_first_rucksack = lines.next();
        if opt_first_rucksack.is_none() {
            break;
        }
        let first_rucksack = opt_first_rucksack.unwrap();
        let second_rucksack = lines.next().unwrap();
        let third_rucksack = lines.next().unwrap();
        let mutual_char = first_rucksack
            .chars()
            .find(|c| second_rucksack.contains(*c) && third_rucksack.contains(*c))
            .unwrap();
        if mutual_char.is_ascii_uppercase() {
            result += mutual_char as u64 - 'A' as u64 + 27;
        } else {
            result += mutual_char as u64 - 'a' as u64 + 1;
        }
    }
    println!("{}", result);
}
