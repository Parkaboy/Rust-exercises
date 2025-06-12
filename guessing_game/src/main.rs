use std::io;

fn main() {

println!("Guess the number!");

println!("Please input your guess");

let mut guess = String::new(); // Las variables son inmutables por defecto (constantes), la palabra reservada mut las convierte en mutables (variables).

io::stdin().read_line(&mut guess).expect("Failed to read line");

println!("You guessed: {}", guess);


}
