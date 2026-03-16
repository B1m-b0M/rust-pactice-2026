// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem
pub fn breaking_records(scores: Vec<i32>) -> (i32, i32) {
    let mut max = scores[0];
    let mut min = scores[0];

    let mut max_break = 0;
    let mut min_break = 0;

    for s in scores.iter().skip(1) {
        if *s > max {
            max = *s;
            max_break += 1;
        }

        if *s < min {
            min = *s;
            min_break += 1;
        }
    }

    (max_break, min_break)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_records() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(scores), (2, 4));
    }
}
