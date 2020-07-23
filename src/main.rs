extern crate itertools;
mod solver;

use itertools::Itertools;
use solver::Solver;
use std::io::stdin;

fn prompt_usize(prompt: &str) -> usize {
    loop {
        let mut buffer = String::new();
        println!("{}", prompt);
        stdin().read_line(&mut buffer).unwrap();
        let length = buffer.len();
        if let Ok(colors) = buffer[0..length - 1].parse::<usize>() {
            return colors;
        }
    }
}

fn main() {
    println!("Mastermind solver");
    let colors = prompt_usize("How many colors of pegs are there?:");
    let answer_len = prompt_usize("How many pegs does the answer have?:");
    let repetition = loop {
        let mut buffer = String::new();
        println!("Could the answer have pegs colored the same?(y/N):");
        stdin().read_line(&mut buffer).unwrap();
        break match buffer.chars().next().unwrap() {
            'Y' | 'y' => true,
            'N' | 'n' | '\n' => false,
            _ => continue,
        };
    };
    if answer_len > colors && !repetition {
        eprintln!("Length of the answer should be larger than kinds of colors if the answer has no color repetition");
        std::process::exit(1);
    }

    let mut solver = Solver::new(colors, answer_len, repetition);
    while !solver.solved() {
        let next = solver.next_answer();

        println!("The nest guess is: {}", next.iter().map(|x| x.to_string()).join(" "));
        println!("How many hits/blows are there with this guess? (Input space-separated value):");
        let hits_blows = loop {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            let nums: Vec<usize> = buffer.trim().split(' ').flat_map(str::parse).collect();
            if nums.len() != 2 {
                continue;
            }
            break (nums[0], nums[1]);
        };
        solver.process_answer(&next, hits_blows);
    }

    if let Some(answer) = solver.answer() {
        println!("The answer is: {}", answer.iter().map(|x| x.to_string()).join(" "));
    } else {
        println!("Some condition conflicts: there's no answer.");
    }
}
