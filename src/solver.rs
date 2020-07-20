use itertools::Itertools;

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

pub fn possible_hits_blows(answer_len: usize) -> Vec<(usize, usize)> {
    (0..=answer_len)
        .flat_map(|x| (0..=x).map(move |y| (y, x - y)))
        .filter(|x| *x != (answer_len - 1, 1))
        .collect()
}
