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
