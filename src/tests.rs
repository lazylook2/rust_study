// #[cfg(test)] 告诉 Rust 只在执行 cargo test 时才编译和运行测试代码，
// 而在运行 cargo build 时不这么做。
#[cfg(test)] // 标注为测试模块
mod tests {
    #[test]
    /// cargo test 命令执行
    fn it_works() {
// 判断相等
        assert_eq!(2 + 2, 4);
// 判断不等
        assert_ne!(2, 4)
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
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    fn greeting(name: &str) -> String {
        String::from("Hello!")
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
// 自定义失败信息
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess {
                value
            }
        }
    }

    #[test]
    #[should_panic]
    /// 通过对函数增加另一个属性 should_panic 来实现这些。
    /// 这个属性在函数中的代码 panic 时会通过，而在其中的代码没有 panic 时失败。
    /// 程序正常显示报错的信息，只是不显示红色的报错状态
    fn greater_than_100() {
        Guess::new(200);
    }
    /// 将 Result<T, E> 用于测试
    fn it_works1() -> Result<bool, String> {
        if 2 + 2 == 4 {
            Ok(true)
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    #[test]
    fn test_return(){
        let a = it_works1();
        println!("{}", a.unwrap())
    }
}
pub fn test2() {
//    并行或连续的运行测试
//     $ cargo test -- --test-threads=1 // 1为线程数量

//    显示函数输出（如果你希望也能看到通过的测试中打印的值）
//    assert_eq!(10, value);  // I got the value 8
//     $ cargo test -- --nocapture

    // 通过指定名字来运行部分测试
    // $ cargo test one_hundred
    // 过滤运行多个测试
    // $ cargo test add // 测试的名称包含add的

    // 忽略某些测试
    #[test]
    #[ignore] // cargo test不会运行这个，cargo test -- --ignored 会
    fn expensive_test() {
        // 需要运行一个小时的代码
    }
}
pub fn add_two (a: i32) -> i32{
    a+2
}