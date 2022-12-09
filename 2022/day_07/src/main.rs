use std::collections::HashMap;

struct Dir {
    size: i32,
    dirs: HashMap<String, Dir>,
    files: HashMap<String, i32>,
}

impl Dir{
    pub fn new() -> Self{
        Self { 
            size: 0,
            dirs: Default::default(),
            files: Default::default(),
        }
    }
}

fn get_dir<'a>(root: &'a mut Dir, stack: &Vec<String>) -> &'a mut Dir {
    let mut curr = root;
    for entry in stack {
        curr = curr.dirs.get_mut(entry).expect("unknown dir"); 
    }
    return curr; 
}

fn calculate_dir_size(dir: &Dir) -> i32 {
    let mut size = if dir.size < 100000 { dir.size } else { 0 };
    for (_, child_dir) in &dir.dirs {
        size += calculate_dir_size(&child_dir);     
    }
    return size;
}

fn find_smallest_dir(dir: &Dir, smallest_size: i32, desired_size: i32, depth: i32) -> i32 {

    if dir.size < desired_size {
        return smallest_size;
    }

    let mut size = smallest_size;
    if dir.size < smallest_size{
        size = dir.size;   
    }

    for (_, child_dir) in &dir.dirs {
        size = find_smallest_dir(child_dir, size, desired_size, depth + 1);
    }
    return size;
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("failed to read file");
    let mut root = Dir::new();
    let mut stack: Vec<String> = Vec::new();
    for line in input.lines() {
        let split = line.split(' ').collect::<Vec<&str>>();
        match split[0].as_ref() {
            "$" => {
                // Command
                match split[1].as_ref() {
                    "ls" => { },
                    "cd" => {
                        match split[2].as_ref() {
                            ".." => { stack.truncate(stack.len() - 1); }
                            "/" => { stack.clear(); }
                            _ => { stack.push(split[2].to_string()); }
                        }
                    }
                    _ => { println!("Unknown command {line}"); },
                }
            },
            "dir" => {
                // Dir
                let curr_dir = get_dir(&mut root, &stack);
                match curr_dir.dirs.get(split[1]) {
                    Some(_) => { },
                    None => {
                        // New dir
                        let new_dir = Dir {
                            size: 0,
                            dirs: Default::default(),
                            files: Default::default(),
                        };
                        curr_dir.dirs.insert(
                            split[1].to_string(), 
                            new_dir);
                    },
                }
            },
            _ => {
                // File
                let curr_dir = get_dir(&mut root, &stack);
                match curr_dir.files.get(split[1]) {
                    Some(_) => { },
                    None => {
                        // New file
                        let size = split[0]
                            .to_string()
                            .parse::<i32>()
                            .expect("file size not a number");
                        curr_dir.files.insert(
                            split[1].to_string(), 
                            size);
                        let mut curr = &mut root;
                        for entry in &stack {
                            curr.size += size;
                            curr = curr.dirs.get_mut(entry).expect("unknown dir"); 
                        }
                        curr.size += size;
                    },
                }
            },
        }
    }

    let space = calculate_dir_size(&root);
    println!("Puzzle 1: {space}");

    let total_space = 70000000;
    let required_space = 30000000;
    let used_space = root.size;
    let unused_space = total_space - used_space;
    let overflowing_space = required_space - unused_space;
    let smallest_dir_size = find_smallest_dir(&root, total_space, overflowing_space, 0);
    println!("Puzzle 2: {smallest_dir_size}");
}
