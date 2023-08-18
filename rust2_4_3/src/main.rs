
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, world!");
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user3 = User {
        email: String::from("another2@example.com"),
        ..user1
    };
    println!("{}", user1.active);
    println!("{:?}", user3);
    dbg!(user3);


    // 元组结构体 tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 单元结构体 unit struct
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    // impl SomeTrait for AlwaysEqual {
    //
    // }
}
