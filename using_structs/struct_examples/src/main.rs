#[derive(Debug)] //Printing in debug format works after this attribute
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 60,
        height: 40,
    };

    let res = calc_area(&rect);
    println!("{res}");

    println!("{rect:?}"); //This works because of :? and the debug attribute imported above
    println!("{rect:#?}"); //Even this works (better formatted)

    dbg!(&rect); //This takes ownership of rect unlike println!(""); which takes reference, we dont
                 //want this so we gave it &rect (reference to rect)
}

fn calc_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
