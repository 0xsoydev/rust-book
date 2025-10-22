enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    //Instances of enum variants V4 nad V6
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let absent_num: Option<i32> = None;
    let some_char = Some('e');
    let some_num = Some(5);
}
