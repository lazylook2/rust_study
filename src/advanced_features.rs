use std::slice;

/// 解引用裸指针<br>
/// 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针<br>
/// 不保证指向有效的内存<br>
/// 允许为空<br>
/// 不能实现任何自动清理功能<br>
pub fn unsafe_rust1() {
    let mut num = 5;
    let r1 = &num as *const i32;    // 不可变裸指针
    let r2 = &mut num as *mut i32;      // 可变裸指针

    // 创建了同时指向相同内存位置 num 的裸指针 *const i32 和 *mut i32。
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2)
    }
}
/// 调用不安全函数或方法<br>
/// 开头有一个额外的 unsafe<br>
/// 必须在一个单独的 unsafe 块中调用 dangerous 函数。
pub fn unsafe_rust2() {
    unsafe fn dangerous() { println!("不安全的方法") }
    unsafe {
        dangerous();
    }
}
/// 创建不安全代码的安全抽象
pub fn unsafe_rust3() {
    pub fn split_at_mut1(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        /*let len = slice.len();
        assert!(mid <= len);
        (&mut slice[..mid], &mut slice[mid..])*/
        let len = slice.len();
        let ptr = slice.as_mut_ptr(); // 使用 as_mut_ptr 方法访问 slice 的裸指针
        assert!(mid <= len);
        unsafe {
            (
                // slice::from_raw_parts_mut 函数获取一个裸指针和一个长度来创建一个 slice。
                slice::from_raw_parts_mut(ptr, mid),
                // 之后在 ptr 上调用 offset 方法并使用 mid 作为参数来获取一个从 mid 开始的裸指针，
                // 使用这个裸指针并以 mid 之后项的数量为长度创建一个 slice。
                slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
            )
        }


    }
    let address = 0x01234usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };
}
/// 使用 extern 函数调用外部代码<br>
/// extern，有助于创建和使用 外部函数接口（Foreign Function Interface， FFI）
pub fn unsafe_rust4() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("使用C语言的函数计算绝对值：{}", abs(-3))
    }
}
/// 访问或修改可变静态变量
/// 通常静态变量的名称采用 SCREAMING_SNAKE_CASE 写法，并 必须 标注变量的类型
pub unsafe fn unsafe_rust5() {
    static HELLO_WORLD: &str = "Hello, world!";
    println!("name is: {}", HELLO_WORLD);

    static mut COUNTER: u32 = 0;
    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc
        }
    }
    add_to_count(3);
    unsafe {
        // 常量与静态变量的另一个区别在于静态变量可以是可变的。访问和修改可变静态变量都是 不安全 的。
        // 此处如果不包unsafe就得  pub unsafe fn unsafe_rust5()
        println!("COUNTER: {}", COUNTER);
    }
}
/// 实现不安全 trait
fn unsafe_rust6() {
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}
/// 高级 trait
/// 关联类型在 trait 定义中指定占位符类型
/// 关联类型（associated types）是一个将类型占位符与 trait 相关联的方式，这样 trait 的方法签名中就可以使用这些占位符类型。
use std::iter::Iterator;
pub fn advanced_traits1() {
    // use std::iter::Iterator;
    pub trait Iterator {
        type Item; // 返回数据类型
        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter { num: String }
    impl Iterator for Counter {
        type Item = u32; // 必须得是 Item

        fn next(&mut self) -> Option<Self::Item> {
            Some(self.num.len() as u32)
        }
    }
    let mut cs = Counter{ num: "哈哈".to_string() };
    println!("{}", cs.next().unwrap());
    println!("{}", "dd".to_string().len());
    println!("哈哈长度: {}（中文每个字符占3个字节）\t 正确使用：{}", String::from("哈哈").len(), String::from("哈哈").chars().count() as u32);

}
pub fn advanced_traits2 () {
    /*struct Counter {
        count: u32,
    }
    impl Counter{
        fn new () -> Counter{
            Counter{ count: 0 }
        }
    }
    impl Iterator for Counter{
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // 这里调用into_iter 返回的还是Counter类型
    let couter_iter = counter.into_iter().count;
    println!("counter.into_iter().count: {}", couter_iter);*/


}
/// 宏 <br>
/// 使用 macro_rules! 的 声明宏 用于通用元编程<br>
/// 声明宏允许我们编写一些类似 Rust match 表达式的代码。
pub fn macros1() {
    

}
