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
}
