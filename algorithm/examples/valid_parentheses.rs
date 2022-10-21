struct Solution {}

impl Solution {
    pub fn is_valid_(s: String) -> bool {
        let mut stack = Vec::new();
        let mut i = 0;
        while i < s.len() {
            let c = s.get(i..i + 1).unwrap();
            if Self::should_insert(c) {
                stack.push(c);
            } else {
                if stack.is_empty() {
                    return false;
                }
                let c_t = stack.remove(stack.len() - 1);
                if !Self::is_matched(c, c_t) {
                    return false;
                }
            }
            i = i + 1;
        }
        stack.len() == 0
    }

    pub fn should_insert(c: &str) -> bool {
        match c {
            "(" | "{" | "[" => true,
            _ => false,
        }
    }

    pub fn is_matched(c: &str, t: &str) -> bool {
        match c {
            ")" => t == "(",
            "}" => t == "{",
            "]" => t == "[",
            _ => false,
        }
    }
}

fn main() {
    let input = String::from("{()[[]{}}");

    println!("{:?}", Solution::is_valid(input));
}
