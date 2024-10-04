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

// 一个与枚举相得益彰的用法就是 match 控制流匹配
// 我们定义了一个硬币枚举类，并通过函数与 match 
// 返回硬币的实际面值
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
// 在 match 控制流中，匹配成功后可以进行一系列操作
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky coin!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// 我们也可以搭配 Option<t> 来使用 match
// 比如以下函数在为 None 时返回 None，在为
// Some(i) 时返回 Some(i+1)
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

// 还有一个关于 match 的重要特性就是，match 是穷尽的(exhaustive),在 match 控制流
// 中如果匹配的是枚举，则必须穷尽该枚举的所有可能，不然会通过不了编译
// 如果你非匹配枚举，则可在最后添加通配模式或是占位符_

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

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 如果需要关心通配模式的值，可以写为
        // other => do_something(other),
        // 或是无需任何操作，可以返回空元组
        // _ => (),
        _ => reroll(),
    }

    // 如果你只需匹配一种可能而无需关心其余情况，使用 match 会让你的代码量更多
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // 我们可以使用 if let 来代替这种情况
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // 如果 match 需处理通配情况，在 if let 控制流中还可以使用一个 else 处理其余情况
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}