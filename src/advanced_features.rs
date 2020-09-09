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
use std::ops::Add;

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
/// 默认泛型类型参数和运算符重载
/// 默认泛型类型参数：当使用泛型类型参数时，可以为泛型指定一个默认的具体类型。
/// 例子：运算符重载。std::ops 中所列出的运算符和相应的 trait 可以通过实现运算符相关 trait 来重载。
///     RHS=Self：这个语法叫做 默认类型参数
///     Self表示调用者的类型
///     trait Add<RHS=Self> {
///         type Output;
///         fn add(self, rhs: RHS) -> Self::Output;
///     }
///
/// 为了使 Millimeters 和 Meters 能够相加，
/// 我们指定 impl Add<Meters> 来设定 RHS 类型参数的值而不是使用默认的 Self。
/// 默认参数类型主要用于如下两个方面：
///     扩展类型而不破坏现有代码。
///     在大部分用户都不需要的特定情况进行自定义。
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

    // PartialEq ???????????????????????????????????
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point{ x: self.x + other.x, y: self.y + other.y }
        }
    }
    assert_eq!(Point{ x: 1, y: 0 } + Point{ x: 2, y: 3 }, Point{ x: 3, y: 3 });
    #[derive(Debug)]
    struct Millimeters(u32);
    #[derive(Debug)]
    struct Meters(u32);
    // Add<Meters> 代表要加的类型为Meters（就是 + 后面的）
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + other.0 * 1000)
        }
    }
    // let mi = Millimeters(1);
    // let m = Meters(1);
    // let r = mi + m;
    println!("运算符重载：{:?} + {:?} = {:?}", Millimeters(3000), Meters(3), Millimeters(3000) + Meters(3))
}
/// 完全限定语法与消歧义：调用相同名称的方法
pub fn advanced_traits3(){
    /// 飞行员 接口
    trait Pilot {
        fn fly(&self);
    }
    /// 巫师 接口
    trait Wizard {
        fn fly(&self);
    }
    /// 实体
    struct Human {
        name: String
    }

    impl Pilot for Human {
        fn fly(&self) {
            println!("{}\t都让开，我要上天了！", self.name)
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("汝等{}\t退下，吾要作法了！", self.name)
        }
    }
    impl Human{
        fn fly(&self) {
            println!("我{} 就静静的看着你们装逼！", self.name)
        }
    }

    let person = Human{ name: String::from("智障") }; // 啥意思？？？？？？？？？？？
    person.fly(); // 默认调用直接是现在类型上的方法
    Pilot::fly(&person); //
    Pilot::fly(&Human{ name: String::from("智障") }); // 同上

///////// 关联函数是 trait 的一部分，但没有 self 参数。 ///////
    trait Animal {
        fn baby_name(name: &str) -> String;
    }
    struct Dog {
        name: String
    };
    impl Dog {
        fn baby_name(name: &str) -> String {
            format!("Spot  {}", name)
        }
    }
    impl Animal for Dog {
        fn baby_name(name: &str) -> String {
            // String::from("puppy")
            format!("属于动物的狗的名字：{}", name)
        }
    }
    println!("{}", Dog::baby_name("小黑"));
    // 方法无self参数
    println!("{}", <Dog as Animal>::baby_name("小狗"));
}
/// 宏 <br>
/// 使用 macro_rules! 的 声明宏 用于通用元编程<br>
/// 声明宏允许我们编写一些类似 Rust match 表达式的代码。
pub fn macros1() {
    

}
