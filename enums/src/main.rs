#![allow(unused)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("{:?}", self);
    }
} 
fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
    // rust 一大不同于其他语言的设计在于其对空值（null）的定义。
    // rust 中没有空值，如果需要处理空值的可能性，需要使用 Option
    // 枚举类。
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // 所以一下的 some_number 是 Option<i32> 类型的，当我们需要用到
    // 其中的值时，也可以用 unwrap 方法从中取出来
    let some_number = Some(5);
    println!("number: {}", some_number.unwrap());
}