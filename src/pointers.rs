use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;


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

// 解引用强制多态：是 Rust 在函数或方法传参上的一种便利。
// 【其将实现了 Deref 的类型的引用转换为原始类型通过 Deref 所能够转换的类型的引用。】
// 【当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时，解引用强制多态将自动发生。】
// 上面的 &m 为 &MyBox<String>， Rust 可以通过 deref 转换为 &String，就是 实现了Deref 的类型的引用 转换为 原始类型的引用（&String）


// 点进 Deref   https://doc.rust-lang.org/std/ops/trait.Deref.html#associated-types
    // #[lang = "deref"]
    // #[doc(alias = "*")]
    // #[doc(alias = "&*")]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub trait Deref {
    //     /// The resulting type after dereferencing.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Target: ?Sized;
    //
    //     /// Dereferences the value.
    //     #[must_use]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn deref(&self) -> &Self::Target;
    // }

// 对 fn deref(&self) 右键 -> 转到 -> definition(s)
// 标准库中提供了 String 上的 Deref 实现，其会返回字符串 slice
// Rust 再次调用 deref 将 &String 变为 &str
    // #[stable(feature = "rust1", since = "1.0.0")]
    // impl ops::Deref for String {
    //     type Target = str;
    //
    //     #[inline]
    //     fn deref(&self) -> &str {
    //         unsafe { str::from_utf8_unchecked(&self.vec) }
    //     }
    // }


    hello(&m);
    // 如果 Rust 没有实现解引用强制多态
    hello(&(*m)[..]);

    // Rust 在发现类型和 trait 实现满足三种情况时会进行解引用强制多态：

    // 当 T: Deref<Target=U> 时从 &T 到 &U。
    // 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
    // 当 T: Deref<Target=U> 时从 &mut T 到 &U。

    // 第三个情况有些微妙：Rust 也会将可变引用强转为不可变引用。
    // 但是反之是 不可能 的：不可变引用永远也不能强转为可变引用。
    // 因为根据借用规则，如果有一个可变引用，其必须是这些数据的唯一引用（否则程序将无法编译）。
    // 将一个可变引用转换为不可变引用永远也不会打破借用规则。
    // 将不可变引用转换为可变引用则需要数据只能有一个不可变引用，而借用规则无法保证这一点。
    // 因此，Rust 无法假设将不可变引用转换为可变引用是可能的。
}
/// 使用 Drop Trait 运行清理代码：其允许我们在值要离开作用域时执行一些代码
pub fn drop_trait() {

    struct CustomSmartPointer {
        data: String,
    }
    impl  Drop for CustomSmartPointer{
        fn drop(&mut self) {
            println!("正在进行离开作用域时的清除方法，数据为：{}", self.data);
        }
    }
    let c = CustomSmartPointer { data: String::from("my stuff") };
    println!("CustomSmartPointers created.");
    // 通过 std::mem::drop 提早丢弃值
    drop(c); // 丢弃在结束之前
    println!("CustomSmartPointer dropped before the end of main.");

    // CustomSmartPointers created.
    // 正在进行离开作用域时的清除方法，数据为：my stuff
    // CustomSmartPointer dropped before the end of main.
}
/// Rc<T> 引用计数智能指针. 注意 Rc<T> 只能用于单线程场景
pub fn Rc () {
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // 或 a.clone();
    let c = Cons(4, Rc::clone(&a));
    // Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。
    // Rc::clone 只会增加引用计数
    println!("{:?}", b);

    // 每个引用计数变化的点，会打印出引用计数，其值可以通过调用 Rc::strong_count 函数获得
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

// 测试最初的为可变引用，其他变量clone引用之后改变之前的变量值，检查引用的变量 值是否改变

    // 有别的引用之后，就不要改值了（待检查）
    struct AA { aa: Rc<i32>, bb: i32 }
    let mut x = Rc::new(3);
    let a = AA{ aa: Rc::clone(&x), bb: 0 };
    let b = AA{ aa: Rc::clone(&x), bb: 1 };
    println!("x引用次数：{}", Rc::strong_count(&x)); // x引用次数：3
    // *Rc::get_mut(&mut x).unwrap() = 4; // thread 'main' panicked at 'called `Option::unwrap()` on a `None` value',
    println!("x: {}", x);

}
pub fn refcell(){
    pub trait Messenger {
        fn send (&self, msg: &str);
    }
    pub struct LimitTracker<'a, T: Messenger> { // trait作为参数，同时使用生命周期参数
        messenger: &'a T,
        value: usize,
        max: usize,
    }
    impl<'a, T> LimitTracker<'a, T>
        where T: Messenger {
        pub fn new (messenger: &T, max: usize) -> LimitTracker<T>{
            LimitTracker{
                messenger,
                value: 0,
                max
            }
        }
        // 这里是设置值，所以用可变
        pub fn set_value(&mut self, value: usize){
            self.value = value;
            let percentage_of_max = self.value as f64 / self.max as f64;
            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger.send("Warning: You've used up over 75% of your quota!");
            }
        }
    }
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }
    impl MockMessenger{
        fn new() -> MockMessenger{
            MockMessenger{ sent_messages: RefCell::new(vec![]) }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // 如果参数不为可变那就需要 sent_messages: RefCell<Vec<String>>,
            // self.sent_messages.push(String::from(msg));

            // borrow_mut 以获取 vector 的可变引用
            self.sent_messages.borrow_mut().push(String::from(msg))
        }
    }

    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger,100);
    limit_tracker.set_value(80);
    // borrow() 以获取 vector 的不可变引用
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);

    // borrow 方法返回 Ref 类型的智能指针，
    // borrow_mut 方法返回 RefMut 类型的智能指针。
    // RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用。

    let a = "TT";
    let one_refcell = RefCell::new(String::from(a));
    let mut ref1 = one_refcell.borrow_mut();
    // let ref2 = one_refcell.borrow();
    ref1.push_str("2222");
    // one_refcell.borrow() 为不可变引用，*one_refcell.borrow() 为String类型
    println!("{:?}", one_refcell);


    // 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
    #[derive(Debug)]
    enum List{
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{*};
    // use crate::pointers::refcell::list;
    let value =  Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value),Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)),Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
pub fn reference_cycles(){
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }
    use List::{*};
    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None
            }
        }
    }

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
    }
    let leaf = Rc::new(
        Node{ value: 3, children: RefCell::new(vec![]) }
    );
    let branch = Rc::new(
        Node{ value: 5, children: RefCell::new(vec![Rc::clone(&leaf)]) }
    );


}