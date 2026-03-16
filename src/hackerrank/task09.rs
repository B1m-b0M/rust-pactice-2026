// https://www.hackerrank.com/challenges/migratory-birds/problem
use std::collections::HashMap;

pub fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for bird in arr {
        *map.entry(bird).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut result = i32::MAX;

    for (id, count) in map {
        if count > max_count || (count == max_count && id < result) {
            max_count = count;
            result = id;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birds() {
        let birds = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(birds), 4);
    }
}
