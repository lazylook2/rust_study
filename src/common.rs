pub fn c1 () {
    let _i = 32;
    let _i = 3.0;
    let _i = true;
    let _i = 'z';
    let i: (i32, f64, u8) = (500, 6.4, 1); // 元组
    let (_i1, _i2, _i3) = i;
    let _i1 = i.0;
    let _i = [1, 2, 3]; // 数组
    let _i: [i32; 5] = [1,2,3,4,5];
    let _i = [3; 5]; // 长度为5，元素都是3
    assert_eq!(3, _i[0]);
    let y = {
        let x = 3;
        x + 1
    };
    //*************    if    *************//
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    //*************    循环    *************//
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}