use std::fs::File;
use std::io::ErrorKind;

pub fn e1 () {
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
}