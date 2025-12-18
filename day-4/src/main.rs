use std::{io, usize};
use std::io::Read;
use timer;
use md5;

fn step1(key: &str) -> usize {
    for i in 0..usize::MAX {
        let d = md5::compute(format!("{key}{i}"));
        if d[0] == 0 && d[1] == 0 && (d[2] & 0xF0) == 0 {
            return i;
        }
    }

    panic!("No match found")
}

fn step2(key: &str) -> usize {
    for i in 0..usize::MAX {
        let d = md5::compute(format!("{key}{i}"));
        if d[0] == 0 && d[1] == 0 && d[2] == 0 {
            return i;
        }
    }

    panic!("No match found")
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
