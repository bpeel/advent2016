use regex::Regex;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct Entry {
    name: String,
    parent: Option<Weak<RefCell<Entry>>>,
    data: EntryData,
}

#[derive(Debug, Clone)]
enum EntryData {
    File { size: usize },
    Directory { children: Vec<Rc<RefCell<Entry>>> },
}

#[derive(Debug, Clone)]
struct Shell {
    root_dir: Rc<RefCell<Entry>>,
    cwd: Rc<RefCell<Entry>>,

    file_re: Regex,
    dir_re: Regex,
    cd_re: Regex,
}

impl Entry {
    fn new_directory(name: String) -> Entry {
        Entry {
            name,
            parent: None,
            data: EntryData::Directory {
                children: Vec::new(),
            },
        }
    }

    fn new_file(name: String, size: usize) -> Entry {
        Entry {
            name,
            parent: None,
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
        let children = match cwd_ref.data {
            EntryData::Directory { ref children } => children.iter(),
            _ => panic!("cwd is not a directory!"),
        };

        for child in children {
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
        }

        Err("Tried to change directory into a file that doesn’t exist"
            .to_string())
    }

    fn add_entry(&mut self, child: Entry) {
        let mut child_mut = child;

        child_mut.parent = Some(Rc::downgrade(&self.cwd));

        let mut parent = self.cwd.borrow_mut();

        let children = match parent.data {
            EntryData::Directory { ref mut children } => children,
            EntryData::File { .. } =>
                panic!("Tried to add a child entry to a file entry"),
        };

        children.push(Rc::new(RefCell::new(child_mut)));
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
}

fn main() -> std::process::ExitCode {
    let mut shell = Shell::new();

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
        }
    }

    std::process::ExitCode::SUCCESS
}
