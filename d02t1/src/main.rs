use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut points: u64 = 0;
    for line in input.lines() {
        let (opponent, me) = line.split_once(" ").unwrap();
        points += match me {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!()
        };
        // A = Rock
        // B = Paper
        // C = Scissors
        // X = Rock
        // Y = Paper
        // Z = Scissors
        points += match (opponent, me) {
            ("A","X") => 3,
            ("B","X") => 0,
            ("C","X") => 6,
            ("A","Y") => 6,
            ("B","Y") => 3,
            ("C","Y") => 0,
            ("A","Z") => 0,
            ("B","Z") => 6,
            ("C","Z") => 3,
            _ => panic!()
        }
    }
    println!("{}", points);
}