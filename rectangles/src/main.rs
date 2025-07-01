fn main() {

let width = 3;
let height = 5;


    println!("The area of the rectacngle is {} square pixels.", area(width, height));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
