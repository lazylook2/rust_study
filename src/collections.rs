use std::collections::HashMap;

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
pub fn String1() {
    let data = "initial contents"; // 字符串字面值
    let mut s = String::new(); // String 这样拥有所有权，被移动就回不来了
    let s = data.to_string(); // String
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('d');

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s = s1 + "-" + &s2; // String 和 &str。s1 被移动了，不能继续使用
    let s = format!("{}-{}", s2, s2);
    assert_eq!("tac-tac", &s);assert_eq!("tac-tac", s);

    // String 是一个 Vec<u8> 的封装。
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 字符串 slice
    assert_eq!("Зд", s);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    // 其他类型的 slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }
    let mut s = String::from("hello world");
    let word = first_word(&s); // word 的值为 5
    s.clear(); // 这清空了字符串，使其等于 ""
    // println!("word: {}", word); // 借用规则，当拥有某值的不可变引用时（这个值原来就是可变的），就不能再获取一个可变引用。

    let my_string = String::from("hello world");
    // first_word 中传入 `String` 的 slice
    let word = first_word(&my_string[..]);assert_eq!(word, "hello");
    let my_string_literal = "hello world";
    // first_word 中传入字符串字面值的 slice
    let word = first_word(&my_string_literal[..]);assert_eq!(word, "hello");
    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);assert_eq!(word, "hello");
    println!("my_string_literal: {}", my_string_literal); // &str还能调用了
    let a11 = String::from("dd");
    fn a1 (s: String) {
        println!("s: {}", s)
    }
    a1(a11);
    // println!("a11: {}", a11); // String不能调用了

}
pub fn map1 () {
    let mut scores1: HashMap<String, i32> = HashMap::new(); // 泛型可以省略
    scores1.insert(String::from("Blue"), 10);

    // HashMap的构建方式可以使用一个元组的 vector 的 collect 方法
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // HashMap<_, _> 类型注解是必要的
    // 因为可能 collect 很多不同的数据结构，而除非显式指定否则 Rust 无从得知你需要的类型。
    // 但是对于键和值的类型参数来说，可以使用下划线占位，而 Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型。

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效
    // 如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。
    // 但是这些引用指向的值必须至少在哈希 map 有效时也是有效的。涉及到生命周期

// 访问哈希 map 中的值
    let blue = &String::from("Blue");
    println!("{}", scores1.get(blue).unwrap());

    for (key, value) in &scores1 {
        println!("{}: {}", key, value);
    }

// 更新哈希 map
    // 覆盖一个值
    scores1.insert(String::from("Blue"), 25);
    println!("{:?}", scores1);
    // 只在键没有对应值时插入
    scores1.entry(String::from("Yellow")).or_insert(0);
    println!("{:?}", scores1);
    // 根据旧值更新一个值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）
        let count = map.entry(word).or_insert(0);
        println!("{}\tcount: {}", word, count);
        *count += 1;
    }
    println!("{:?}", map);
}