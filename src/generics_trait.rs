use std::fmt::{Display, Debug};

pub fn generics() {
    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest_char(&char_list));

    /*fn largest<T>(list: &[T]) -> T{
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    println!("The largest char is {}", largest(&char_list));*/
    // 结构体定义中的泛型
    struct Point<T> {
        x: T,
        y: T,
    }
    let integer = Point { x: 5, y: 10 };
    // 枚举定义中的泛型
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        OK(T),
        Err(E),
    }
    // 方法定义中的泛型
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    struct Point1<T, U> {
        x: T,
        y: U,
    }
    impl<T, U> Point1<T, U> {
        fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
            Point1 {
                x: self.x,
                y: other.y,
            }
        }
    }
}

pub fn traits() {
    pub trait School {
        fn study(&self) -> &String;
        fn eat_shit(&self) -> String;
        fn idiot_bitch(&self) -> String { // 默认实现
            String::from("存在臭傻逼！")
            // format!("臭傻逼！{}", self.eat_shit())
        }
    }
    pub trait teacher {
        fn speak(&self) -> String {
            String::from("滚，出去站着！")
        }
    }
    pub struct Student {
        pub name: String,
        pub age: i32,
    }
    impl School for Student {
        // impl 接口 for 类名
        fn study(&self) -> &String {
            &self.name
        }

        fn eat_shit(&self) -> String {
            format!("吃屎 \t{}: {}", self.name, self.age)
        }
        fn idiot_bitch(&self) -> String { // 重写默认实现
            String::from("臭傻逼1111111111111！")
            // format!("臭傻逼！{}", self.eat_shit())
        }
    }
    impl teacher for Student {}
    let xiaoming = Student { name: String::from("小明"), age: 12 };
    println!("{} 学习的时候想要 {}", &xiaoming.study(), &xiaoming.eat_shit());
    println!("{}", &xiaoming.idiot_bitch()); // 如果重写默认实现，会调用重写的方法

    pub fn notify(item: impl School) {
        println!("端午节放假！ {}", item.eat_shit())
    }
    // notify(xiaoming); // 能引用吗？？？？？
    pub fn notify1<T: School>(item: &T) {
        println!("端午节放假！ {}", item.eat_shit())
    }
    pub fn notify2<T: School>(item1: &T, item2: &T) {
        println!("端午节放假！ {}", item1.eat_shit())
    }
    // 参数：实现了School 和 teacher接口的实例
    pub fn notify3<T: School + teacher>(item: T) -> String {
        format!("{}{}", item.speak(), item.idiot_bitch())
    }
    // println!("{}", notify3(xiaoming));
    fn some_function<T, U>(t: T, u: U) -> i32
        where T: Display + Clone,
              U: Clone + Debug
    { 2 }
    // 返回实现了 trait 的类型
    /// 调用方不知道返回的是 具体哪个实现了这个接口的类，就是具体哪个trait
    /// 第十三章会遇到
    fn return_school() -> impl School {
        Student { name: String::from("小强"), age: 11 }
    }

    /*
    /// 这里尝试返回 NewsArticle 或 Tweet。这不能编译，因为 impl Trait 工作方式的限制。第十七章的 “为使用不同类型的值而设计的 trait 对象” 部分会介绍如何编写这样一个函数。
    fn returns_summarizable(switch: bool) -> impl Summary {
        if switch {
            NewsArticle {
                headline: String::from("Penguins win the Stanley Cup Championship!"),
                location: String::from("Pittsburgh, PA, USA"),
                author: String::from("Iceburgh"),
                content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
            }
        } else {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }
    }*/
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
    let pair = Pair::new(String::from("好X"), String::from("有Y"));
    // let pair = Pair::new(2, 3);
    pair.cmp_display();
}

pub fn lifetime() {
    /*fn longest(x: &str, y: &str) -> &str{
        if x.len() > y.len() {
            x
        } else {

        }
    }
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // 当我们定义这个函数的时候，并不知道传递给函数的具体值，所以也不知道到底是 if 还是 else 会被执行。
    // 我们也不知道传入的引用的具体生命周期，那样通过观察作用域来确定返回的引用是否总是有效。
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);*/

    // 参数类型
    // &i32    // 引用
    // &'a i32 // 带有显示生命周期的引用
    // &'a mut i32 // 带有显示声明周期的可变引用

    // 这里我们想要告诉 Rust 关于参数中的引用和返回值之间的限制是他们都必须拥有相同的生命周期

    // 当具体的引用被传递给 longest 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分。
    // 换一种说法就是泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个。
    // 因为我们用相同的生命周期参数 'a 标注了返回的引用值，所以返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效。
    fn longest1<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // string1 直到外部作用域结束都是有效的，string2 则在内部作用域中是有效的，而 result 则引用了一些直到内部作用域结束都是有效的值。
    // 借用检查器认可这些代码；它能够编译和运行，并打印出 The longest string is long string is long。
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = longest1(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }


    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        let string2 = "xyz"; // ????????????????????  为啥能运行呢
        result = longest1(string1.as_str(), string2);
    }
    println!("result: {}", result);

    /// 参数 y 指定，因为 y 的生命周期与参数 x 和返回值的生命周期没有任何关系。
    fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    /*/// 当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。
    /// 如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用
    /// 因为它将会在函数结束时离开作用域
    fn longest3<'a>(x: &str, y: &str) -> &'a str {
        let result = String::from("really long string");
        result.as_str()
    }*/

    // 结构体定义中的生命周期注解
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("ImportantExcerpt: {:#?}", i);
    println!("{}", i.part);

// 生命周期省略
//     第一条规则是每一个是引用的参数都有它自己的生命周期参数。
//     第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
//     第三条规则是如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为 &self 或 &mut self，那么 self 的生命周期被赋给所有输出生命周期参数。

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..];
            }
        }
        &s[..]
    }
    // 方法定义中的生命周期注解
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            self.part.len() as i32
        }
    }
    println!("{}", i.level());
    impl<'a> ImportantExcerpt<'a> {
        /// 这里有两个输入生命周期，所以 Rust 应用第一条生命周期省略规则并给予 &self 和 announcement 他们各自的生命周期。
        /// 接着，因为其中一个参数是 &self，返回值类型被赋予了 &self 的生命周期，这样所有的生命周期都被计算出来了。
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

// 静态生命周期
    let s: &'static str = "I have a static lifetime.";
    // 'static，其生命周期能够存活于整个程序期间。

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where T: Display
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

}