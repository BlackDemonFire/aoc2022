use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut points: u64 = 0;
    for line in input.lines() {
        let (opponent, me) = line.split_once(' ').unwrap();
        points += match me {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!(),
        };
        // A = Rock
        // B = Paper
        // C = Scissors
        // X = Rock
        // Y = Paper
        // Z = Scissors
        points += match (opponent, me) {
            ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
            ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
            _ => panic!(),
        }
    }
    println!("{}", points);
}
