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


}