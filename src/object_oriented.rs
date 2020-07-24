use std::str::from_boxed_utf8_unchecked;

/// 封装
pub fn oo1() {
    pub struct AveragedCollection{
        list: Vec<i32>,
        average: f64
    }
    impl AveragedCollection{
        pub fn add(&mut self, value: i32){
            self.list.push(value);
            self.update_average()
        }
        pub fn update_average(&mut self){
            let sum: i32 = self.list.iter().sum(); // 这里需要明确变量类型
            self.average = sum as f64 / self.list.len() as f64
        }
        pub fn remove(&mut self) -> Option<i32>{
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                },
                None => None
            }
        }
        pub fn average(&self) -> f64 {
            self.average
        }
    }
    let mut c = AveragedCollection{ list: vec![], average: 0.0 };
    c.add(1);
    c.add(2);
    c.add(3);
    println!("平均值：{}", c.average());
    c.remove().unwrap();
    println!("平均值：{}", c.average());
}
/// 多态 <br>
/// 为使用不同类型的值而设计的 trait 对象
pub fn oo2 () {
    /// 绘制 - 接口
    pub trait Draw {
        fn draw(&self);
    }
    /// 屏幕 - 实体
    pub struct Screen {
        /// 组件（实现了Draw接口的实体集合） <br>
        /// dyn Trait与通用参数或不同impl Trait，编译器不知道要传递的具体类型。也就是说，该类型已被擦除。 <br>
        /// 这样，一个dyn Trait引用包含两个指针。一个指针指向该数据（例如，结构的实例）。 <br>
        /// 另一个指针指向方法调用名称到函数指针的映射（称为虚拟方法表或vtable）。
        pub components: Vec<Box<dyn Draw>>,
    }
    impl Screen {
        /// 循环屏幕上的组件，并调用接口实现类的draw方法
        fn run (&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    // 泛型类型参数一次只能替代一个具体类型，而 trait 对象则允许在运行时替代多种具体类型。
    /*pub struct Screen<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> Screen<T>
        where T: Draw {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }*/

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String
    }
    impl Draw for Button{
        fn draw(&self) {
            println!("开始绘制按钮")
        }
    }
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("开始绘制选择框")
        }
    }

    let screen  = Screen{ components: vec![
        Box::new(SelectBox{
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No")
            ]
        }),
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }),
        // 如果值没有实现 trait 对象所需的 trait 则 Rust 不会编译这些代码。
        // Box::new(String::from("Hi")),
    ] };
    screen.run();

    // Trait 对象要求对象安全
    // 只有 对象安全（object safe）的 trait 才可以组成 trait 对象。

    // 1、返回值类型不为 Self
    // 2、方法没有任何泛型类型参数

    /*pub trait Clone {
        fn clone(&self) -> Self;
    }
    pub struct Screen {
        pub components: Vec<Box<dyn Clone>>,
    }*/
}
/// 面向对象设计模式的实现
pub fn oo3(){

    /// 博文
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String
    }
    impl Post {
        pub fn new () -> Post{
            Post { state: Some(Box::new(Draft)), content: String::new() }
        }
        /// 插入博文内容
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text)
        }
        /// 如果是草案状态就返回空
        pub fn content(&self) -> &str{
            ""
        }
        pub fn request_review(&mut self){
            // 调用 take 方法将 state 字段中的 Some 值取出并留下一个 None
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

    }
    /// 状态
    trait State {
        fn request_review(self: Box<self>) -> Box<dyn State>;
    }
    // 草案
    struct  Draft {}
    impl State for Draft {
        fn request_review(self: Box<_>) -> Box<dyn State> {
            Box::new(PendingReview{})
        }
    }
    struct PendingReview {}
    impl State for PendingReview {
        fn request_review(self: Box<_>) -> Box<dyn State> {
            self
        }
    }
}