use std::io;
use std::io::Read;
use timer;

fn step1(line: &str) -> i64 {
    let mut answer = 0;

    for c in line.trim().chars() {
        match c {
            '(' => answer += 1,
            ')' => answer -= 1,
            _ => panic!("Invalid input"),
        }
    }

    answer
}

fn step2(line: &str) -> usize {
    let mut answer = 0;

    for (i, c) in line.trim().chars().enumerate() {
        match c {
            '(' => answer += 1,
            ')' => answer -= 1,
            _ => panic!("Invalid input"),
        }
        if answer == -1 {
            return i + 1;
        }
    }

    panic!("Never reach basement")
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let (step1_answer, step1_time) = timer::measure(|| step1(&input));
    println!("Step 1 answer: {}, in {:?}", step1_answer, step1_time);

    let (step2_answer, step2_time) = timer::measure(|| step2(&input));
    println!("Step 2 answer: {}, in {:?}", step2_answer, step2_time);
}
