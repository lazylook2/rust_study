use std::thread;
use std::time::Duration;

/// 闭包
pub fn Closures() {
    fn calculate(a: u32) -> u32 {
        println!("计算太慢");
        thread::sleep(Duration::from_secs(2));
        a
    }
    let user_value = 10;
    let random_number = 7;

    generate_workout(
        user_value,
        random_number,
    );

    //--------------- 有效的定义 ---------------------
    /// 函数定义
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    /// 完整标注的闭包定义
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    /// 闭包定义中省略了类型注解
    let add_one_v3 = |x|             { x + 1 };
    /// 去掉了可选的大括号
    let add_one_v4 = |x|               x + 1  ;

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // 推断出类型后就不能更改类型 let n = example_closure(5);


// 使用带有泛型和 Fn trait 的闭包
    struct Cacher<T>
        where T: Fn(u32) -> u32
    {
        calculation: T,
        value: Option<u32>,
    }
    impl<T> Cacher<T> where T: Fn(u32) -> u32
    {
        fn new (calculation: T) -> Cacher<T>{
            Cacher { calculation, value: None }
        }
        fn value (&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    fn generate_workout(intensity: u32, random_number: u32) {
        // 解决了第一个 if 块中不必要的两次调用函数的问题。
        // 不幸的是，现在所有的情况下都需要调用函数并等待结果，包括那个完全不需要这一结果的内部 if 块。
        // let result = calculate(intensity);

        // 现在耗时的计算只在一个地方被调用，并只会在需要结果的时候执行改代码。
        // 但是这样依然会调用两次
        let result = |num| {
            println!("计算太慢");
            thread::sleep(Duration::from_secs(2));
            num
        };
        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                result(intensity)
            );
            println!(
                "Next, do {} situps!",
                result(intensity)
            );
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    result(intensity)
                );
            }
        }
    }
}

