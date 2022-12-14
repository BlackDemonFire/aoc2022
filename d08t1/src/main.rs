use std::fs::read_to_string;

#[derive(PartialEq, Eq, Clone, Debug)]
enum State {
    Undetermined,
    Visible,
    Invisible,
}

#[derive(Clone, Debug)]
struct Tree {
    x: usize,
    y: usize,
    state: State,
    value: u32,
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let last_line_index = input.lines().count() - 1;
    let last_column_index = input.lines().next().unwrap().chars().count() - 1;
    let mut trees = vec![];
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            trees.push(Tree {
                state: State::Undetermined,
                value: char.to_digit(10).unwrap(),
                x: col,
                y: row,
            });
        }
    }
    let trees_clone = trees.clone();
    for tree in &mut trees.iter_mut() {
        if tree.x == 0 || tree.y == 0 || tree.x == last_column_index || tree.y == last_line_index {
            tree.state = State::Visible;
            continue;
        }
        let x_trees = trees_clone
            .iter()
            .filter(|other_tree| other_tree.x == tree.x)
            .collect::<Vec<_>>();
        let y_trees = trees_clone
            .iter()
            .filter(|other_tree| other_tree.y == tree.y)
            .collect::<Vec<_>>();
        if (!x_trees
            .iter()
            .filter(|other_tree| (other_tree.y < tree.y))
            .any(|other_tree| other_tree.value >= tree.value))
            || !(x_trees
                .iter()
                .filter(|other_tree| (other_tree.y > tree.y))
                .any(|other_tree| other_tree.value >= tree.value))
            || (!y_trees
                .iter()
                .filter(|other_tree| (other_tree.x < tree.x))
                .any(|other_tree| other_tree.value >= tree.value))
            || !(y_trees
                .iter()
                .filter(|other_tree| (other_tree.x > tree.x))
                .any(|other_tree| other_tree.value >= tree.value))
        {
            tree.state = State::Visible;
            continue;
        }
        tree.state = State::Invisible;
    }
    println!(
        "{}",
        trees.iter().filter(|a| a.state == State::Visible).count()
    );
}
