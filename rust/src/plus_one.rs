use crate::Solution;

impl Solution {
    // pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    //     fn add_one(digits: &mut Vec<i32>, pos: usize) -> i32 {
    //         digits[pos] += 1;
    //         if digits[pos] == 10 {
    //             digits[pos] = 0;
    //             if pos > 0 {
    //                 return add_one(digits, pos - 1);
    //             }
    //             return 1;
    //         }
    //         return 0;
    //     }

    //     let end_pos = digits.len() - 1;
    //     if add_one(&mut digits, end_pos) != 0 {
    //         digits.insert(0, 1);
    //     }
    //     return digits;
    // }
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut pos = digits.len();
        let mut rest = 1;
        while pos > 0 {
            pos -= 1;
            digits[pos] += 1;
            if digits[pos] < 10 {
                rest = 0;
                break;
            }
            digits[pos] = 0;
            rest = 1;
        }
        if rest != 0 {
            digits.insert(0, 1);
        }
        return digits;
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn plus_one() {
        for (input, expected) in [
            (
                Vec::from([1, 2, 2, 3, 4, 5, 5, 6]),
                Vec::from([1, 2, 2, 3, 4, 5, 5, 7]),
            ),
            (
                Vec::from([1, 2, 2, 3, 4, 5, 9, 9]),
                Vec::from([1, 2, 2, 3, 4, 6, 0, 0]),
            ),
            (Vec::from([9, 9]), Vec::from([1, 0, 0])),
        ] {
            let res = Solution::plus_one(input);
            assert_eq!(res, Vec::from(expected));
        }
    }
}
