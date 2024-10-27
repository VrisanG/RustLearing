use std::io::{self, };
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("This is a Guessing Game");

    println!("You need to Guess the Number in Command Line");
    
    let mut user_guess= String::new();
    
    let secret_number=rand::thread_rng().gen_range(1..50);
    
    
    io::stdin().read_line(&mut user_guess).expect("Failed to read your rexponse please Re run the game");
    
    let user_guess   = user_guess.trim().parse::<u32>().expect("The given response is not a number");

    loop{
        match user_guess.cmp(&secret_number){
            Ordering::Equal=>{
                println!("You Won lol");
                break;
            }
            Ordering::Less=>{
                println!("Nah Homie Your guessing way too less");
                break

            }
            Ordering::Greater=>{
                println!("Nah homie your guessing way too much ");
                break
            }
        }
    }
    
    



}