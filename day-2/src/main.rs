use std::io;
use std::io::Read;
use timer;

fn compute_paper(l: usize, w: usize, h: usize) -> usize {
    let area_l = l * w;
    let area_w = w * h;
    let area_h = h * l;

    let min = *[area_l, area_w, area_h].iter().min().unwrap();

    2 * area_l + 2 * area_w + 2 * area_h + min
}

fn step1(lines: &[&str]) -> usize {
    let mut answer = 0;

    for line in lines {
        let splitted: Vec<usize> = line
            .split('x')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        let l = splitted[0];
        let w = splitted[1];
        let h = splitted[2];

        answer += compute_paper(l, w, h);
    }

    answer
}

fn compute_ribbon(l: usize, w: usize, h: usize) -> usize {
    let mut size = vec![l, w, h];
    size.sort();

    size[0] * 2 + size[1] * 2 + l * w * h
}

fn step2(lines: &[&str]) -> usize {
    let mut answer = 0;

    for line in lines {
        let splitted: Vec<usize> = line
            .split('x')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        let l = splitted[0];
        let w = splitted[1];
        let h = splitted[2];

        answer += compute_ribbon(l, w, h);
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
