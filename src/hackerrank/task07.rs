// https://www.hackerrank.com/challenges/between-two-sets/problem
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

pub fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let l = a.iter().copied().reduce(lcm).unwrap();
    let g = b.iter().copied().reduce(gcd).unwrap();

    let mut count = 0;
    let mut multiple = l;

    while multiple <= g {
        if g % multiple == 0 {
            count += 1;
        }
        multiple += l;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between_sets() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];

        assert_eq!(get_total_x(a, b), 3);
    }
}
