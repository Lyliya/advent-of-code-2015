use std::collections::HashSet;
use std::io;
use std::io::Read;
use timer;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn step1(line: &str) -> usize {
    let mut start = Point { x: 0, y: 0 };
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(start);

    for c in line.chars() {
        match c {
            '^' => {
                start.y -= 1;
                visited.insert(start);
            }
            'v' => {
                start.y += 1;
                visited.insert(start);
            }
            '>' => {
                start.x += 1;
                visited.insert(start);
            }
            '<' => {
                start.x -= 1;
                visited.insert(start);
            }
            _ => panic!("Invalid direction"),
        }
    }

    visited.len()
}

fn handle_move(c: char, start: &mut Point, visited: &mut HashSet<Point>) {
    match c {
        '^' => {
            start.y -= 1;
            visited.insert(*start);
        }
        'v' => {
            start.y += 1;
            visited.insert(*start);
        }
        '>' => {
            start.x += 1;
            visited.insert(*start);
        }
        '<' => {
            start.x -= 1;
            visited.insert(*start);
        }
        _ => panic!("Invalid direction"),
    }
}

fn step2(line: &str) -> usize {
    let mut start_santa = Point { x: 0, y: 0 };
    let mut start_robot = Point { x: 0, y: 0 };
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(start_santa);

    for (i, c) in line.chars().enumerate() {
        if i % 2 == 0 {
            handle_move(c, &mut start_santa, &mut visited);
        } else {
            handle_move(c, &mut start_robot, &mut visited);
        }
    }

    visited.len()
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let (step1_answer, step1_time) = timer::measure(|| step1(&input.trim()));
    println!("Step 1 answer: {}, in {:?}", step1_answer, step1_time);

    let (step2_answer, step2_time) = timer::measure(|| step2(&input.trim()));
    println!("Step 2 answer: {}, in {:?}", step2_answer, step2_time);
}
