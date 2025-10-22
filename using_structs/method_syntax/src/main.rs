struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //These below are called methods since they have &self

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //This is not a method since it doesnt consist of &self as an argument, its just an associative
    //function

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 60,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("{}", rect1.area());
    println!("{}", rect1.width());

    println!("{}", rect1.can_hold(&rect2));

    //This is how associative functions are generally initialized
    let sq = Rectangle::square(3);
}
