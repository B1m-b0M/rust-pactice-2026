// https://www.hackerrank.com/challenges/sock-merchant/problem
use std::collections::HashMap;

pub fn sock_merchant(_n: i32, ar: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut pairs = 0;

    for sock in ar {
        let entry = map.entry(sock).or_insert(0);
        *entry += 1;

        if *entry == 2 {
            pairs += 1;
            *entry = 0;
        }
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock() {
        let socks = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(9, socks), 3);
    }
}
