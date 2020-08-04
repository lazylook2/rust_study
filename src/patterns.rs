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
        Some(y) => println!("匹配到了，y = {:?}", y),
        _ => println!("其他情况，x = {:?}", x)
    }
    println!("最后: x = {:?}, y = {:?}", x, y);
}