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
use std::fmt;
use std::fmt::Formatter;

/// 父 trait 用于在另一个 trait 中使用某 trait 的功能
/// ???????????????????????????????????????
pub fn advanced_traits4 (){
    /// 因为指定了 OutlinePrint 需要 Display trait，则可以在 outline_print 中使用 to_string，其会为任何实现 Display 的类型自动实现。
    /// 如果不在 trait 名后增加 : Display 并尝试在 outline_print 中使用 to_string，
    /// 则会得到一个错误说在当前作用域中没有找到用于 &Self 类型的方法 to_string。
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point {}
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    println!("{}", Point{ x: 3, y: 4 })
}
/// newtype 模式用以在外部类型上实现外部 trait
/// 如果想要在 Vec<T> 上实现 Display，而孤儿规则阻止我们直接这么做，因为 Display trait 和 Vec<T> 都定义于我们的 crate 之外。
/// 可以创建一个包含 Vec<T> 实例的 Wrapper 结构体
pub fn advanced_traits5 () {
    struct Wapper (Vec<String>);
    impl fmt::Display for Wapper {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    // Display 的实现使用 self.0 来访问其内部的 Vec<T>，因为 Wrapper 是元组结构体而 Vec<T> 是结构体总位于索引 0 的项。
    // 接着就可以使用 Wrapper 中 Display 的功能了。

    let w = Wapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w)
}
use std::error::Error;
/// 类型别名用来创建类型同义词
pub fn advanced_types1 () {
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

// 用于代替很长的类型如：Box<dyn Fn() + Send + 'static>
    /*type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
    }*/

// 类型别名也经常与 Result<T, E> 结合使用来减少重复。

    /*pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        fn flush(&mut self) -> Result<(), Error>;

        fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    }*/
    type Result<T> = std::result::Result<T, std::io::Error>;
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }
}
/// 从不返回的 never type<br>
/// Rust 有一个叫做 ! 的特殊类型。在类型理论术语中，它被称为 empty type，因为它没有值。<br>
/// 我们更倾向于称之为 never type。<br>
/// 这个名字描述了它的作用：在函数从不返回的时候充当返回值。
pub fn advanced_types2() {
    // 这读 “函数 bar 从不返回”，而从不返回的函数被称为 发散函数
    // 不能创建 ! 类型的值，所以 bar 也不可能返回值。
    //   fn bar() -> ! {}

    let guess = "3";
    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    };

    /*let guess = match guess.trim().parse() {
        Ok(_) => 5,
        Err(_) => "hello",
    };*/
    // 上面的guess必须是整形也是字符串而rust要求guess只能是一个类型
    // 那么continue返回了u32另一个分支却以continue结束呢
    // continue的值是 ! 。也就是说rust计算guess的类型时，它查看这两个分支。
    // 前者是 u32 值，而后者是 ! 值。因为 ! 并没有一个值，Rust 决定 guess 的类型是 u32。
    // 描述 ! 的行为的正式方式是 never type 可以强转为任何其他类型。
    //      允许 match 的分支以 continue 结束是因为 continue 并不真正返回一个值；
    //      相反它把控制权交回上层循环，所以在 Err 的情况，事实上并未对 guess 赋值。

    // never type 的另一个用途是 panic!。还记得 Option<T> 上的 unwrap 函数吗？它产生一个值或 panic。


    /*impl<T> Option<T> {
        pub fn unwrap(self) -> T {
            match self {
                Some(val) => val,
                None => panic!("called `Option::unwrap()` on a `None` value"),
            }
        }
    }*/
    // 这里与示例 19-34 中的 match 发生了相同的情况：
    //     Rust 知道 val 是 T 类型，panic! 是 ! 类型，所以整个 match 表达式的结果是 T 类型。
    // 这能工作是因为 panic! 并不产生一个值；它会终止程序。
    // 对于 None 的情况，unwrap 并不返回一个值，所以这些代码是有效。

    // 最后一个有着 ! 类型的表达式是 loop：
    print!("forever ");

    // 这里，循环永远也不结束，所以此表达式的值是 !。但是如果引入 break 这就不为真了，因为循环在执行到 break 后就会终止。
    loop {
        print!("and ever ");
    }
}
/// 高级函数与闭包
/// 函数指针
pub fn advanced_functions1() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    let answer = do_twice(add_one, 5);
    println!("答案是： {}", answer);
    // 这会打印出 The answer is: 12。do_twice 中的 f 被指定为一个接受一个 i32 参数并返回 i32 的 fn。
    // 接着就可以在 do_twice 函数体中调用 f。
    // 在 main 中，可以将函数名 add_one 作为第一个参数传递给 do_twice。

    // 不同于闭包，fn 是一个类型而不是一个 trait，所以直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数。
    // 函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce），所以总是可以在调用期望闭包的函数时传递函数指针作为参数。
    // 倾向于编写使用泛型和闭包 trait 的函数，这样它就能接受函数或闭包作为参数。

    fn wapper_func<T>(t: T, v: i32) -> i32
        where T: Fn(i32) -> i32 {
        t(v)
    }

    fn func(v: i32) -> i32 {
        v + 1
    }
    //+++++++++++++++++
    let a = wapper_func(|x| x+1, 1);
    println!("a = {}", a);

    let b = wapper_func(func, 1);
    println!("b = {}", b);

    // 使用 map 函数将一个数字 vector 转换为一个字符串 vector，就可以使用闭包，比如这样：
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter().map(|i| i.to_string()).collect();
    for s in list_of_strings.iter() { println!("{}\t", s); }

    // 或者可以将函数作为 map 的参数来代替闭包，像是这样：
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)// ?????????????????
        .collect();
    for s in list_of_strings.iter() { println!("{}\t", s); }
    // 注意这里必须使用 “高级 trait” 部分讲到的完全限定语法，因为存在多个叫做 to_string 的函数；
    //     这里使用了定义于 ToString trait 的 to_string 函数，标准库为所有实现了 Display 的类型实现了这个 trait。

    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> =
        (0u32..20)
            .map(Status::Value)
            .collect();
    // 这里创建了 Status::Value 实例，它通过 map 用范围的每一个 u32 值调用 Status::Value 的初始化函数。
    // 一些人倾向于函数风格，一些人喜欢闭包。这两种形式最终都会产生同样的代码，所以请使用对你来说更明白的形式吧。

    for s in list_of_statuses.iter() {
        match s {
            Status::Value(num) => println!("{}", num),
            _ => {}
        }
    }



}
/// 返回闭包
pub fn advanced_functions2() {
    /*这段代码尝试直接返回闭包，它并不能编译：
    fn returns_closure() -> Fn(i32) -> i32 {
        |x| x + 1
    }*/

    // Rust 并不知道需要多少空间来储存闭包。不过我们在上一部分见过这种情况的解决办法：可以使用 trait 对象：
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    // 这段代码正好可以编译。
}
/// 宏 <br>
/// 使用 macro_rules! 的 声明宏 用于通用元编程<br>
/// 声明宏允许我们编写一些类似 Rust match 表达式的代码。
pub fn macros1() {
    #[macro_export]
    macro_rules! my_vec {
        ( $( $x: expr ), * ) => {
            {
                let mut temp_vec = Vec::new();
                $ (
                    temp_vec.push($x);
                )*
                temp_vec
            }
        }
    }
    let v = my_vec![1, 2, 3];
    println!("v = {:?}", v);

}
