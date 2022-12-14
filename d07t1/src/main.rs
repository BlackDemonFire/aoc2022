use std::fs::read_to_string;

const SLASH: &str = "/";

#[derive(Debug)]
struct DirSize {
    name: String,
    size: usize,
}
fn dir_up(dir: &str) -> String {
    let mut directories = dir.split('/').collect::<Vec<&str>>();
    directories.pop();
    directories.join("/")
}
fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut cwd = String::new();

    let mut dir_sizes: Vec<DirSize> = vec![];
    for line in input.lines().skip(1) {
        let mut args = line.split_whitespace();
        let action_type = args.next().unwrap();
        let command_or_filename = args.next().unwrap();
        match action_type {
            "$" => match command_or_filename {
                "cd" => match args.next().unwrap() {
                    ".." => {
                        cwd = dir_up(&cwd);
                    }
                    dir => {
                        cwd += &(SLASH.to_owned() + dir);
                    }
                },
                "ls" => {}
                other => panic!("Unknown command: {}", other),
            },
            "dir" => {
                let name = command_or_filename;
                dir_sizes.push(DirSize {
                    name: cwd.clone() + &(SLASH.to_owned() + name),
                    size: 0,
                });
            }
            size_str => {
                let size = size_str.parse::<usize>().unwrap();
                let mut tmp_dir = cwd.clone();
                for _ in 0..cwd.matches('/').count() {
                    dir_sizes
                        .iter_mut()
                        .find(|d| d.name == tmp_dir)
                        .unwrap()
                        .size += size;
                    tmp_dir = dir_up(&tmp_dir);
                }
            }
        }
    }
    let result: usize = dir_sizes
        .into_iter()
        .filter(|d| d.size <= 100_000)
        .map(|d| d.size)
        .sum();
    println!("{}", result);
}
