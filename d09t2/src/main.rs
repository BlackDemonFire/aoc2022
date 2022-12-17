use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut positions: Vec<(isize, isize)> = vec![];
    let mut knots: [(isize, isize); 10] = [
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    for (direction, amount) in input
        .lines()
        .map(|e| e.split_once(' '))
        .map(Option::unwrap)
        .map(|(a, b)| (a, b.parse::<u8>().unwrap()))
    {
        for _ in 0..amount {
            match direction.chars().next().unwrap() {
                'L' => knots[0].0 -= 1,
                'R' => knots[0].0 += 1,
                'U' => knots[0].1 += 1,
                'D' => knots[0].1 -= 1,
                _ => unreachable!(),
            };
            for i in 0..9 {
                if (knots[i].0 - knots[i + 1].0).abs() > 1 {
                    knots[i + 1].0 +=
                        (knots[i].0 - knots[i + 1].0) / ((knots[i].0 - knots[i + 1].0).abs());
                    if (knots[i].1 - knots[i + 1].1) != 0 {
                        knots[i + 1].1 +=
                            (knots[i].1 - knots[i + 1].1) / ((knots[i].1 - knots[i + 1].1).abs());
                    }
                } else if (knots[i].1 - knots[i + 1].1).abs() > 1 {
                    knots[i + 1].1 +=
                        (knots[i].1 - knots[i + 1].1) / ((knots[i].1 - knots[i + 1].1).abs());
                    if (knots[i].0 - knots[i + 1].0) != 0 {
                        knots[i + 1].0 +=
                            (knots[i].0 - knots[i + 1].0) / ((knots[i].0 - knots[i + 1].0).abs());
                    }
                }
            }
            positions.push(knots[9]);
        }
    }
    positions.sort_unstable();
    positions.dedup();
    println!("{}", positions.len());
}
