use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut points: u64 = 0;
    for line in input.lines() {
        let (opponent, me) = line.split_once(" ").unwrap();
        // A = Rock = 1
        // B = Paper = 2
        // C = Scissors = 3
        // X = Lose = 0
        // Y = Draw = 3
        // Z = Win = 6
        points += match (opponent, me) {
            ("A","X") => 0 + 3,
            ("B","X") => 0 + 1,
            ("C","X") => 0 + 2,
            ("A","Y") => 3 + 1,
            ("B","Y") => 3 + 2,
            ("C","Y") => 3 + 3,
            ("A","Z") => 6 + 2,
            ("B","Z") => 6 + 3,
            ("C","Z") => 6 + 1,
            _ => panic!()
        }
    }
    println!("{}", points);
}