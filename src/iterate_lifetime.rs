pub(crate) fn do_iterate() {
    let scores: Vec<i32> = vec![1, 2, 3, 4, 5];
    let score_iter = ScoresIterator { scores: &scores };
    for (i, v) in score_iter.enumerate() {
        println!("index {} value {}", i, v);
    }
}

#[derive(Debug)]
struct ScoresIterator<'iter, T> {
    scores: &'iter [T],
}

impl<'iter, T> Iterator for ScoresIterator<'iter, T> {
    type Item = &'iter T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.scores.is_empty() {
            return None;
        }
        let element = self.scores.get(0);
        self.scores = &self.scores[1..];
        element
    }
}

#[derive(Debug)]
struct MutScoresIterator<'miter, T> {
    scores: &'miter mut [T],
}

impl<'miter, T> Iterator for MutScoresIterator<'miter, T> {
    type Item = &'miter mut T;

    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        let slice: &mut &mut [T] = &mut self.scores;
        let slice: &mut [T] = std::mem::replace(slice, &mut []);
        let (first, rest) = slice.split_first_mut()?;
        self.scores = rest;
        Some(first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_iterator() {
        let scores_vec: Vec<i32> = vec![1, 2, 3, 4];
        let score_count = ScoresIterator {
            scores: &scores_vec,
        };
        for (index, score) in score_count.enumerate() {
            assert_eq!(scores_vec[index], *score);
            println!("score are   {}", score);
        }
    }

    #[test]
    fn test_mut_iterator() {
        let mut scores_vec: Vec<i32> = vec![1, 2, 3, 4];
        let score_count = MutScoresIterator {
            scores: &mut scores_vec,
        };

        for (_, score) in score_count.enumerate() {
            *score = *score + 1;
        }
        assert_eq!(scores_vec.get(0), Some(&2));
    }
}
