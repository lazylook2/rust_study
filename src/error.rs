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
}