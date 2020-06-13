use std::fs::File;
use std::io::{ErrorKind, Read, Error};
use std::{io, fs};

/*pub fn e1 () {
// 使用result处理潜在的错误
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
    let f = File::open("fuck.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound  => match File::create("fuck.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_kind=> panic!("Problem opening the file: {:?}", other_error),
        }
    };
    // 用了闭包
    let f = File::open("fuck.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("fuck.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
// 失败时 panic 的简写：unwrap 和 expect

// unwrap 那样使用默认的 panic! 信息
// expect 用来调用 panic! 的错误信息将会作为参数传递给 expect

    // 如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。
    // 如果 Result 是成员 Err，unwrap 会为我们调用 panic!。
    let f = File::open("hello.txt").unwrap();


    // 使用 expect 提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源。
    let f = File::open("hello.txt").expect("无法打开。。。。 hello.txt");
}*/
pub fn e2 () {
// 传播错误（方法里编写，直接抛出错误）
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("fuck.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    let f = read_username_from_file();

    match f {
        Ok(string) => println!("fuck.txt的内容为：{}", string),
        Err(e) => panic!("读取fuck.txt的内容失败: {:?}", e),
    }

    /// Result 值之后的 ? 与match 表达式有着完全相同的工作方式。
    /// 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
    /// 如果值是 Err，Err 中的值将作为整个函数的返回值，就好像使用了 return 关键字一样
    fn read_username_from_file1() -> Result<String, io::Error> {
        /*let mut f = File::open("fuck.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;*/
        // 可以链式调用，下面可以取代注释内容

        /*let mut s = String::new();
        File::open("fuck.txt")?.read_to_string(&mut s)?;*/
        // 更简便的写法如下：
        fs::read_to_string("fuck.txt")

    }
    let f = read_username_from_file();

    match f {
        Ok(string) => println!("fuck.txt的内容为：{}", string),
        Err(e) => panic!("读取fuck.txt的内容失败: {:?}", e),
    }

    let secret_number = 3;
    loop {
        let mut guess = String::new();
        // --snip--
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }
        match guess.cmp(&secret_number) {
            _ => (),
        }
    }
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess {
                value
            }
        }
        pub fn value(&self) -> i32 {
            self.value
        }
    }
}