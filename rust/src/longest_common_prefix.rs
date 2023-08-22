use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("");
        }
        if strs.len() == 1 {
            return strs[0].clone();
        }

        let mut iters = strs[1..].iter().map(|s| s.chars()).collect::<Vec<_>>();

        for (i, c) in strs[0].chars().enumerate() {
            for iter in iters.iter_mut() {
                if iter.next() != Some(c) {
                    return strs[0][..i].to_string();
                }
            }
        }

        return strs[0].clone();
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn longest_common_prefix() {
        for (input, expected) in [(["flowers", "flower", "flowder"], "flow")] {
            let res = Solution::longest_common_prefix(Vec::from(
                input
                    .iter()
                    .map(|s| String::from(*s))
                    .collect::<Vec<String>>(),
            ));
            assert_eq!(res, expected);
        }
    }
}
