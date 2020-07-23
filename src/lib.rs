#![allow(dead_code)]
mod solver;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn answers() {
        assert_eq!(solver::all_answers(4, 2, false), vec![[0, 1], [0, 2], [0, 3], [1, 0], [1, 2], [1, 3], [2, 0], [2, 1], [2, 3], [3, 0], [3, 1], [3, 2]]);
        assert_eq!(solver::all_answers(3, 2, true), vec![[0, 0], [0, 1], [0, 2], [1, 0], [1, 1], [1, 2], [2, 0], [2, 1], [2, 2]]);
    }

    #[test]
    fn eval() {
        assert_eq!(solver::evaluate_answer(&[2, 4, 5, 0], &[2, 3, 4, 5], 6), (1, 2));
        assert_eq!(solver::evaluate_answer(&[2, 3, 4, 5], &[3, 4, 5, 2], 6), (0, 4));
        assert_eq!(solver::evaluate_answer(&[2, 2, 3, 3], &[0, 1, 2, 3], 6), (1, 1));
    }
}
