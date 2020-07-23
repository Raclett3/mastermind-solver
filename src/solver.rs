use itertools::Itertools;
use std::collections::HashMap;

pub fn all_answers(colors: usize, len: usize, repetition: bool) -> Vec<Vec<usize>> {
    if repetition {
        std::iter::repeat(0..colors)
            .take(len)
            .multi_cartesian_product()
            .collect()
    } else {
        (0..colors).permutations(len).collect()
    }
}

pub fn evaluate_answer(answer: &[usize], actual: &[usize], colors: usize) -> (usize, usize) {
    let mut answer_colors_count = vec![0; colors];
    let mut actual_colors_count = vec![0; colors];
    let mut hits = 0;
    for (x, y) in answer.iter().zip(actual.iter()) {
        if x == y {
            hits += 1;
        } else {
            answer_colors_count[*x] += 1;
            actual_colors_count[*y] += 1;
        }
    }

    let blows = answer_colors_count
        .iter()
        .zip(actual_colors_count.iter())
        .map(|(x, y)| std::cmp::min(x, y))
        .sum();
    (hits, blows)
}

pub struct Solver {
    all_answers: Vec<Vec<usize>>,
    possible_answers: Vec<usize>,
    colors: usize,
}

impl Solver {
    pub fn new(colors: usize, answer_len: usize, repetition: bool) -> Self {
        let answers = all_answers(colors, answer_len, repetition);
        let len = answers.len();
        Self {
            all_answers: answers,
            possible_answers: (0..len).collect(),
            colors,
        }
    }

    pub fn next_answer(&mut self) -> Vec<usize> {
        let (next, _) = self
            .all_answers
            .iter()
            .enumerate()
            .map(|(index, answer)| {
                if index % 10 == 0 {
                    println!("{}", index);
                }
                let mut cnt = HashMap::new();
                for i in &self.possible_answers {
                    let actual = &self.all_answers[*i];
                    let hits_blows = evaluate_answer(answer, actual, self.colors);
                    cnt.insert(hits_blows, cnt.get(&hits_blows).unwrap_or(&0) + 1);
                }
                (answer, cnt.iter().map(|(_, x)| *x).max().unwrap())
            })
            .fold1(|acc, x| if acc.1 > x.1 { x } else { acc })
            .unwrap();
        next.clone()
    }

    pub fn process_answer(&mut self, answer: &[usize], hits_blows: (usize, usize)) {
        self.possible_answers = self
            .possible_answers
            .iter()
            .filter(|x| evaluate_answer(&self.all_answers[**x], answer, self.colors) == hits_blows)
            .copied()
            .collect();
    }

    pub fn solved(&self) -> bool {
        self.possible_answers.len() <= 1
    }

    pub fn answer(&self) -> Option<Vec<usize>> {
        if self.possible_answers.len() == 1{
            Some(self.all_answers[self.possible_answers[0]].clone())
        } else {
            None
        }
    }
}
