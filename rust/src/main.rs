mod is_palindrome;
mod longest_common_prefix;
mod remove_duplicates;
mod roman_to_int;

struct Solution {}

fn vec_to_string(v: &[i32]) -> String {
    let s = v
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    String::from("[") + &s + &String::from("]")
}

fn main() {
    for v in [[1, 2, 3], [1, 2, 1], [2, 2, 1], [1, 1, 2]] {
        let mut arg: Vec<i32> = Vec::from(v);
        let before = vec_to_string(arg.as_slice());
        let uniqs = Solution::remove_duplicates(&mut arg);
        let after = vec_to_string(arg.as_slice());

        println!("{} {} {}", before, uniqs, after);
    }
}
