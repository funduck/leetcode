use crate::Solution;

// impl Solution {
//     pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//         let mut prev: Option<&i32> = None;
//         let mut sl = nums
//             .iter()
//             .filter(|x| {
//                 if prev.is_none() || prev != Some(x) {
//                     prev = Some(x);
//                     true
//                 } else {
//                     false
//                 }
//             })
//             .map(|x| *x)
//             .collect::<Vec<i32>>();
//         nums.truncate(sl.len());
//         nums.swap_with_slice(sl.as_mut_slice());
//         nums.len() as i32
//     }
// }

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn remove_duplicates() {
        for (input, expected, expected_output) in
            [([1, 2, 2, 3, 4, 5, 5, 6], [1, 2, 3, 4, 5, 6], 6)]
        {
            let mut v = Vec::from(input);
            let res = Solution::remove_duplicates(&mut v);
            assert_eq!(res, expected_output);
            assert_eq!(v, Vec::from(expected))
        }
    }
}
