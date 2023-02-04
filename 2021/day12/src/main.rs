use regex::Regex;
use std::collections::{HashSet, HashMap};
use std::collections::hash_set;
use std::process::ExitCode;

#[derive(Clone, Copy, Debug)]
enum CaveSize {
    Endpoint,
    Small,
    Big,
}

impl CaveSize {
    fn max_visits(self, part2: bool) -> usize {
        match self {
            CaveSize::Endpoint => 1,
            CaveSize::Small => if part2 { 2 } else { 1 },
            CaveSize::Big => usize::MAX,
        }
    }
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
                let size = if name == "start" {
                    self.start = index;
                    CaveSize::Endpoint
                } else if name == "end" {
                    self.end = index;
                    CaveSize::Endpoint
                } else {
                    match name.chars().next() {
                        Some(ch) if ch.is_uppercase() => CaveSize::Big,
                        _ => CaveSize::Small,
                    }
                };

                self.caves.push(Cave {
                    name: name.to_owned(),
                    size,
                    links: HashSet::new(),
                });

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
    part2: bool,
    stack: Vec<StackEntry<'a>>,
}

struct StackEntry<'a> {
    cave: usize,
    links: hash_set::Iter<'a, usize>,
}

impl<'a> Searcher<'a> {
    fn new<'m> (map: &'m Map, part2: bool) -> Searcher<'m> {
        Searcher {
            map,
            part2,
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
                    let max_visits = self.map.caves[e.cave].size.max_visits(self.part2);
                    let visits = self.stack.iter().filter(|o| o.cave == e.cave).count();
                    if visits >= max_visits {
                        continue 'outer;
                    }
                    self.stack.push(e);
                    self.backtrack();
                }
            }
        }
    }
}

fn count_routes(map: &Map, part2: bool) -> usize {
    let mut count = 0usize;

    for solution in Searcher::new(&map, part2) {
        println!("{:?}", solution.into_iter().map(|c| &map.caves[c].name).collect::<Vec<&String>>());
        count += 1;
    }

    count
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

    println!("== part 1");
    let part1 = count_routes(&map, false);
    println!("== part 2");
    let part2 = count_routes(&map, true);

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);

    ExitCode::SUCCESS
}
