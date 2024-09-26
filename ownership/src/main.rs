fn main() {
    // 为了了解 ownership，我们先创建一个 String
    // 在 rust 中，字符串字面值是不可变的。而 String 是可变的。
    // 字面值在编译期便硬编码至可执行文件，而String 需在运行时在
    // 堆上分配内存。
    // GC语言会帮程序员回收内存，但是会有一定性能损耗；无GC语言需要
    // 手动分配和释放内存，这又对程序员要求甚高。而对于 rust 来说，
    // 变量在离开作用域后会自动释放。rust 会在作用域结尾调用 drop 函数。
    let mut s = String::from("hello"); // 从此处开始 s 生效
    s.push_str(", world!");
    println!("{}", s);

    // 以下的赋值操作是不涉及内存拷贝的，因为 String 在堆上分配，
    // 赋值操作会将 s1 的指针指向堆上的同一块内存。
    let s1 = String::from("hello");
    let s2 = s1;
    // 此时，堆上的 String 的所有权从 s1 move 到 s2
    // 这段代码无法通过编译 println!("{}", s1);
    // rust 这样设计的好处是防止拷贝以及二次释放内存
    println!("{}", s2);
    // clone 会深度复制堆上的数据，因此需要考虑性能因素
    let s3 = s2.clone();
    println!("{}", s3);

    // 这段操作可能看起来与之前所描述的相矛盾，其实不然。
    // x 作为一个编译期可确定的值，其存储在栈上，所以将 x
    // 赋值给 y 时进行的是一个栈上的拷贝，这相对于堆上的拷贝
    // 更加轻量级。并且所有整数类型，布尔类型，浮点类型，
    // 字符类型，以及包括以上的元组的组合类型，都是 Copy 的。
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // 接下来演示嵌套作用域，如函数调用时的例子
    let s = String::from("hello");

    takes_ownership(s); // s 的所有权被移动到函数中
    // s 的值在函数调用结束后失效

    let mut x = 5;
    makes_copy(x); // 因为 i32 是 Copy 的，所以 x 的值不会失效

    // 大部分情况，我们在调用函数过后任然需要使用变量,
    // 为了避免在函数中再返回传入参数，我们在调用函数时
    // 可以使用引用来传递参数。这相当于创建了一个临时变量
    // s 来引用 s1，s-->s1-->"hello"，当 s 被 drop后，
    // s1 任然存活在其作用域内。我们称为借用（borrowing）。
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 如果函数想对传入引用的实际值操作时，我们必须传入一个可变
    // 变量的引用
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // rust 也不允许对可变变量持有多份引用，以下代码无法通过编译:
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
    // 如果引用借给了 r1 后又借给了 r2,那么在 r2 之后就无法继续使用 r1,
    // 这可以在编译期保证不存在数据竞争
    // 正确的做法是在 r2 后不使用 r1或是在一个新作用域内使用 r1

    // rust 可不允许在持有可变变量的不可变应用时声明该变量的可变引用:
    // let mut s = String::from("hello");

    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;

    // println!("{}, {}, and {}", r1, r2, r3);
    // 不可变引用的变量可不希望变量值被修改了
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);

    // 在 rust 中还容易出现悬垂引用(dangling reference)，即返回引用
    // 的实际值已经被释放，如：
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    
    //     &s
    // }
    // 变量 s 在该函数作用域结束后即被释放，所以 &s 指向了被释放了的空间
    // 正确做法是直接返回 s,将 s的所有权移到函数调用方
    let m = no_dangle();
    println!("{}", m);


    // 假设一个例子，你想在一个字符串中寻找第一个为空格的索引来确定第一个单词。
    // 假如你编写的函数只返回索引的数值，那么在这个字符串修改过后，这个索引就
    // 失去了作用，它可能指向一个新的字节。所以，我们定义的函数应该返回该字符串
    // 的一个切片引用，来让编译器在编译期为程序员检查这个切片引用在使用时，其原
    // 字符串是否收到了修改。以下是一段无法通过编译的示例：
    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // s.clear(); // 错误！

    // println!("the first word is: {}", word);

    // 因为字符串字面值是直接存储在二进制文件中的，所以其类型 &str 是一个不可变
    // 引用。为了提高可兼容性，我们可以将如下取第一个单词的函数签名改为：
    // fn first_word(s: &str) -> &str {
    // 因为 String 类型也可以通过切片引用的形式传入

    
} // 从此处开始 s 失效

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}