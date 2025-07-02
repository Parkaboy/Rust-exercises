struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectacngle = Rectangle {
        width: 3,
        height: 5,
    };

    println!(
        "The area of the rectacngle is {} square pixels.",
        area(&rectacngle)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
