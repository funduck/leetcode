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
        let s = x.to_string();
        let len = s.len() / 2;
        let (head, tail) = s.split_at(len);
        head.chars().eq(tail.chars().rev())
    }
}
