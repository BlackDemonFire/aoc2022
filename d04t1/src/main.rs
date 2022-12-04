use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let result = input
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(a, b)| (a.split_once("-").unwrap(), b.split_once("-").unwrap()))
        .map(|(a, b)| {
            (
                (a.0.parse::<usize>().unwrap(), a.1.parse::<usize>().unwrap()),
                (b.0.parse::<usize>().unwrap(), b.1.parse::<usize>().unwrap()),
            )
        })
        .filter(|(a, b)| contains(a, b) || contains(b, a))
        .count();
    println!("{:?}", result);
}

fn contains(a: &(usize, usize), b: &(usize, usize)) -> bool {
    a.0 <= b.0 && a.0 <= b.1 && a.1 >= b.0 && a.1 >= b.1
}
