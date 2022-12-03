fn main() {
    let mut input = std::fs::read_to_string("inputs.txt").unwrap();
    let mut elves = Vec::<usize>::new();
    let mut current_elf = 0;
    for line in input.lines() {
        if line == "" {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += line.parse::<usize>().unwrap();
        }
    }
    elves.sort();
    elves.reverse();
    elves.truncate(3);
    println!("{}", elves.iter().sum::<usize>());
}