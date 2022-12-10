use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct File {
    pub name: String,
    pub size: u32,
}

impl File {
    fn new(name: String, size: u32) -> Self {
        Self { name, size }
    }
}

#[derive(Debug)]
struct Directory {
    pub files: Vec<File>,
    pub directories: HashMap<String, Rc<RefCell<Directory>>>,
    pub parent: Weak<RefCell<Directory>>,
}

impl Directory {
    fn size(&self) -> u32 {
        let file_sizes: u32 = self.files.iter().map(|file| file.size).sum();
        let directory_sizes: u32 = self
            .directories
            .iter()
            .map(|(_, directory)| directory.borrow().size())
            .sum();
        file_sizes + directory_sizes
    }

    fn dir_sizes(&self) -> Vec<u32> {
        let mut sizes: Vec<_> = self
            .directories
            .iter()
            .map(|(_, directory)| directory.borrow().dir_sizes())
            .flatten()
            .collect();
        sizes.push(self.size());
        sizes
    }

    fn new(parent: Weak<RefCell<Directory>>) -> Self {
        Self {
            files: vec![],
            directories: HashMap::new(),
            parent,
        }
    }

    fn from_str(input: &str) -> Rc<RefCell<Self>> {
        let root = Rc::new(RefCell::new(Directory {
            files: vec![],
            directories: HashMap::new(),
            parent: Weak::new(), // no parent for root directory
        }));
        let mut current_dir = Rc::clone(&root);

        let chunks = input.split("$ ");

        for chunk in chunks {
            let mut lines = chunk.lines();
            if let Some(command) = lines.next() {
                let mut tokens = command.split(" ");
                match tokens.next().unwrap() {
                    "cd" => match tokens.next().unwrap() {
                        "/" => current_dir = Rc::clone(&root),
                        ".." => {
                            let new_dir = Rc::clone(
                                &current_dir
                                    .borrow_mut()
                                    .parent
                                    .upgrade()
                                    .expect("can't go up from root"),
                            );
                            current_dir = new_dir;
                        }
                        directory => {
                            let new_dir = Rc::clone(
                                current_dir
                                    .borrow_mut()
                                    .directories
                                    .get_mut(directory)
                                    .expect("sub directory should exist"),
                            );
                            current_dir = new_dir;
                        }
                    },
                    "ls" => {
                        for line in lines {
                            let mut tokens = line.split(" ");
                            match tokens.next().expect("line should have at least 1 token") {
                                "dir" => {
                                    current_dir.borrow_mut().directories.insert(
                                        tokens.next().unwrap().to_string(),
                                        Rc::new(RefCell::new(Directory::new(Rc::downgrade(
                                            &current_dir,
                                        )))),
                                    );
                                }
                                file_size => {
                                    let size: u32 = file_size.parse().unwrap();
                                    let file = File::new(tokens.next().unwrap().to_string(), size);
                                    current_dir.borrow_mut().files.push(file);
                                }
                            }
                        }
                    }
                    _ => panic!("unknown command `{command}`"),
                }
            }
        }
        root
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day7.txt").expect("Failed to read file");

    let dir_tree = Directory::from_str(&input);
    let sum_of_small_dirs: u32 = dir_tree
        .borrow()
        .dir_sizes()
        .into_iter()
        .filter(|size| *size <= 100_000)
        .sum();
    dbg!(sum_of_small_dirs);

    const MAX_ALLOWED_USAGE: u32 = 70_000_000 - 30_000_000;
    let total_usage = dir_tree.borrow().size();
    let min_dir_size = total_usage - MAX_ALLOWED_USAGE;
    let dir_size_to_delete = dir_tree
        .borrow()
        .dir_sizes()
        .into_iter()
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

        let tree = Directory::from_str(INPUT);

        assert_eq!(48381165, tree.borrow().size());
        dbg!(tree.borrow().dir_sizes());
    }
}
