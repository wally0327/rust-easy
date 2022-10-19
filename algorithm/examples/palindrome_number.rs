impl Solution {
    // 构建反向数字
    // 回文数
    pub fn is_palindrome(x: i32) -> bool {
        let res = if x < 0 {
            false
        } else {
            let mut x2 = 0;
            let mut y = x;
            while y > 0 {
                x2 = x2 * 10 + y % 10;
                y = y / 10;
            }
            println!("x: {}, y:{}", x, x2);
            x == x2
        };
        return res;
    }

    // 直接使用字符串头尾比对
    pub fn is_palindrome_2(x: i32) -> bool {
        let str_num = x.to_string();
        let mut i = 0;
        let mut j = str_num.len() - 1;

        while i < j {
            if str_num.get(i..i + 1).unwrap() != str_num.get(j..j + 1).unwrap() {
                return false;
            }
            i = i + 1;
            j = j - 1;
        }

        return true;
    }
}

struct Solution {}

fn main() {
    let num = -121;
    println!("{:?}", Solution::is_palindrome(num));
}
