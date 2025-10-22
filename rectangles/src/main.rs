struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let my_rectangle = Rectangle {
        width: 3,
        height: 5,
    };

    println!(
        "The area of the rectacngle is {} square pixels.",
        my_rectangle.area()
    );
}
