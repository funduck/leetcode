use crate::Solution;

impl Solution {
    // pub fn is_palindrome(x: i32) -> bool {
    //     if x < 0 {
    //         return false;
    //     }
    //     let s = x.to_string();
    //     let len = s.len() / 2;
    //     let (head, tail) = s.split_at(len);
    //     head.bytes()
    //         .zip(tail.bytes().rev())
    //         .find(|(a, b)| a != b)
    //         .is_none()
    // }

    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut tmp = x;
        let mut rev = 0;
        while tmp > 0 {
            rev = rev * 10 + tmp % 10;
            tmp = (tmp - tmp % 10) / 10;
        }
        rev == x
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn is_palindrome() {
        for x in [1, 121, 1221] {
            assert!(Solution::is_palindrome(x), "{} != true", x);
        }
        for x in [12, 21, 13245] {
            assert!(!Solution::is_palindrome(x), "{} != false", x);
        }
    }
}
