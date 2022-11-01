fn main() {
    println!("hello");
}

/// #[test] 注解
/// assert! 判断条件是否为true
/// `assert_eq!` `assert_ne!` 判断是否相等
/// should_panic 检查 panic, 这是一种注解
/// cargo test 默认是并行执行所有的测试case
/// cargo test -- --ignored // 忽略部分耗时测试

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4)
}

#[test]
fn another() {
    panic!("Make this test fail")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(larger.can_hold(&smaller));
}

fn add_two(a: i32) -> i32 {
    a + 2
}

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}

fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    String::from("hello")
}

#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value < 200 {
            panic!("guess value between 1 and 100, got {}", value);
        }

        Guess { value }
    }
}

#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}

#[test]
#[ignore]
fn massive_compute_task() {
    for i in 0..10000 {
        println!("xxx")
    }
}

/// 测试模块 和 `#[cfg(test)]` 告诉编译器只有在运行 cargo test 时才编译和运行测试代码。
///
/// 集成测试对于需要测试的库来说，是完全外部的，同其他使用库一样来使用被测的库代码
///
///

#[cfg(test)]
mod test {
    #[test]
    fn it_will_work() {
        assert_eq!(2 + 2, 4);
    }
}
