use std::collections::HashSet;
use std::iter::Extend;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Point, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(\d+),(\d+)"
            ).unwrap();
        }

        let captures = RE.captures(s).unwrap();

        Ok(Point {
            x: captures[1].parse().unwrap(),
            y: captures[2].parse().unwrap()
        })
    }
}

#[derive(Debug)]
struct Paper {
    points: HashSet<Point>,
}

#[derive(Debug, Copy, Clone)]
enum Fold {
    Left(i32),
    Up(i32),
}

impl Paper {
    fn fold_x(&mut self, x: i32) {
        let mut points_to_add = Vec::new();

        self.points.retain(|&point| {
            if point.x == x {
                false
            } else if point.x < x {
                true
            } else {
                points_to_add.push(Point { x: 2 * x - point.x, y: point.y });
                false
            }
        });

        self.points.extend(points_to_add);
    }

    fn fold_y(&mut self, y: i32) {
        let mut points_to_add = Vec::new();

        self.points.retain(|&point| {
            if point.y == y {
                false
            } else if point.y < y {
                true
            } else {
                points_to_add.push(Point { x: point.x, y: 2 * y - point.y });
                false
            }
        });

        self.points.extend(points_to_add);
    }

    fn fold(&mut self, fold: Fold) {
        match fold {
            Fold::Left(x) => self.fold_x(x),
            Fold::Up(y) => self.fold_y(y),
        }
    }
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Fold, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^fold along ([xy])=(-?\d+)$"
            ).unwrap();
        }

        let captures = RE.captures(s).unwrap();
        let coord = captures[2].parse().unwrap();
        match &captures[1] {
            "x" => Ok(Fold::Left(coord)),
            "y" => Ok(Fold::Up(coord)),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut paper = Paper { points: HashSet::new() };
    let mut folds = Vec::<Fold>::new();
    let mut in_folds = false;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        if line.len() == 0 {
            in_folds = true;
        } else if in_folds {
            folds.push(line.parse().unwrap());
        } else {
            paper.points.insert(line.parse().unwrap());
        }
    }

    for (i, fold) in folds.into_iter().enumerate() {
        paper.fold(fold);

        if i == 0 {
            println!("part 1: {}", paper.points.len());
        }
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for &point in paper.points.iter() {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Point { x, y };
            let ch = if paper.points.contains(&point) {
                '#'
            } else {
                ' '
            };
            print!("{}", ch);
        }
        println!("");
    }
}
