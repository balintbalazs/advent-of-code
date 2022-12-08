use std::{collections::HashMap, fs};

fn parse_input(input: &str) -> HashMap<Vec<String>, u32> {
    let mut dir_tree = HashMap::new();
    let root: Vec<String> = vec!["/".to_string()];
    let mut current_dir = root.clone();

    let chunks = input.split("$ ");

    for chunk in chunks {
        let mut lines = chunk.lines();
        if let Some(command) = lines.next() {
            let mut tokens = command.split(" ");
            match tokens.next().unwrap() {
                "cd" => match tokens.next().unwrap() {
                    "/" => current_dir = root.clone(),
                    ".." => {
                        if current_dir.len() > 1 { // don't go up if already in root
                            current_dir.pop();
                        }
                    }
                    directory => {
                        current_dir.push(directory.to_string());
                    }
                },
                "ls" => {
                    for line in lines {
                        let mut tokens = line.split(" ");
                        match tokens.next().expect("line should have at least 1 token") {
                            "dir" => {
                                let sub_dir = tokens.next().unwrap().to_string();
                                let mut new_path = current_dir.clone();
                                new_path.push(sub_dir);
                                dir_tree.insert(new_path, 0);
                            }
                            file_size => {
                                let file_size: u32 = file_size.parse().unwrap();
                                let mut parents = current_dir.clone();
                                while parents.len() > 0 {
                                    let dir_size = dir_tree.get(&parents).unwrap_or(&0);
                                    dir_tree.insert(parents.clone(), dir_size + file_size);
                                    parents.pop();
                                }
                            }
                        }
                    }
                }
                _ => panic!("unknown command `{command}`"),
            }
        }
    }
    dir_tree
}

fn main() {
    let input = fs::read_to_string("inputs/day7.txt").expect("Failed to read file");

    let dir_tree = parse_input(&input);
    let sum_of_small_dirs: u32 = dir_tree
        .iter()
        .map(|(_, size)| *size)
        .filter(|size| *size <= 100_000)
        .sum();
    dbg!(sum_of_small_dirs);

    const MAX_ALLOWED_USAGE: u32 = 70_000_000 - 30_000_000;
    let total_usage = dir_tree.get(&vec!["/".to_string()]).unwrap();
    let min_dir_size = total_usage - MAX_ALLOWED_USAGE;
    let dir_size_to_delete = dir_tree
        .iter()
        .map(|(_, size)| *size)
        .filter(|size| *size >= min_dir_size)
        .min();
    dbg!(dir_size_to_delete);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_parse_commands() {
        const INPUT: &str = r###"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"###;

        let tree = parse_input(INPUT);

        assert_eq!(&48381165, tree.get(&vec!["/".to_string()]).unwrap());
        dbg!(tree);
    }
}
