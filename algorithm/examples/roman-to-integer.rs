impl Solution {
    pub fn get_value(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
        // 手动比 hash map 快不少，不用算 hash 了
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut num = 0;
        let values = s.as_bytes();
        let mut n = 0;

        while n < values.len() {
            let k1 = *values.get(n).unwrap() as char;
            let temp1 = Self::get_value(k1);
            if n + 1 >= values.len() {
                num = num + temp1;
            } else {
                let k2 = *values.get(n + 1).unwrap() as char;
                let temp2 = Self::get_value(k2);

                if temp1 >= temp2 {
                    num = num + temp1;
                } else {
                    num = num + (temp2 - temp1);
                    n = n + 1;
                }
            }
            n = n + 1;
        }

        num
    }
}

struct Solution {}

fn main() {
    let input = String::from("MCMXCIV");
    let target = 6;

    println!("{:?}", Solution::roman_to_int(input));
}
