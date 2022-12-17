use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut positions: Vec<(isize, isize)> = vec![];
    let mut head = (0isize, 0isize);
    let mut tail = (0isize, 0isize);
    for (direction, amount) in input
        .lines()
        .map(|e| e.split_once(' '))
        .map(Option::unwrap)
        .map(|(a, b)| (a, b.parse::<u8>().unwrap()))
    {
        for _ in 0..amount {
            match direction.chars().next().unwrap() {
                'L' => head.0 -= 1,
                'R' => head.0 += 1,
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                _ => unreachable!(),
            };
            if (head.0 - tail.0).abs() > 1 {
                tail.0 += (head.0 - tail.0) / ((head.0 - tail.0).abs());
                if (head.1 - tail.1) != 0 {
                    tail.1 += (head.1 - tail.1) / ((head.1 - tail.1).abs());
                }
            } else if (head.1 - tail.1).abs() > 1 {
                tail.1 += (head.1 - tail.1) / ((head.1 - tail.1).abs());
                if (head.0 - tail.0) != 0 {
                    tail.0 += (head.0 - tail.0) / ((head.0 - tail.0).abs());
                }
            }
            positions.push(tail);
        }
    }
    positions.sort_unstable();
    positions.dedup();
    println!("{}", positions.len());
}
