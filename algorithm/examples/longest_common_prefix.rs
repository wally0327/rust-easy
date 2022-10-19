impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut lcp = 0; // 记录一下公共长度的位置即可，节省内存占用
        let mut shortest_str = strs.get(0).unwrap();
        for s in &strs {
            if s.len() < shortest_str.len() {
                shortest_str = s;
            }
        }
        let mut i = 0;
        while i < shortest_str.len() {
            let c = shortest_str.as_bytes().get(i).unwrap();
            let mut is_common = true;
            let mut j = 0;
            while j < strs.len() {
                let cc: &u8 = strs.get(j).unwrap().as_bytes().get(i).unwrap();
                if c != cc {
                    is_common = false;
                    return shortest_str.get(0..lcp).unwrap().to_string();
                }
                j = j + 1;
            }
            if is_common {
                lcp = lcp + 1;
            }
            i = i + 1;
        }
        return shortest_str.get(0..lcp).unwrap().to_string();
    }
}

struct Solution {}

fn main() {
    let strs = vec!["dog".to_string(), "racer".to_string(), "car".to_string()];
    println!("{:?}", Solution::longest_common_prefix(strs));
}
