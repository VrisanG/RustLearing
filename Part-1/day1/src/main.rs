use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess your number: ");

    let secret = rand::thread_rng().gen_range(1..100);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = guess.trim().parse().expect("Please type a valid number!");

        match guess.cmp(&secret) {
            Ordering::Greater => println!("The guess is too big"),
            Ordering::Less => println!("The guess is too small"),
            Ordering::Equal => {
                println!("You won the game");
                break;
            }
        }
    }
}
