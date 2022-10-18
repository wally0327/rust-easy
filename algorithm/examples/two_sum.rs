impl Solution {
    // 暴力迭代
    pub fn two_sum_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let (i, j) = (0, 1);
        let max_index = nums.len();
        for i in 0..max_index {
            let temp = nums.get(i).unwrap();
            for j in (i + 1)..max_index {
                if temp + nums.get(j).unwrap() == target {
                    res.push(i as i32);
                    res.push(j as i32);
                    return res;
                }
            }
        }
        return res;
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut map = std::collections::HashMap::new();

        for i in 0..nums.len() {
            let temp = nums.get(i).unwrap();
            if map.contains_key(&&(target - temp)) {
                res.push(i as i32);
                res.push(*map.get(&&(target - temp)).unwrap() as i32);

                return res;
            }
            map.insert(temp, i);
        }

        return res;
    }
}

struct Solution {}

fn main() {
    let nums = vec![3, 3];
    let target = 6;

    println!("{:?}", Solution::two_sum(nums, target))
}
