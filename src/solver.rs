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
