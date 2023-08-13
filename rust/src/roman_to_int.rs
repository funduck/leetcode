use crate::Solution;

// Singleton HashMap
// use std::collections::HashMap;
// static mut ROMAN_HASH_MAP: Option<HashMap<char, i32>> = None;
// fn roman_char_to_int(c: &char) -> i32 {
//     *unsafe {
//         ROMAN_HASH_MAP
//             .get_or_insert(HashMap::from([
//                 ('I', 1),
//                 ('V', 5),
//                 ('X', 10),
//                 ('L', 50),
//                 ('C', 100),
//                 ('D', 500),
//                 ('M', 1000),
//             ]))
//             .get(c)
//             .unwrap()
//     }
// }

// Fast
fn roman_char_to_int(c: &char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => panic!("Unexpected roman char {c}"),
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars()
            .map(|c| roman_char_to_int(&c))
            .fold((None, 0), |(prev, sum), cur| {
                (
                    Some(cur),
                    match prev {
                        Some(prev) if prev < cur => sum - 2 * prev + cur,
                        _ => sum + cur,
                    },
                )
            })
            .1
    }
}
