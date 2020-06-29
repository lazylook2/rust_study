use std::borrow::Borrow;

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
    match list {
        Cons(i, b) => {
            println!("i: {}", i);
            match b.as_ref() { // ??????????? 这里需要有方法（从Box<List> 到 List）
                Cons(i, b) => {
                    println!("i: {}", i);
                },
                _ => (),
            }
        }
        _ => (),
    }
}
pub fn deref_trait () {}