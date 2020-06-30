use std::borrow::Borrow;
use std::ops::Deref;

/// 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候 （方法里面有）
/// 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候 （需要例子）
/// 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候 （？？？？？？）
pub fn box1() {
    let b = Box::new(5);
    println!("b = {}", b);

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::Cons;
    use List::Nil;
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let i = 2;
    match list  {
        List::Cons(i, j) => {
            println!("第一层：{}", i);
            match *j { // 注意：这里需要*j，j是指针
                List::Cons(i, j) => {println!("第二层：{}", i);},
                _ => {println!("第二层没有");}
            }
        },
        List::Nil => {println!("第二层没有");}
    }
}
/// 通过 Deref trait 将智能指针当作常规引用处理
pub fn deref_trait () {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(5);
    println!("{}", y); // 5
    assert_eq!(5, x);
    assert_eq!(5, *y);

    struct MyBox<T>(T); // 元组结构体
    impl<T> MyBox<T> {
        fn new (x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }
    let y = MyBox::new(5);
    assert_eq!(5, *y);

    fn hello (name: &str) {
        println!("hello, {}!", name);
    }
    let m = MyBox::new(String::from("hehe"));
    println!("*m: {}", *m);
    hello(&m);
}