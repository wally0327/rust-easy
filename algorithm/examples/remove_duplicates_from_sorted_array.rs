struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = i + 1;
        let mut k = 1;
        while i < nums.len() && j < nums.len() {
            let t = nums[i];
            while j < nums.len() {
                let n = nums[j];
                if n <= t {
                    j = j + 1;
                } else {
                    break;
                }
            }
            if j < nums.len() && i + 1 < nums.len() {
                k = k + 1;
                nums[i + 1] = nums[j];
            }

            i = i + 1;
            j = j + 1;
        }
        k
    }
}

pub fn main() {
    let mut input = vec![1, 1];
    println!("ans：{}", Solution::remove_duplicates(&mut input));
    println!("ans：{:?}", input);
}
