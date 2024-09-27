struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/**
 * 矩形
 */
// 开启 debug
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 这是一个计算矩形面积的函数，但是它只能接收矩形结构体引用，
// 所以最好的方式是定义一个绑定结构体的方法
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

// 这是 Rectangle 结构体的方法，方法的首个参数都是 &self，这在oop
// 特性的语言中几乎一致。如果需要方法对对象数据进行修改，则需要改为
// &mut self
impl Rectangle {
    // 计算矩形面积
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 当前矩形是否可容纳另一个矩形
    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }
}

// 元组结构体，类似于为特定类型的元组命名以区分用法
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 还有一种比较特殊的结构体，其不包含任何字段，称为类单元结构体，
// 因为它类似于元组中的 unit 类型。类似于 go 中的空结构体
struct AlwaysEqual;

fn main() {
    // 如果想要让结构体可变，只要在声明的时候添加 mut 关键字，
    // 添加之后该结构体的所有字段均可变
    let user1 = User {
        active: true,
        username: String::from("somebody1"),
        email: String::from("someemail"),
        sign_in_count: 1,
    };

    // 如果我们想更新某结构体的几个字段来创建新结构体，我们可以使用如下语法，类似js和go
    // 但是此处将 user1 字段赋值给user2的操作相当于将user1这些字段的 ownership 交给了user2，
    // 所以在 user2 声明后，我们无法继续使用 user1 的email字段，而 active 和 sign_in_count
    // 字段的类型具有 Copy 属性，可以继续使用
    let user2 = User {
        username: String::from("somebody2"),
        ..user1
    };

    // 虽然两者的字段类型完全相同，且字段值也完全相同，却是两个不同的类型
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let scale = 2;
    let rect = Rectangle {
        width: dbg!(scale * 2),
        height: 5,
    };

    // println! 宏接收引用
    println!("rect is {rect:?}");

    // dbg! 宏接收所有权
    dbg!(&rect);

    let area = area(&rect);
    println!("area = {area}");

    // rust 在方法调用时有自动引用和解引用的功能，因为我们在定义方法的时候，
    // 方法的第一个参数就决定了需要对象的所有权、引用还是可变引用，所以：
    // rect.area() 等同于 &rect.area()
    println!("the area of a rectangle is {}", rect.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


}

// 如果参数名称和结构体字段名称相同，则可以省略
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}