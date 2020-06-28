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
    /// 闭包定义中省略了类型注解（现在要编译器需要返回类型了，目前编译不通过）
    // let add_one_v3 = |x| { x + 1 };
    /// 去掉了可选的大括号（现在要编译器需要返回类型了，目前编译不通过）
    // let add_one_v4 = |x| x + 1  ;

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // 推断出类型后就不能更改类型 let n = example_closure(5);


// 使用带有泛型和 Fn trait 的闭包
    struct Cacher<T>
        where T: Fn(u32) -> u32
    {
        calculation: T, // 这是一个泛型
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
                    let v = (self.calculation)(arg); // 对第一次传入的值传入到闭包(self.calculation)并计算
                    self.value = Some(v); // value 为 Option枚举类型
                    v
                }
            }
        }
    }
// 闭包会捕获其环境
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
    // FnMut 获取可变的借用值所以可以改变其环境
    // Fn 从其环境获取不可变的借用值

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
/// 迭代器
pub fn Iterator() {
    // v1_iter 需要是可变的：在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。
    let v1 = vec![1, 2, 3];
    // 调用 iter 方法生成一个不可变引用的迭代器
    let mut v1_iter = v1.iter();
    // 调用 into_iter
    // 调用 iter_mut

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

// 消费迭代器的方法
    // 这些调用 next 方法的方法被称为 消费适配器（consuming adaptors），因为调用他们会消耗迭代器。一个消费适配器的例子是 sum 方法。
    // 这个方法获取迭代器的所有权并反复调用 next 来遍历迭代器，因而会消费迭代器。当其遍历每一个项时，它将每一个项加总到一个总和并在迭代完成时返回总和。
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

// 产生其他迭代器的方法：迭代器适配器
    // 他们允许我们将当前迭代器变为不同类型的迭代器。可以链式调用多个迭代器适配器。
    // 不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。

    // map 方法使用闭包来调用每个元素以生成新的迭代器。
    let v1: Vec<i32> = vec![1,2,3];

    // 会有警告 迭代器适配器是惰性的，而这里我们需要消费迭代器
    // v1.iter().map(|x| x + 1);

    // 调用 map 方法创建一个新迭代器，接着调用 collect 方法消费新迭代器并创建一个 vector
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

// 使用闭包获取环境
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }
    /// 迭代器的 filter 方法获取一个使用迭代器的每一个项并返回布尔值的闭包。
    fn shose_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    let in_my_size = shose_in_my_size(shoes, 10);
    assert_eq!(in_my_size, vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 10, style: String::from("boot") },
    ])

}

