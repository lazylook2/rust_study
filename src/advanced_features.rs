use std::slice;

/// 解引用裸指针<br>
/// 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针<br>
/// 不保证指向有效的内存<br>
/// 允许为空<br>
/// 不能实现任何自动清理功能<br>
pub fn unsafe_rust1() {
    let mut num = 5;
    let r1 = &num as *const i32;    // 不可变裸指针
    let r2 = &mut num as *mut i32;      // 可变裸指针

    // 创建了同时指向相同内存位置 num 的裸指针 *const i32 和 *mut i32。
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2)
    }
}
/// 调用不安全函数或方法<br>
/// 开头有一个额外的 unsafe<br>
/// 必须在一个单独的 unsafe 块中调用 dangerous 函数。
pub fn unsafe_rust2() {
    unsafe fn dangerous() { println!("不安全的方法") }
    unsafe {
        dangerous();
    }
}
/// 创建不安全代码的安全抽象
pub fn unsafe_rust3() {
    pub fn split_at_mut1(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        /*let len = slice.len();
        assert!(mid <= len);
        (&mut slice[..mid], &mut slice[mid..])*/
        let len = slice.len();
        let ptr = slice.as_mut_ptr(); // 使用 as_mut_ptr 方法访问 slice 的裸指针
        assert!(mid <= len);
        unsafe {
            (
                // slice::from_raw_parts_mut 函数获取一个裸指针和一个长度来创建一个 slice。
                slice::from_raw_parts_mut(ptr, mid),
                // 之后在 ptr 上调用 offset 方法并使用 mid 作为参数来获取一个从 mid 开始的裸指针，
                // 使用这个裸指针并以 mid 之后项的数量为长度创建一个 slice。
                slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
            )
        }


    }
    let address = 0x01234usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };
}
/// 使用 extern 函数调用外部代码<br>
/// extern，有助于创建和使用 外部函数接口（Foreign Function Interface， FFI）
pub fn unsafe_rust4() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("使用C语言的函数计算绝对值：{}", abs(-3))
    }
}
/// 访问或修改可变静态变量
pub fn unsafe_rust5() {

}
/// 实现不安全 trait

