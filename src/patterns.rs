pub fn p1(){
    /*
    match 分支
    match VALUE {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
    }
    */

    // if let 条件表达式
    let color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = color {
        println!("color: {}", color)
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("大于30岁")
        } else {
            println!("小于30岁")
        }
    } else {
        println!("呃呃")
    }

    // while let 条件循环（它允许只要模式匹配就一直进行 while 循环）
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top)
    }

    // for 循环
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let 语句
    let (x, y, z) = (1, 2, 3); //(1, 2, 3)会匹配(x, y, z)，将1绑定到x，2绑定到y，3绑定到z
    println!("{}, {}, {}", x, y, z);

    let (x, .., z) = (1, 2, 3);
    println!("{}, {}", x, z);

}
/// 所有的模式语法
pub fn p2 () {
// 匹配字面值
    let x = 1;
    match x {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("其他")
    }

// 匹配命名变量
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("到50了"),
        // y 是在这个作用域内的新变量，匹配任何 Some 中的值，非Nome
        Some(y) => println!("匹配到了，y = {:?}", y),
        _ => println!("其他情况，x = {:?}", x)
    }
    // 一旦 match 表达式执行完毕，其作用域也就结束了，同理内部 y 的作用域也结束了。
    println!("最后: x = {:?}, y = {:?}", x, y);

// 多个模式
    let x = 1;

    match x {
        1 | 2 | 3 => println!("1 或 2 或 3"),
        4 => println!("4"),
        _ => println!("其他")
    }

// 通过 ..= 匹配值的范围（..= 语法允许你匹配一个闭区间范围内的值）
    let x = 5;

    match x {
        1 ..= 5 => println!("1 到 5"),
        _ => println!("其他")
    }

    let x = 'c';
    match x {
        'a' ..= 'j' => println!("前侧的字符"),
        'k' ..= 'z' => println!("后侧的字符"),
        _ => println!("其他")
    }

// 解构并分解值
// 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point{ x: 0, y: 7 };
    let Point{x: a, y: b} = p;
    // 简写
    let Point{ x, y} = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    assert_eq!(x, 0);

    match p {
        // 创建了x变量
        Point{ x, y: 0 } => println!("在x轴上的任何点 {}", x),
        // 创建了y变量
        Point{ x: 0, y } => println!("在y轴上的任何点 {}", y),
        Point{ x, y} => println!("不在轴上的点 ({}, {})", x, y)
    }

// 解构枚举
    enum Message {
        Quit,
        Move { x: i32, y: i32},
        Write (String),
        ChangeColor(i32, i32, i32)
    }
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => { println!("Quit") },
        Message::Move { x, y } => { println!("Move: {}\t{}", x, y) },
        Message::Write(x) => { println!("Write: {}", x) },
        Message::ChangeColor(r, g, b) => { println!("ChangeColor: {}\t{}\t{}", r,g,b) }
    }
// 解构嵌套的结构体和枚举
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message1 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message1::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message1::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("ChangeColor -> Color : {}\t{}\t{}", r, g, b);
        }
        Message1::ChangeColor(Color::Hsv(r, g, b)) => {
            println!("ChangeColor -> Color : {}\t{}\t{}", r, g, b);
        }
        _ => ()
    }
// 解构结构体和元组
    let ((feet, inches), Point{ x, y }) = ((3, 10), Point{ x: 3, y: -10 });
    assert_eq!(feet, 3);
    assert_eq!(inches, 10);
    assert_eq!(x, 3);
    assert_eq!(y, -10);
// 忽略模式中的值
// 使用 _ 忽略整个值
    fn foo(_: i32, y: i32){
        println!("只有 y 有值：{}", y)
    }
    foo(3, 4);

// 使用嵌套的 _ 忽略部分值
    let mut setting_value = Some(5); // None
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => { println!("匹配到了任何Some的值") },
        // 如果有一个是None，那就执行这一条
        _ => setting_value = new_setting_value
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

// 通过在名字前以一个下划线开头来忽略未使用的变量
    let _x = 5;
    let y = 10;

    let age: Result<u8, _> = "34".parse();
    if let Ok(age1) = age {
        if age1 > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    }
    println!("age: {:?}", age);

    let s = Some(String::from("Hello"));
    // if let Some(_s) = s {
    //     println!("正常匹配") // 因为 s 的值仍然会移动进 _s, 报错
    // }
    // println!("{:?}", s);

    if let Some(_) = s {
        // 单独使用下划线不会绑定值 它没有被移动 会进入
        println!("found a string");
    }
    println!("{:?}", s);

// 用 .. 忽略剩余值
    let origin = Point{ x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        // 不能 (.., second, ..)
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }

// 匹配守卫提供的额外条件
// 匹配守卫（match guard）是一个指定于 match 分支模式之后的额外 if 条件
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("小于5：{}", x),
        Some(x) => println!("完全匹配的Some {}", x),
        None => ()
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // 这个 y 正是 外部的 y (外部变量)
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;
    match x {
        // 这个匹配条件表明此分支值匹配 x 值为 4、5 或 6 同时 y 为 true 的情况。
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

// @ 绑定
// at 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式。
    enum Message2 {
        Hello {id : i32}
    }
    let msg = Message2::Hello { id: 5 };
    match msg {
        // 通过在 3...7 之前指定 id_variable @，我们捕获了任何匹配此范围的值并同时测试其值匹配这个范围模式。
        Message2::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        // 指定范围
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message2::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}