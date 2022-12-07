use regex::Regex;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct Entry {
    name: String,
    sibling: Option<Rc<RefCell<Entry>>>,
    parent: Option<Weak<RefCell<Entry>>>,
    data: EntryData,
}

#[derive(Debug, Clone)]
enum EntryData {
    File { size: usize },
    Directory { child: Option<Rc<RefCell<Entry>>> },
}

#[derive(Debug, Clone)]
struct Shell {
    root_dir: Rc<RefCell<Entry>>,
    cwd: Rc<RefCell<Entry>>,

    file_re: Regex,
    dir_re: Regex,
    cd_re: Regex,
}

#[derive(Debug)]
struct StackEntry {
    dir: Rc<RefCell<Entry>>,
    next_child: Option<Rc<RefCell<Entry>>>,
    size: usize,
}

struct DirIter {
    stack: Vec<StackEntry>,
}

impl DirIter {
    fn new(dir: Rc<RefCell<Entry>>) -> DirIter {
        let mut stack = Vec::<StackEntry>::new();

        stack.push(StackEntry {
            dir: dir.clone(),
            next_child: match dir.borrow().data {
                EntryData::Directory { child: ref next_child } =>
                    next_child.clone(),
                EntryData::File { .. } => panic!("Tried to iterate a file"),
            },
            size: 0,
        });

        DirIter { stack }
    }
}

impl Iterator for DirIter {
    type Item = (usize, Rc<RefCell<Entry>>);

    fn next(&mut self) -> Option<(usize, Rc<RefCell<Entry>>)> {
        loop {
            let top = match self.stack.last_mut() {
                Some(e) => e,
                None => break None,
            };

            match top.next_child {
                None => {
                    let size = top.size;
                    let dir = top.dir.clone();
                    self.stack.pop();

                    if let Some(top) = self.stack.last_mut() {
                        top.size += size;
                    }

                    break Some((size, dir));
                },
                Some(ref entry) => {
                    let entry = entry.clone();
                    top.next_child = entry.borrow().sibling.clone();

                    match entry.borrow().data {
                        EntryData::Directory { ref child } => {
                            self.stack.push(StackEntry {
                                dir: entry.clone(),
                                next_child: child.clone(),
                                size: 0,
                            });
                        },
                        EntryData::File { size } => {
                            top.size += size;
                            break Some((size, entry.clone()));
                        },
                    };
                },
            };
        }
    }
}

impl Entry {
    fn new_directory(name: String) -> Entry {
        Entry {
            name,
            parent: None,
            sibling: None,
            data: EntryData::Directory {
                child: None,
            },
        }
    }

    fn new_file(name: String, size: usize) -> Entry {
        Entry {
            name,
            parent: None,
            sibling: None,
            data: EntryData::File {
                size,
            },
        }
    }
}

impl Shell {
    fn new() -> Shell {
        let root_dir =
            Rc::new(RefCell::new(Entry::new_directory("".to_string())));

        Shell {
            root_dir: root_dir.clone(),
            cwd: root_dir.clone(),

            file_re: Regex::new(r"^(\d+) (.*)$").unwrap(),
            dir_re: Regex::new(r"^dir (.*)$").unwrap(),
            cd_re: Regex::new(r"^\$ cd (.*)$").unwrap(),
        }
    }

    fn cd_to_root(&mut self) {
        self.cwd = self.root_dir.clone();
    }

    fn cd_to_parent(&mut self) -> Result<(), String> {
        let parent = self.cwd.borrow().parent.clone();

        match parent {
            None => Err("Attempt to move to parent of root directory"
                        .to_string()),
            Some(parent) => {
                self.cwd =
                    parent.upgrade().expect("Link to parent from child \
                                             should always be valid \
                                             because the parent should keep \
                                             a reference to the child");
                Ok(())
            }
        }
    }

    fn cd_to_child(&mut self, child_name: &str) -> Result<(), String> {
        let cwd_rc = self.cwd.clone();
        let cwd_ref = cwd_rc.borrow();
        let mut next_child = match cwd_ref.data {
            EntryData::Directory { ref child } => child.clone(),
            _ => panic!("cwd is not a directory!"),
        };

        while let Some(ref child) = next_child {
            let child = child.clone();

            if child.borrow().name.eq(child_name) {
                return match child.borrow().data {
                    EntryData::Directory { .. } => {
                        self.cwd = child.clone();
                        Ok(())
                    },
                    _ => Err("Tried to change directory into a file"
                             .to_string()),
                };
            }

            next_child = child.borrow().sibling.clone();
        }

        Err("Tried to change directory into a file that doesn’t exist"
            .to_string())
    }

    fn add_entry(&mut self, child: Entry) {
        let mut child_mut = child;

        child_mut.parent = Some(Rc::downgrade(&self.cwd));

        let mut parent = self.cwd.borrow_mut();

        match parent.data {
            EntryData::Directory { ref mut child } => {
                child_mut.sibling = child.clone();
                *child = Some(Rc::new(RefCell::new(child_mut)));
            },
            EntryData::File { .. } =>
                panic!("Tried to add a child entry to a file entry"),
        };
    }

    fn add_file(&mut self, name: String, size: usize) {
        self.add_entry(Entry::new_file(name, size));
    }

    fn add_directory(&mut self, name: String) {
        self.add_entry(Entry::new_directory(name));
    }

    fn run_command(&mut self, command: &str) -> Result<(), String> {
        if command.eq("$ cd ..") {
            self.cd_to_parent()
        } else if command.eq("$ cd /") {
            self.cd_to_root();
            Ok(())
        } else if command.eq("$ ls") {
            // We don’t need to do anything because the filenames are
            // in the input
            Ok(())
        } else if let Some(captures) = self.file_re.captures(command) {
            let size = match captures[1].parse::<usize>() {
                Err(e) => return Err(e.to_string()),
                Ok(size) => size,
            };

            self.add_file(captures[2].to_string(), size);
            Ok(())
        } else if let Some(captures) = self.dir_re.captures(command) {
            self.add_directory(captures[1].to_string());
            Ok(())
        } else if let Some(captures) = self.cd_re.captures(command) {
            self.cd_to_child(&captures[1])
        } else {
            Err(format!("Invalid command: {}", command))
        }
    }

    fn iter(&self) -> DirIter {
        DirIter::new(self.root_dir.clone())
    }
}

impl IntoIterator for Shell {
    type Item = <DirIter as Iterator>::Item;
    type IntoIter = DirIter;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl Drop for Shell {
    fn drop(&mut self) {
        // Use an iterator to destroy the entries instead of letting
        // rust do it. This is important if the tree is really deep
        // because otherwise rust will just do it with a recursive
        // method and this can blow the stack.
        //
        // This obviously doesn’t matter for the actual problem but it
        // is needed for example with this data:
        //
        // https://www.reddit.com/r/adventofcode/comments/zeuba6/comment/iz9sncz
        for (_, entry) in DirIter::new(self.root_dir.clone()) {
            // The iterator is depth first so whenever we encounter a
            // directory all of its child directories will already
            // have been destroyed
            if let EntryData::Directory { ref mut child } =
                entry.borrow_mut().data {
                *child = None;
            }
        }
    }
}

fn main() -> std::process::ExitCode {
    let mut shell = Shell::new();
    let mut exit_code = std::process::ExitCode::SUCCESS;

    for (line_num, result) in std::io::stdin().lines().enumerate() {
        let line = match result {
            Err(e) => {
                eprintln!("{}", e);
                return std::process::ExitCode::FAILURE;
            },
            Ok(line) => line,
        };

        if let Err(e) = shell.run_command(&line) {
            eprintln!("line {}: {}", line_num + 1, e);
            exit_code = std::process::ExitCode::FAILURE;
        }
    }

    let part1: usize = shell
        .iter()
        .filter_map(|(size, entry)|
                    if (matches!(entry.borrow().data,
                                 EntryData::Directory { .. }) &&
                        size <= 100_000) {
                        Some(size)
                    } else {
                        None
                    })
        .sum();

    println!("part 1: {}", part1);

    const SPACE_NEEDED: usize = 30_000_000;
    const DISK_SIZE: usize = 70_000_000;

    // The iterator will report the root directory last and that will
    // have the total size
    let total_used = shell.iter().last().unwrap().0;

    let space_free = DISK_SIZE - total_used;

    let part2 = shell
        .iter()
        .filter_map(|(size, entry)|
                    if (matches!(entry.borrow().data,
                                 EntryData::Directory { .. }) &&
                        space_free + size >= SPACE_NEEDED) {
                        Some(size)
                    } else {
                        None
                    })
        // This should never be None because at worst it will just
        // pick the / directory which is guaranteed to reduce the
        // space used to 0
        .min().unwrap();

    println!("part 2: {}", part2);

    exit_code
}
