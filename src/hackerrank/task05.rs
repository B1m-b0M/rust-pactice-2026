// https://www.hackerrank.com/challenges/apple-and-orange/problem
pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: Vec<i32>,
    oranges: Vec<i32>,
) -> (i32, i32) {
    let apple_count = apples
        .iter()
        .map(|d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    let orange_count = oranges
        .iter()
        .map(|d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apples_oranges() {
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        assert_eq!(
            count_apples_and_oranges(7, 11, 5, 15, apples, oranges),
            (1, 1)
        );
    }
}
