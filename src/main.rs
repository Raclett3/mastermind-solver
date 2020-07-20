extern crate itertools;
mod solver;

use itertools::Itertools;
use solver::*;
use std::collections::HashMap;
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
    let colors = prompt_usize("How many kinds of colors does answer have?:");
    let answer_len = prompt_usize("How many colors does answer have?:");
    let repetition = loop {
        let mut buffer = String::new();
        println!("Do you allow repetition?(y/N):");
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

    let answers = all_answers(colors, answer_len, repetition);
    let mut filtered: Vec<_> = answers.iter().collect();
    let all_hits_blows = possible_hits_blows(answer_len);
    while filtered.len() > 1 {
        let (next, _) = answers
            .iter()
            .map(|answer| {
                let mut cnt = HashMap::new();
                for x in &all_hits_blows {
                    cnt.insert(*x, 0);
                }
                for actual in &filtered {
                    let hits_blows = evaluate_answer(answer, actual, colors);
                    cnt.insert(hits_blows, cnt.get(&hits_blows).unwrap() + 1);
                }
                (answer, cnt.iter().map(|(_, x)| *x).max().unwrap())
            })
            .fold1(|acc, x| if acc.1 > x.1 { x } else { acc })
            .unwrap();

        println!("{}", next.iter().map(|x| x.to_string()).join(" "));
        let hits_blows = loop {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            let nums: Vec<usize> = buffer.trim().split(' ').flat_map(str::parse).collect();
            if nums.len() != 2 {
                continue;
            }
            break (nums[0], nums[1]);
        };

        filtered = filtered
            .iter()
            .filter(|x| evaluate_answer(x, next, colors) == hits_blows)
            .copied()
            .collect();
    }
    if filtered.len() == 1 {
        println!("{}", filtered[0].iter().map(|x| x.to_string()).join(" "));
    }
}
