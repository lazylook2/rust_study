pub fn s1 () {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    let user1 = User{
        username: "haha".to_string(),
        email: String::from("hihi"),
        sign_in_count: 0,
        active: false
    };
    let mut user1 = User{
        username: "haha".to_string(),
        email: String::from("hihi"),
        sign_in_count: 4,
        active: true
    };
    user1.username = String::from("hoho");
    println!("username: {}", user1.username);

    fn build_user(username: String) -> User{
        User{
            username, // 简写语法
            email: "dd".to_string(),
            sign_in_count: 0,
            active: false
        }
    }
    // 结构体更新语法
    let user2 = User{
        username: String::from("ff"),
        email: user1.email,
        sign_in_count: 0,
        active: false
    };
    let user2 = User{
        username: "user2".to_string(),
        email: "user2".to_string(),
        ..user1
    };
    println!("user2.sign_in_count: {}", user2.sign_in_count);
    // 元组结构体
    struct Color(i32, i32, i32);
    let black = Color(1, 2, 3);
    println!("{}", black.2);
}
pub fn s2() {
    #[derive(Debug)]
    struct Rectangle{
        width: u32,
        height: u32,
    }
    let rect1 = Rectangle{ width: 35, height: 30 };
    fn area (rectangle: &Rectangle) -> u32{
        rectangle.width * rectangle.height
    }
    println!("面积为：{}", area(&rect1));
    println!("{:#?}", rect1);
    impl Rectangle {
        fn area(&self) -> u32{ // 方法rect1.area和Rectangle::area()调用
            self.height * self.width
        }
        fn can_hold(&self, other: &Rectangle) -> bool{
            self.width > other.width && self.height > other.height
        }
        fn square(size: u32) -> Rectangle {
            Rectangle { width: size, height: size }
        }

    }
    println!("面积为：{}\t{}", Rectangle::area(&rect1), rect1.area());
    println!("{:#?}", Rectangle::square(23));

}
