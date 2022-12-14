use std::fs::read_to_string;

const SIZE: usize = 99;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut trees: [[u32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            trees[row][col] = char.to_digit(10).unwrap();
        }
    }
    let mut max_score = 0;
    for x in 0..SIZE {
        for y in 0..SIZE {
            let mut left_trees = trees[x].iter().rev().copied();
            let mut right_trees = trees[x].iter().copied();
            let mut top_trees = trees.iter().map(|e| e[y]).rev();
            let mut bottom_trees = trees.iter().map(|e| e[y]);
            let tree_score = left_trees.nth(SIZE - (y + 1)).unwrap();
            assert_eq!(right_trees.nth(y).unwrap(), tree_score);
            assert_eq!(top_trees.nth(SIZE - (x + 1)).unwrap(), tree_score);
            assert_eq!(bottom_trees.nth(x).unwrap(), tree_score);
            assert_eq!(tree_score, trees[x][y]);
            let scores = [
                count(left_trees, tree_score),
                count(right_trees, tree_score),
                count(top_trees, tree_score),
                count(bottom_trees, tree_score),
            ];
            let cur_score = scores[0] * scores[1] * scores[2] * scores[3];
            max_score = max_score.max(cur_score);
        }
    }
    println!("{}", max_score);
}

fn count(trees: impl Iterator<Item = u32>, max_size: u32) -> i32 {
    let mut cnt = 0;
    for t in trees {
        cnt += 1;
        if t >= max_size {
            break;
        }
    }
    cnt
}
