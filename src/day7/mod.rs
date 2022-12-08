use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    rc::Rc,
};

use super::utils::Problem;

enum Cmd<'a> {
    CD(&'a str),
    LS,
}

#[derive(Debug)]
struct Directory {
    path: String,
    size: u32,

    // Also updated by children
    total_size: u32,

    // This is ALL parents, direct or indirect
    parents: Vec<Rc<RefCell<Directory>>>,
}

fn get_directory_map() -> HashMap<String, Rc<RefCell<Directory>>> {
    let file = File::open("src/day7/input.txt").unwrap();
    let reader = BufReader::new(file);

    // Maps directory names to Directory references
    // This is where directories actually live
    let mut dirmap = HashMap::<String, Rc<RefCell<Directory>>>::new();
    dirmap.insert(
        String::from("/"),
        Rc::new(RefCell::new(Directory {
            parents: vec![],
            path: String::from("/"),
            size: 0,
            total_size: 0,
        })),
    );

    // Starts with `cd /` so just start with this
    let mut cwd = String::from("/");
    let mut lines_iter = reader.lines().skip(1).peekable();
    while lines_iter.peek().is_some() {
        // Get command
        let line = lines_iter.next().unwrap().unwrap();
        let mut iter = line.split_whitespace().skip(1);
        let (command, text) = (iter.next().unwrap(), iter.next());
        let cmd = match command {
            "cd" => Cmd::CD(text.unwrap()),
            "ls" | _ => Cmd::LS,
        };

        // Check command
        if let Cmd::CD(child_dir) = cmd {
            // Set new CWD REF
            let cwd_ref = dirmap.get(&cwd).unwrap();
            if child_dir == ".." {
                cwd = PathBuf::from(&cwd_ref.borrow().path.clone())
                    .parent()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                continue;
            }

            // Push to path
            let mut next_path = PathBuf::from(&cwd_ref.borrow().path.clone());
            next_path.push(child_dir);

            let newpath = String::from(next_path.to_str().unwrap());
            let mapres = dirmap.get(&newpath);
            match mapres {
                Some(next_dir) => {
                    cwd = next_dir.borrow().path.clone();
                }
                None => {
                    panic!("Not sure what to do here");
                }
            }
        }

        // Must be an LS command
        // Iterate until new command found
        loop {
            if lines_iter.peek().is_none() {
                break;
            }

            // Break if command found
            let peeked = lines_iter.peek().unwrap();
            if peeked.as_ref().unwrap().starts_with("$") {
                break;
            }

            // Not a command, grab it
            let text = lines_iter.next().unwrap().unwrap();
            let mut split = text.split_whitespace();
            let (a, b) = (split.next().unwrap(), split.next().unwrap());

            // Parse if dir or not
            if a == "dir" {
                let mut newpath = PathBuf::from(&cwd);
                newpath.push(b);

                // Get current dir
                let directory = dirmap.get(&cwd).unwrap();

                // Create new parents
                let mut new_parents = directory.borrow().parents.clone();
                new_parents.push(Rc::clone(directory));

                // Create and push new dir
                let pathstr = String::from(newpath.to_str().unwrap());
                dirmap.insert(
                    pathstr.clone(),
                    Rc::new(RefCell::new(Directory {
                        path: pathstr,
                        size: 0,
                        total_size: 0,
                        parents: new_parents,
                    })),
                );
            } else {
                // Get current dir
                let mut directory = dirmap.get(&cwd).unwrap().borrow_mut();
                let size = a.parse::<u32>().expect("Couldn't parse file size");

                // Update for this dir
                directory.size += size;
                directory.total_size += size;

                // Update for all parents
                for parent in &directory.parents {
                    parent.borrow_mut().total_size += size;
                }
            }
        }
    }

    dirmap
}

pub struct Day7 {}
impl Problem for Day7 {
    fn day(&self) -> &str {
        return "SEVEN";
    }

    fn part_one(&self) -> String {
        let dirmap = get_directory_map();
        let sum: u32 = dirmap
            .values()
            .filter_map(|dir| {
                let d = dir.borrow();
                if d.total_size > 100000 {
                    return None;
                }

                Some(d.total_size)
            })
            .sum();
        return sum.to_string();
    }

    fn part_two(&self) -> String {
        let dirmap = get_directory_map();

        let used_space = dirmap.get("/").unwrap().borrow().total_size;
        let free_space = 70000000 - used_space;
        let space_to_free = 30000000 - free_space;

        // Create vector of sizes
        let mut sizes = Vec::from_iter(dirmap.values().map(|dir| dir.borrow().total_size));
        sizes.sort();

        // Find fitting value
        for i in sizes {
            if i >= space_to_free {
                return i.to_string();
            }
        }
        panic!("Loop exited without correct answer");
    }
}
