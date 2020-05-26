pub fn vector() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);

    let third = v[2];
    println!("The third element is {}", third);
    let third = &v[3];
    println!("The third element is {}", third);

    match v.get(2) { // get()方法有效确保数组越界问题
        Some(i) => println!("The third element is {}", i),
        None => println!("There is no third element."),
    }
    let first = &v[0];
    v.push(6);
    // println!("The first element is: {}", first); // 这一步报错了
    // 在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。
    // 这时，第一个元素的引用就指向了被释放的内存。

    for i in &v {
        println!("{}", i)
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // 使用枚举来储存多种类型可以通过面向对象的方式取代
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // 1、Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道储存每个元素到底需要多少内存
    // 2、可以准确的知道这个 vector 中允许什么类型。
    // 如果 Rust 允许 vector 存放任意类型，那么当对 vector 元素执行操作时一个或多个类型的值就有可能会造成错误。
    // 使用枚举外加 match 意味着 Rust 能在编译时就保证总是会处理所有可能的情况
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}