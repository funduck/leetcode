// const ROMAN: [char; 7] = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
// const VALUE: [i32; 7] = [1, 5, 10, 50, 100, 500, 1000];

// pub fn roman_to_int(s: String) -> i32 {
//     s.chars()
//         .map(|c| {
//             ROMAN
//                 .iter()
//                 .enumerate()
//                 .find(|(_, ch)| ch.eq_ignore_ascii_case(&c))
//                 .unwrap()
//         })
//         .map(|(i, _)| (i, VALUE[i], VALUE[i]))
//         .reduce(|(prev, prev_val, sum), (cur, val, _)| {
//             (
//                 cur,
//                 val,
//                 if prev < cur {
//                     sum - 2 * prev_val + val
//                 } else {
//                     sum + val
//                 },
//             )
//         })
//         .unwrap()
//         .2
// }

use std::collections::HashMap;

use crate::Solution;

// How to make single global instance of a map?
fn roman_hash_map() -> HashMap<char, i32> {
    HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ])
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map = roman_hash_map();
        s.chars()
            .map(|c| {
                let val = map.get(&c).unwrap();
                (*val, *val)
            })
            .reduce(|(prev, sum), (cur, _)| {
                (
                    cur,
                    if prev < cur {
                        sum - 2 * prev + cur
                    } else {
                        sum + cur
                    },
                )
            })
            .unwrap()
            .1
    }
}
