mod longest_common_prefix;
mod roman_to_int;

struct Solution {}

fn main() {
    for v in [
        "MDCLXVI", "CMDLXVI", "MCDLXVI", "MDXCLVI", "MDCLIX", "MDCLXIV", "III", "IV", "XVII",
    ] {
        println!("{} {}", v, Solution::roman_to_int(v.to_string()));
    }
}
