// https://www.hackerrank.com/challenges/kangaroo/problem
pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> bool {
    if v1 <= v2 {
        return false;
    }

    (x2 - x1) % (v1 - v2) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo() {
        assert_eq!(kangaroo(0, 3, 4, 2), true);
    }
}
