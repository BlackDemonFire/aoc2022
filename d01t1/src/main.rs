fn main() {
    let input = std::fs::read_to_string("inputs.txt").unwrap();
    let mut elves = Vec::<usize>::new();
    let mut current_elf = 0;
    for line in input.lines() {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += line.parse::<usize>().unwrap();
        }
    }
    println!("{}", elves.iter().max().unwrap());
}