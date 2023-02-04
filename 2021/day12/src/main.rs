use regex::Regex;
use std::collections::{HashSet, HashMap};
use std::collections::hash_set;
use std::process::ExitCode;

#[derive(Clone, Copy, Debug)]
enum CaveSize {
    Small,
    Big,
}

#[derive(Debug)]
struct Cave {
    name: String,
    size: CaveSize,
    links: HashSet<usize>,
}

#[derive(Debug)]
struct Map {
    cave_names: HashMap<String, usize>,
    caves: Vec<Cave>,
    start: usize,
    end: usize,
    line_re: Regex,
}

impl Map {
    fn new() -> Map {
        Map {
            cave_names: HashMap::new(),
            caves: Vec::new(),
            start: 0,
            end: 0,
            line_re: Regex::new("^(\\w+)-(\\w+)$").unwrap(),
        }
    }

    fn cave_index(&mut self, name: &str) -> usize {
        match self.cave_names.get(name) {
            None => {
                let index = self.caves.len();

                self.cave_names.insert(name.to_owned(), index);
                self.caves.push(Cave {
                    name: name.to_owned(),
                    size: match name.chars().next() {
                        Some(ch) if ch.is_uppercase() => CaveSize::Big,
                        _ => CaveSize::Small,
                    },
                    links: HashSet::new(),
                });

                if name == "start" {
                    self.start = index;
                } else if name == "end" {
                    self.end = index;
                }

                index
            },
            Some(&index) => index,
        }
    }

    fn add_link(&mut self, a: &str, b: &str) {
        let a = self.cave_index(a);
        let b = self.cave_index(b);
        self.caves[a].links.insert(b);
        self.caves[b].links.insert(a);
    }

    fn add_line(&mut self, line: &str) -> Result<(), ()> {
        let captures = match self.line_re.captures(line) {
            None => return Err(()),
            Some(c) => c,
        };

        self.add_link(&captures[1], &captures[2]);
        
        Ok(())
    }
}

struct Searcher<'a> {
    map: &'a Map,
    stack: Vec<StackEntry<'a>>,
}

struct StackEntry<'a> {
    cave: usize,
    links: hash_set::Iter<'a, usize>,
}

impl<'a> Searcher<'a> {
    fn new<'m> (map: &'m Map) -> Searcher<'m> {
        Searcher {
            map,
            stack: vec![StackEntry::<'m> {
                cave: map.start,
                links: map.caves[map.start].links.iter(),
            }],
        }
    }

    fn backtrack(&mut self) {
        loop {
            let mut entry = match self.stack.pop() {
                None => break,
                Some(e) => e,
            };

            if let Some(&cave) = entry.links.next() {
                self.stack.push(entry);

                self.stack.push(StackEntry {
                    cave,
                    links: self.map.caves[cave].links.iter(),
                });

                break;
            }
        }
    }
}

impl<'a> Iterator for Searcher<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            match self.stack.pop() {
                None => break None,
                Some(StackEntry { cave, .. }) if cave == self.map.end => {
                    let mut route = self.stack.iter().map(|e| e.cave).collect::<Vec<usize>>();
                    route.push(self.map.end);
                    self.backtrack();
                    break Some(route);
                },
                Some(e) => {
                    if let CaveSize::Small = self.map.caves[e.cave].size {
                        for o in self.stack[0..self.stack.len()].iter() {
                            if o.cave == e.cave {
                                self.backtrack();
                                continue 'outer;
                            }
                        }
                    }
                    self.stack.push(e);
                    self.backtrack();
                }
            }
        }
    }
}

fn main() -> ExitCode {
    let mut map = Map::new();

    for (line_num, line) in std::io::stdin().lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            },
        };

        if let Err(()) = map.add_line(&line) {
            eprintln!("line: {}: invalid", line_num + 1);
            return ExitCode::FAILURE;
        }
    }

    let mut count = 0usize;

    for solution in Searcher::new(&map) {
        println!("{:?}", solution.into_iter().map(|c| &map.caves[c].name).collect::<Vec<&String>>());
        count += 1;
    }

    println!("part 1: {}", count);

    ExitCode::SUCCESS
}
