pub fn em1() {
    enum IpAddKind {V4,V6,}
    let four = IpAddKind::V4;
    fn route (ip_type: IpAddKind) { }
    route(IpAddKind::V4); // 参数
    struct IpAddr {
        kind: IpAddKind,
        address: String,
    }
    let home = IpAddr{ kind: IpAddKind::V4, address: String::from("127.0.0.1") };

    enum Message {
        Quit,   // 没有关联任何数据
        Move {x: i32, y: i32},  // 包含一个匿名结构体
        Write(String),  // 包含单独一个 String
        ChangeColor(i32, i32, i32), // 包含三个 i32
    }
    impl Message {
        fn call(&self) {
            // 这里定义方法体
            match self {
                Message::Write(str) => println!("{}", str),
                _ => println!("匹配不到"),
            }
        }
    }
    let m = Message::Write("hehe".to_string());
    m.call();

    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    match some_number {
        Option::Some(i) => println!("{}", i),
        Option::None => println!("空值"),
    }
    // 方法在[https://doc.rust-lang.org/std/option/enum.Option.html]里找
    println!("{}", absent_number.unwrap_or(0));
}
pub fn em2 () {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1)
        }
    }
    let five = Some(5);
    println!("加一操作：{}", plus_one(five).unwrap());
    // _通配符
    let some_u8_value = 0u8; // 0
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let some_u8_value = Some(0u8);
    if let Some(1) = some_u8_value { // 左右没有区别
        println!("zero")
    } else {
        println!("呃")
    }

}