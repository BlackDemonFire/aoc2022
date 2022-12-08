use std::fs::read_to_string;

const SLASH: &str = "/";
const DISK_SIZE: usize = 70_000_000;
const NECESSARY_DISK_SPACE: usize = 30_000_000;

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
    let mut used_disk_space = 0usize;
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
                used_disk_space += size;
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
    let required_space = used_disk_space + NECESSARY_DISK_SPACE - DISK_SIZE;
    let mut matching_directories = dir_sizes
        .into_iter()
        .filter(|d| d.size >= required_space).collect::<Vec<_>>();
    matching_directories.sort_by(|a, b| a.size.cmp(&b.size));
    println!("{}", matching_directories.first().unwrap().size);
}
