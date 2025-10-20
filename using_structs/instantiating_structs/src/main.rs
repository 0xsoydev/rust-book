struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//This wont work as there's no lifetime specifier
//struct RefUser {
//    active: bool,
//    username: &str,
//    email: &str,
//    sign_in_count: u64,
//}

//Tuple-type structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit-like structs
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("zer0day"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //Alternative way to do it
    //let user3 = User {
    //    email: String::from("another@example.com"),
    //    ..user1
    //};

    user1.email = String::from("anotheremail@example.com");
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
