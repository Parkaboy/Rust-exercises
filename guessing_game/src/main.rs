use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // el programa elije un numero al azar

    loop {

        let mut guess = String::new(); // Las variables son inmutables por defecto (constantes), la palabra reservada mut las convierte en mutables (variables).

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Lee el input del usuario

        // convierte el texto ingresado por el usuario en un entero.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too big!"),
            Ordering::Greater => println!("Too small"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
