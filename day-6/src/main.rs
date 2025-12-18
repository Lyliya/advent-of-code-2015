use std::{io, usize};
use std::io::Read;
use std::collections::HashMap;
use timer;
use regex::Regex;

fn has_double(s: &str) -> bool {
    s.as_bytes()
     .windows(2)
     .any(|w| w[0] == w[1])
}

fn check_is_nice_step1(line: &str) -> bool {
    let twice = has_double(line);
    // ab, cd, pq, or xy
    let forbidden = Regex::new(r"(?:ab)|(?:cd)|(?:pq)|(?:xy)").unwrap();
    let vowels = Regex::new(r"a|e|i|o|u").unwrap();

    let vowels_count: Vec<_> = vowels.find_iter(line).collect();

    twice && !forbidden.is_match(line) && vowels_count.len() >= 3
}

fn step1(lines: &[&str]) -> usize {
    let mut answer = 0;

    for line in lines {
        if check_is_nice_step1(line) {
            answer += 1;
        }
    }

    answer
}

fn has_repeated_pair(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut seen: HashMap<(u8, u8), usize> = HashMap::new();

    for i in 0..bytes.len() - 1 {
        let pair = (bytes[i], bytes[i + 1]);

        if let Some(&prev) = seen.get(&pair) {
            if i - prev >= 2 {
                return true;
            }
        } else {
            seen.insert(pair, i);
        }
    }
    false
}

fn has_repeat_with_gap(s: &str) -> bool {
    let bytes = s.as_bytes();

    bytes
        .windows(3)
        .any(|w| w[0] == w[2])
}

fn step2(lines: &[&str]) -> usize {
    let mut answer = 0;

    for line in lines {
        if has_repeat_with_gap(line) && has_repeated_pair(line) {
            answer += 1;
        }
    }

    answer
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");
    let lines: Vec<&str> = input.lines().collect();

    let (step1_answer, step1_time) = timer::measure(|| step1(&lines));
    println!("Step 1 answer: {}, in {:?}", step1_answer, step1_time);

    let (step2_answer, step2_time) = timer::measure(|| step2(&lines));
    println!("Step 2 answer: {}, in {:?}", step2_answer, step2_time);
}
