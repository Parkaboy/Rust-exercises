fn main() {

let rectacngle = (3, 5);


    println!("The area of the rectacngle is {} square pixels.", area(width, height));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
