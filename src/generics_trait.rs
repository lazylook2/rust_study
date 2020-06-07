use std::fmt::{Display, Debug};

pub fn generics(){
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
    let integer = Point{ x: 5, y: 10 };
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
    let p = Point{ x: 5, y: 10 };
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
    impl School for Student { // impl 接口 for 类名
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
    let xiaoming = Student{ name: String::from("小明"), age: 12 };
    println!("{} 学习的时候想要 {}", &xiaoming.study(), &xiaoming.eat_shit());
    println!("{}", &xiaoming.idiot_bitch()); // 如果重写默认实现，会调用重写的方法

    pub fn notify(item: impl School){
        println!("端午节放假！ {}", item.eat_shit())
    }
    // notify(xiaoming); // 能引用吗？？？？？
    pub fn notify1<T: School> (item: &T) {
        println!("端午节放假！ {}", item.eat_shit())
    }
    pub fn notify2<T: School> (item1: &T, item2: &T) {
        println!("端午节放假！ {}", item1.eat_shit())
    }
    // 参数：实现了School 和 teacher接口的实例
    pub fn notify3<T: School + teacher>(item: T) -> String{
        format!("{}{}", item.speak(), item.idiot_bitch())
    }
    // println!("{}", notify3(xiaoming));
    fn some_function<T, U>(t: T, u: U) -> i32
        where T: Display + Clone,
              U: Clone + Debug
    {2}
// 返回实现了 trait 的类型
    /// 调用方不知道返回的是 具体哪个实现了这个接口的类，就是具体哪个trait
    /// 第十三章会遇到
    fn return_school() -> impl School {
        Student{ name: String::from("小强"), age: 11 }
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