// use std::cmp::Ordering;

use crate::Solution;

impl Solution {
    pub fn merge_sorted_array(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i1 = m as usize;
        let mut i2 = n as usize;
        while i2 > 0 {
            if i1 == 0 || nums2[i2 - 1] > nums1[i1 - 1] {
                nums1[i1 + i2 - 1] = nums2[i2 - 1];
                i2 -= 1;
            } else {
                nums1[i1 + i2 - 1] = nums1[i1 - 1];
                i1 -= 1;
            }
        }
    }

    // pub fn merge_sorted_array(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    //     let mut i1 = m as usize;
    //     let mut i2 = n as usize;

    //     for k in (0..(m + n) as usize).rev() {
    //         let n1 = if i1 > 0 { Some(nums1[i1 - 1]) } else { None };
    //         let n2 = if i2 > 0 { Some(nums2[i2 - 1]) } else { None };

    //         match n1.cmp(&n2) {
    //             Ordering::Less => {
    //                 nums1[k] = nums2[i2 - 1];
    //                 i2 -= 1;
    //             }
    //             Ordering::Equal | Ordering::Greater => {
    //                 nums1[k] = nums1[i1 - 1];
    //                 i1 -= 1;
    //             }
    //         };
    //     }
    // }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn merge_sorted_array() {
        for (input1, input2, expected) in [([2, 2, 4, 0, 0, 0], [1, 3, 5], [1, 2, 2, 3, 4, 5])] {
            let m = input1.len() as i32;
            let n = input2.len() as i32;
            let mut input: Vec<i32> = Vec::from(input1);
            let mut other: Vec<i32> = Vec::from(input2);

            Solution::merge_sorted_array(&mut input, m - n, &mut other, n);

            assert_eq!(input, Vec::from(expected))
        }
    }
}
