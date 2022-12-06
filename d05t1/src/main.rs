use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (base, actions) = input.split_once("\n\n").unwrap();
    let mut cols: [&mut Vec<char>; 9] = [
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
    ];
    for line in base.lines().rev().skip(1) {
        for (index, col) in cols.iter_mut().enumerate() {
            let char = line.chars().nth((index * 4) + 1).unwrap();
            if !char.is_whitespace() {
                col.push(char);
            }
        }
    }
    for line in actions.lines().map(extract_action) {
        for _ in 0..line.0 {
            let moving_value = cols[line.1 as usize - 1].pop().unwrap();
            cols[line.2 as usize - 1].push(moving_value);
        }
    }
    let result = cols
        .map(|col| col.pop().unwrap())
        .iter()
        .collect::<String>();
    println!("{}", result);
}

fn extract_action(line: &str) -> (u8, u8, u8) {
    let mut args = line.split_whitespace();
    let amount: u8 = args.nth(1).unwrap().parse().unwrap();
    let from: u8 = args.nth(1).unwrap().parse().unwrap();
    let to: u8 = args.nth(1).unwrap().parse().unwrap();
    (amount, from, to)
}
