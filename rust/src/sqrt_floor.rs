use std::cmp::Ordering;

use crate::Solution;

impl Solution {
    pub fn sqrt_floor(x: i32) -> i32 {
        let mut left = 0;
        // 46341 = 46340 + 1, where 46340 is max valid sqrt for i32, so +1 is good upper bound for all input
        // x/2 is good right bound for sqrt only if x > 4, so we use 3.max(x/2)
        let mut right = 46341.min(3.max(x / 2));
        let mut mid: i32;
        loop {
            mid = (right + left) / 2;
            if mid == left {
                return mid;
            }
            match (mid * mid).cmp(&x) {
                Ordering::Greater => right = mid,
                Ordering::Less => left = mid,
                _ => return mid,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn sqrt_floor() {
        for (input, expected) in [
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 2),
            (8, 2),
            (9, 3),
            (10, 3),
            (2147395599, 46339),
            (2147395600, 46340),
        ] {
            assert_eq!(
                Solution::sqrt_floor(input),
                expected,
                "input: {input} expected: {expected}"
            );
        }
    }
}
